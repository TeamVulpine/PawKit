use std::{
    ops::Deref,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use bytes::Bytes;
use just_webrtc::{
    platform::Channel, types::PeerConnectionState, DataChannelExt, PeerConnectionExt,
    SimpleLocalPeerConnection,
};
use pawkit_net_signaling::{client::ClientPeerSignalingClient, model::HostId};
use tokio::sync::{
    mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    RwLock,
};

use crate::{recieve_packet, RUNTIME};

pub struct NetClientPeer {
    channel: RwLock<Option<Channel>>,
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
    PacketReceived { data: Vec<u8> },
}

impl NetClientPeer {
    pub fn create(
        game_id: u32,
        host_id: HostId,
    ) -> (Arc<Self>, UnboundedReceiver<NetClientPeerEvent>) {
        let (ev_dispatcher, ev_queue) = unbounded_channel::<NetClientPeerEvent>();

        let peer = Arc::new(Self {
            channel: RwLock::new(None),
            ev_dispatcher,
            running: AtomicBool::new(true),
            host_id,
            game_id,
        });

        peer.clone().spawn_worker();

        (peer, ev_queue)
    }

    pub fn send_packet(&self, data: &[u8]) {
        let conn = self.channel.blocking_read();

        if let Some(conn) = &*conn {
            let _ = RUNTIME.block_on(conn.send(&Bytes::copy_from_slice(data)));
        }
    }

    async fn connect_to_host(&self) -> Option<Channel> {
        let mut signaling =
            ClientPeerSignalingClient::new(&self.host_id.server_url, self.game_id).await?;

        let connection = SimpleLocalPeerConnection::build(true).await.ok()?;

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
            let channel = connection.receive_channel().await.ok()?;
            return Some(channel);
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
            let mut lock = self.channel.write().await;
            *lock = Some(conn);
        }

        let _ = self.ev_dispatcher.send(NetClientPeerEvent::Connected);

        while self.running.load(Ordering::Relaxed) {
            let channel = self.channel.read().await;
            let Some(channel) = &*channel else {
                break;
            };

            let Some(packet) = recieve_packet(channel).await else {
                break;
            };

            let _ = self
                .ev_dispatcher
                .send(NetClientPeerEvent::PacketReceived { data: packet });
        }

        let _ = self.ev_dispatcher.send(NetClientPeerEvent::Disconnected);

        self.running.store(false, Ordering::Relaxed);
    }

    pub fn disconnect(&self) {
        self.running.store(false, Ordering::Relaxed);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn spawn_worker(self: Arc<Self>) {
        RUNTIME.spawn(self.worker_loop());
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn_worker(self: Arc<Self>) {
        wasm_bindgen_futures::spawn_local(self.worker_loop());
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
