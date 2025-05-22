use std::sync::atomic::AtomicBool;

use just_webrtc::platform::Channel;
use tokio::sync::mpsc::UnboundedSender;

pub struct NetClientPeer {
    channel: Channel,
    ev_dispatcher: UnboundedSender<NetClientPeerEvent>,
    running: AtomicBool,
}

#[derive(Debug)]
pub enum NetClientPeerEvent {
    PeerConnected { peer_id: usize },
    PeerDisconnected { peer_id: usize },
    PacketReceived { peer_id: usize, data: Vec<u8> },
    HostIdUpdated,
}
