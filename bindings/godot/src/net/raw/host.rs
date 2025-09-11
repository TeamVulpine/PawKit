use std::{ops::Deref, sync::Arc};

use godot::{
    builtin::{GString, PackedByteArray},
    obj::Gd,
    prelude::{Export, GodotClass, GodotConvert, Var, godot_api},
};
use pawkit_net::{NetHostPeer, NetHostPeerEvent};
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(GodotClass)]
#[class(no_init)]
struct PawkitNetHostPeer {
    peer: Arc<NetHostPeer>,
    recv: UnboundedReceiver<NetHostPeerEvent>,
}

#[godot_api]
impl PawkitNetHostPeer {
    #[func]
    fn new(server_url: GString, game_id: u32) -> Gd<Self> {
        let (peer, recv) = NetHostPeer::create(&server_url.to_string(), game_id, false);

        return Gd::from_init_fn(|_| Self { peer, recv });
    }

    #[func]
    fn host_id(&self) -> GString {
        return self.peer.get_host_id().to_string().into();
    }

    #[func]
    fn send_packet(&self, client_id: i64, data: PackedByteArray) {
        self.peer.send_packet(client_id as usize, data.as_slice());
    }

    #[func]
    fn poll_event(&mut self) -> Option<Gd<PawkitNetHostPeerEvent>> {
        let ev = self.recv.try_recv().ok()?;

        return Some(PawkitNetHostPeerEvent::new(ev));
    }
}

#[repr(i32)]
#[derive(
    Debug, Copy, Clone, GodotConvert, Var, Export, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[godot(via = i32)]
enum PawkitNetHostPeerEventType {
    #[default]
    PeerConnected = 0,
    PeerDisconnected = 1,
    PacketReceived = 2,
    HostIdUpdated = 3,
}

#[derive(GodotClass)]
#[class(init, base=Object)]
struct PawkitNetHostPeerEvent {
    #[export]
    event_type: PawkitNetHostPeerEventType,
    #[export]
    peer_id: i64,
    #[export]
    data: PackedByteArray,
}

#[godot_api]
impl PawkitNetHostPeerEvent {
    #[constant]
    const PEER_CONNECTED: i32 = PawkitNetHostPeerEventType::PeerConnected as i32;
    #[constant]
    const PEER_DISCONNECTED: i32 = PawkitNetHostPeerEventType::PeerDisconnected as i32;
    #[constant]
    const HOST_ID_UPDATED: i32 = PawkitNetHostPeerEventType::PacketReceived as i32;
    #[constant]
    const PACKET_RECIEVED: i32 = PawkitNetHostPeerEventType::HostIdUpdated as i32;

    fn new(ev: NetHostPeerEvent) -> Gd<Self> {
        let (event_type, peer_id, data) = match ev {
            NetHostPeerEvent::HostIdUpdated => (
                PawkitNetHostPeerEventType::HostIdUpdated,
                0,
                PackedByteArray::new(),
            ),
            NetHostPeerEvent::PeerConnected { peer_id } => (
                PawkitNetHostPeerEventType::PeerConnected,
                peer_id as i64,
                PackedByteArray::new(),
            ),
            NetHostPeerEvent::PeerDisconnected { peer_id } => (
                PawkitNetHostPeerEventType::PeerDisconnected,
                peer_id as i64,
                PackedByteArray::new(),
            ),
            NetHostPeerEvent::PacketReceived { peer_id, data } => (
                PawkitNetHostPeerEventType::PacketReceived,
                peer_id as i64,
                PackedByteArray::from(data.deref()),
            ),
        };

        return Gd::from_init_fn(|_| Self {
            event_type,
            peer_id,
            data,
        });
    }
}
