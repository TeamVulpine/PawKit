use std::{
    ops::Deref,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use bytes::Bytes;
use futures_util::{StreamExt, stream::FuturesUnordered};
use just_webrtc::{
    DataChannelExt, PeerConnectionBuilder, PeerConnectionExt, types::PeerConnectionState,
};
use pawkit_holy_array::HolyArray;
use pawkit_net_signaling::{
    client::{ClientConnectionCandidate, HostPeerSignalingClient},
    model::{ChannelConfiguration, HostId},
};
use tokio::sync::{
    RwLock,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};

use crate::{Connection, PacketFuture, RUNTIME, receive_packet};

pub struct NetHostPeer {
    connected_clients: RwLock<HolyArray<Arc<Connection>>>,
    ev_dispatcher: UnboundedSender<NetHostPeerEvent>,
    running: AtomicBool,
    game_id: u32,
    host_id: RwLock<HostId>,
    request_proxy: bool,
}

#[derive(Debug)]
pub enum NetHostPeerEvent {
    PeerConnected {
        peer_id: usize,
    },
    PeerDisconnected {
        peer_id: usize,
    },
    PacketReceived {
        peer_id: usize,
        channel: usize,
        data: Vec<u8>,
    },
    HostIdUpdated,
}

impl NetHostPeer {
    pub fn create(
        server_url: &str,
        game_id: u32,
        request_proxy: bool,
        channel_configurations: Vec<ChannelConfiguration>,
    ) -> (Arc<Self>, UnboundedReceiver<NetHostPeerEvent>) {
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
            request_proxy,
        });

        value.clone().spawn_worker(channel_configurations);

        return (value, ev_queue);
    }

    pub fn get_host_id(&self) -> HostId {
        return self.host_id.blocking_read().clone();
    }

    pub fn send_packet(&self, client_id: usize, channel: usize, data: &[u8]) {
        let clients = self.connected_clients.blocking_read();

        let Some(client) = clients.get(client_id) else {
            return;
        };

        let _ = RUNTIME.block_on(client.channels[channel].send(&Bytes::copy_from_slice(data)));
    }

    async fn handle_candidate(
        &self,
        signaling: &mut HostPeerSignalingClient,
        candidate: ClientConnectionCandidate,
        channels: usize,
    ) -> Option<usize> {
        let Ok(connection) = PeerConnectionBuilder::new()
            .with_remote_offer(Some(candidate.offer.clone()))
            .unwrap()
            .build()
            .await
        else {
            signaling.reject_candidate(candidate.client_id).await;
            return None;
        };

        let _ = connection
            .add_ice_candidates(candidate.candidates.clone())
            .await;

        let Some(offer) = connection.get_local_description().await else {
            signaling.reject_candidate(candidate.client_id).await;
            return None;
        };

        let Ok(candidates) = connection.collect_ice_candidates().await else {
            signaling.reject_candidate(candidate.client_id).await;
            return None;
        };

        signaling
            .accept_candidate(candidate.client_id, offer, candidates)
            .await;

        let mut connected_clients = self.connected_clients.write().await;

        let PeerConnectionState::Connected = connection.state_change().await else {
            return None;
        };

        let connection = Connection::from(connection, channels).await.ok()?;

        let peer_id = connected_clients.acquire(Arc::new(connection));

        let _ = self
            .ev_dispatcher
            .send(NetHostPeerEvent::PeerConnected { peer_id });

        return Some(peer_id);
    }

    async fn refresh_signaling(
        &self,
        signaling: &mut HostPeerSignalingClient,
        channel_configurations: &Vec<ChannelConfiguration>,
    ) {
        if !signaling.is_open() {
            {
                let Some(new_signaling) = HostPeerSignalingClient::new(
                    &self.host_id.read().await.server_url,
                    self.game_id,
                    self.request_proxy,
                    channel_configurations.clone(),
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
            let _ = self.ev_dispatcher.send(NetHostPeerEvent::HostIdUpdated);
        }
    }

    async fn packet_task(
        peer: Arc<Connection>,
        peer_id: usize,
        channel: usize,
    ) -> (Option<(usize, Vec<u8>)>, usize) {
        tokio::select! {
            Some(packet) = receive_packet(&peer.channels[channel]) => {
                return (Some((channel, packet)), peer_id)
            }

            PeerConnectionState::Disconnected = peer.raw_connection.state_change() => {
                return (None, peer_id)
            }
        }
    }

    async fn add_packet_task(
        &self,
        tasks: &FuturesUnordered<Pin<Box<PacketFuture>>>,
        peer_id: usize,
        channel: usize,
    ) {
        let clients = self.connected_clients.read().await;
        let Some(peer) = clients.get(peer_id) else {
            tasks.push(Box::pin(async move { (None, peer_id) }));
            return;
        };

        tasks.push(Box::pin(Self::packet_task(peer.clone(), peer_id, channel)));
    }

    async fn worker_loop(&self, channel_configurations: Vec<ChannelConfiguration>) {
        let mut signaling = {
            loop {
                let Some(host) = HostPeerSignalingClient::new(
                    &self.host_id.read().await.server_url,
                    self.game_id,
                    self.request_proxy,
                    channel_configurations.clone(),
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
        let _ = self.ev_dispatcher.send(NetHostPeerEvent::HostIdUpdated);

        let mut tasks = FuturesUnordered::<Pin<Box<PacketFuture>>>::new();

        while self.running.load(Ordering::Relaxed) {
            self.refresh_signaling(&mut signaling, &channel_configurations)
                .await;

            tokio::select! {
                Some(candidate) = signaling.next_candidate() => {
                    let Some(peer_id) = self.handle_candidate(&mut signaling, candidate, channel_configurations.len()).await else {
                        continue;
                    };

                    for channel in 0..channel_configurations.len() {
                        self.add_packet_task(&tasks, peer_id, channel).await;
                    }
                }

                Some((packet, peer_id)) = tasks.next() => {
                    let Some((channel, data)) = packet else {
                        let mut connected_clients = self.connected_clients.write().await;
                        connected_clients.release(peer_id);

                        let _ = self
                            .ev_dispatcher
                            .send(NetHostPeerEvent::PeerDisconnected { peer_id });

                        continue;
                    };

                    let _ = self
                        .ev_dispatcher
                        .send(NetHostPeerEvent::PacketReceived { peer_id, channel, data });

                    self.add_packet_task(&tasks, peer_id, channel).await;
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

    fn spawn_worker(self: Arc<Self>, channel_configurations: Vec<ChannelConfiguration>) {
        tokio::spawn(async move {
            self.worker_loop(channel_configurations).await;
        });
    }
}

pub struct SimpleNetHostPeer {
    raw_peer: Arc<NetHostPeer>,
    ev_queue: UnboundedReceiver<NetHostPeerEvent>,
}

impl SimpleNetHostPeer {
    pub fn create(
        server_url: &str,
        game_id: u32,
        request_proxy: bool,
        channel_configurations: Vec<ChannelConfiguration>,
    ) -> Self {
        let (raw_peer, ev_queue) =
            NetHostPeer::create(server_url, game_id, request_proxy, channel_configurations);

        return Self { raw_peer, ev_queue };
    }

    pub fn next_event(&mut self) -> Option<NetHostPeerEvent> {
        return self.ev_queue.try_recv().ok();
    }
}

impl Drop for SimpleNetHostPeer {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Deref for SimpleNetHostPeer {
    type Target = Arc<NetHostPeer>;

    fn deref(&self) -> &Self::Target {
        return &self.raw_peer;
    }
}
