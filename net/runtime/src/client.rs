use std::{
    ops::Deref,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use bytes::Bytes;
use just_webrtc::{
    DataChannelExt, PeerConnectionBuilder, PeerConnectionExt,
    types::{DataChannelOptions, PeerConnectionState},
};
use pawkit_net_signaling::{
    client::ClientPeerSignalingClient,
    model::{ChannelConfiguration, HostId},
};
use tokio::sync::{
    RwLock,
    mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel},
};

use crate::{Connection, RUNTIME, receive_packets};

pub struct NetClientPeer {
    connection: RwLock<Option<Connection>>,
    ev_dispatcher: UnboundedSender<NetClientPeerEvent>,
    running: AtomicBool,
    host_id: HostId,
    game_id: u32,
}

#[derive(Debug)]
pub enum NetClientPeerEvent {
    Connected,
    Disconnected,
    ConnectionFailed,
    PacketReceived { channel: usize, data: Vec<u8> },
}

impl NetClientPeer {
    pub fn create(
        game_id: u32,
        host_id: HostId,
    ) -> (Arc<Self>, UnboundedReceiver<NetClientPeerEvent>) {
        let (ev_dispatcher, ev_queue) = unbounded_channel::<NetClientPeerEvent>();

        let peer = Arc::new(Self {
            connection: RwLock::new(None),
            ev_dispatcher,
            running: AtomicBool::new(true),
            host_id,
            game_id,
        });

        peer.clone().spawn_worker();

        (peer, ev_queue)
    }

    pub fn send_packet(&self, channel: usize, data: &[u8]) {
        let conn = self.connection.blocking_read();

        if let Some(conn) = &*conn {
            let _ = RUNTIME.block_on(conn.channels[channel].send(&Bytes::copy_from_slice(data)));
        }
    }

    fn channel_config_to_option(config: &ChannelConfiguration) -> DataChannelOptions {
        return DataChannelOptions {
            ordered: Some(config.ordered),
            max_retransmits: config.reliability,

            ..Default::default()
        };
    }

    async fn connect_to_host(&self) -> Option<Connection> {
        let mut signaling =
            ClientPeerSignalingClient::new(&self.host_id.server_url, self.game_id).await?;

        let configurations = signaling
            .channel_configurations(self.host_id.clone())
            .await?;

        let channel_options = configurations
            .iter()
            .map(|it| ("pawkit_".to_string(), Self::channel_config_to_option(it)))
            .collect::<Vec<_>>();

        let channels = channel_options.len();

        let connection = PeerConnectionBuilder::new()
            .with_channel_options(channel_options)
            .unwrap()
            .build()
            .await
            .ok()?;

        let offer = connection.get_local_description().await?;
        let candidates = connection.collect_ice_candidates().await.ok()?;

        let candidate = signaling
            .offer_connection(self.host_id.clone(), offer, candidates)
            .await?;

        connection
            .set_remote_description(candidate.offer)
            .await
            .ok()?;
        let _ = connection.add_ice_candidates(candidate.candidates).await;

        if let PeerConnectionState::Connected = connection.state_change().await {
            return Connection::from(connection, channels).await.ok();
        }

        None
    }

    async fn worker_loop(self: Arc<Self>) {
        let Some(conn) = self.connect_to_host().await else {
            let _ = self
                .ev_dispatcher
                .send(NetClientPeerEvent::ConnectionFailed);
            return;
        };

        {
            let mut lock = self.connection.write().await;
            *lock = Some(conn);
        }

        let _ = self.ev_dispatcher.send(NetClientPeerEvent::Connected);

        while self.running.load(Ordering::Relaxed) {
            let connection = self.connection.read().await;
            let Some(connection) = &*connection else {
                break;
            };

            tokio::select! {
                Some((channel, data)) = receive_packets(&connection.channels) => {
                    let _ = self
                        .ev_dispatcher
                        .send(NetClientPeerEvent::PacketReceived { channel, data });
                }

                PeerConnectionState::Disconnected = connection.raw_connection.state_change() => {
                    break;
                }

                else => break
            }
        }

        let _ = self.ev_dispatcher.send(NetClientPeerEvent::Disconnected);

        self.running.store(false, Ordering::Relaxed);
    }

    pub fn disconnect(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    fn spawn_worker(self: Arc<Self>) {
        tokio::spawn(async move {
            self.worker_loop().await;
        });
    }
}

pub struct SimpleNetClientPeer {
    raw_peer: Arc<NetClientPeer>,
    ev_queue: UnboundedReceiver<NetClientPeerEvent>,
}

impl SimpleNetClientPeer {
    pub fn create(game_id: u32, host_id: HostId) -> Self {
        let (raw_peer, ev_queue) = NetClientPeer::create(game_id, host_id);
        Self { raw_peer, ev_queue }
    }

    pub fn next_event(&mut self) -> Option<NetClientPeerEvent> {
        return self.ev_queue.try_recv().ok();
    }
}

impl Drop for SimpleNetClientPeer {
    fn drop(&mut self) {
        self.disconnect();
    }
}

impl Deref for SimpleNetClientPeer {
    type Target = Arc<NetClientPeer>;

    fn deref(&self) -> &Self::Target {
        &self.raw_peer
    }
}
