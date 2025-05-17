use std::{
    future::Future,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, LazyLock,
    },
};

use futures_util::{stream::FuturesUnordered, StreamExt};
use just_webrtc::{
    platform::Channel, types::PeerConnectionState, DataChannelExt, PeerConnectionExt,
    SimpleRemotePeerConnection,
};
use pawkit_holy_array::HolyArray;
use pawkit_net_signaling::{
    client::{ClientConnectionCandidate, HostPeerSignalingClient},
    model::HostId,
};
use tokio::{
    runtime::Runtime,
    sync::{
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
        RwLock,
    },
};

pub struct NetHostPeer {
    connected_clients: RwLock<HolyArray<Arc<NetHostConnection>>>,
    ev_dispatcher: UnboundedSender<NetHostPeerEvent>,
    running: AtomicBool,
    game_id: u32,
    host_id: RwLock<HostId>,
}

struct NetHostConnection {
    pub channel: Channel,
}

#[derive(Debug)]
pub enum NetHostPeerEvent {
    PeerConnected { peer_id: usize },
    PeerDisconnected { peer_id: usize },
    PacketRecieved { peer_id: usize, data: Vec<u8> },
    HostIdUpdated { host_id: HostId },
}

#[cfg(not(target_arch = "wasm32"))]
type PacketFuture = dyn Future<Output = (Option<Vec<u8>>, usize)> + Send + Sync;
#[cfg(target_arch = "wasm32")]
type PacketFuture = dyn Future<Output = (Option<Vec<u8>>, usize)>;

static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| Runtime::new().unwrap());

impl NetHostPeer {
    pub fn create(
        server_url: &str,
        game_id: u32,
    ) -> Option<(Arc<Self>, UnboundedReceiver<NetHostPeerEvent>)> {
        let (ev_dispatcher, ev_queue) = unbounded_channel::<NetHostPeerEvent>();

        let value = Arc::new(Self {
            connected_clients: RwLock::new(HolyArray::new()),
            ev_dispatcher,
            running: AtomicBool::new(true),
            game_id,
            host_id: RwLock::new(HostId {
                server_url: server_url.into(),
                lobby_id: 0,
                shard_id: 0,
            }),
        });

        value.clone().spawn_worker();

        return Some((value, ev_queue));
    }

    pub fn get_host_id(&self) -> HostId {
        return self.host_id.blocking_read().clone();
    }

    async fn handle_candidate(
        &self,
        signaling: &mut HostPeerSignalingClient,
        candidate: ClientConnectionCandidate,
    ) -> Option<usize> {
        let Ok(conn) = SimpleRemotePeerConnection::build(candidate.offer.clone()).await else {
            signaling.reject_candidate(candidate.client_id).await;
            return None;
        };

        let _ = conn.add_ice_candidates(candidate.candidates.clone()).await;

        let Some(offer) = conn.get_local_description().await else {
            signaling.reject_candidate(candidate.client_id).await;
            return None;
        };

        let Ok(candidates) = conn.collect_ice_candidates().await else {
            signaling.reject_candidate(candidate.client_id).await;
            return None;
        };

        signaling
            .accept_candidate(candidate.client_id, offer, candidates)
            .await;

        let mut connected_clients = self.connected_clients.write().await;

        let PeerConnectionState::Connected = conn.state_change().await else {
            return None;
        };

        let Ok(channel) = conn.receive_channel().await else {
            return None;
        };

        let peer_id = connected_clients.acquire(Arc::new(NetHostConnection { channel }));

        let _ = self
            .ev_dispatcher
            .send(NetHostPeerEvent::PeerConnected { peer_id });

        return Some(peer_id);
    }

    async fn refresh_signaling(&self, signaling: &mut HostPeerSignalingClient) {
        if !signaling.is_open() {
            {
                let Some(new_signaling) = HostPeerSignalingClient::new(
                    &self.host_id.read().await.server_url,
                    self.game_id,
                )
                .await
                else {
                    return;
                };
                *signaling = new_signaling;
            }
            {
                *self.host_id.write().await = signaling.host_id.clone();
            }
            let _ = self.ev_dispatcher.send(NetHostPeerEvent::HostIdUpdated {
                host_id: signaling.host_id.clone(),
            });
        }
    }

    async fn add_packet_task(
        &self,
        tasks: &FuturesUnordered<Pin<Box<PacketFuture>>>,
        peer_id: usize,
    ) {
        let clients = self.connected_clients.read().await;
        let Some(peer) = clients.get(peer_id) else {
            tasks.push(Box::pin(async move { (None, peer_id) }));
            return;
        };

        let peer = peer.clone();
        tasks.push(Box::pin(async move { (peer.next_packet().await, peer_id) }));
    }

    async fn worker_loop(&self) {
        let mut signaling = {
            loop {
                let Some(host) = HostPeerSignalingClient::new(
                    &self.host_id.read().await.server_url,
                    self.game_id,
                )
                .await
                else {
                    continue;
                };

                break host;
            }
        };
        {
            *self.host_id.write().await = signaling.host_id.clone();
        }
        let _ = self.ev_dispatcher.send(NetHostPeerEvent::HostIdUpdated {
            host_id: signaling.host_id.clone(),
        });

        let mut tasks = FuturesUnordered::<Pin<Box<PacketFuture>>>::new();

        while self.running.load(Ordering::Relaxed) {
            self.refresh_signaling(&mut signaling).await;

            tokio::select! {
                Some(candidate) = signaling.next_candidate() => {
                    let Some(peer_id) = self.handle_candidate(&mut signaling, candidate).await else {
                        continue;
                    };

                    self.add_packet_task(&tasks, peer_id).await;
                }

                Some((packet, peer_id)) = tasks.next() => {
                    let Some(packet) = packet else {
                        let mut connected_clients = self.connected_clients.write().await;
                        connected_clients.release(peer_id);

                        let _ = self
                            .ev_dispatcher
                            .send(NetHostPeerEvent::PeerDisconnected { peer_id });

                        continue;
                    };

                    let _ = self
                        .ev_dispatcher
                        .send(NetHostPeerEvent::PacketRecieved { peer_id, data: packet });

                    self.add_packet_task(&tasks, peer_id).await;
                }

                else => {
                    continue;
                }
            }
        }
    }

    pub fn shutdown(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn spawn_worker(self: Arc<Self>) {
        RUNTIME.spawn(async move {
            self.worker_loop().await;
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn_worker(self: Arc<Self>) {
        wasm_bindgen_futures::spawn_local(async move {
            self.worker_loop().await;
        });
    }
}

impl NetHostConnection {
    async fn next_packet(&self) -> Option<Vec<u8>> {
        let Ok(data) = self.channel.receive().await else {
            return None;
        };

        return Some(data.to_vec());
    }
}
