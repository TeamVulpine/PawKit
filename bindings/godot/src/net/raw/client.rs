use std::{ops::Deref, str::FromStr, sync::Arc};

use godot::{
    builtin::{GString, PackedByteArray},
    global::godot_error,
    obj::Gd,
    prelude::{Export, GodotClass, GodotConvert, Var, godot_api},
};
use pawkit_net::{NetClientPeer, NetClientPeerEvent};
use pawkit_net_signaling::model::HostId;
use tokio::sync::mpsc::UnboundedReceiver;

#[derive(GodotClass)]
#[class(no_init)]
struct PawkitNetClientPeer {
    peer: Arc<NetClientPeer>,
    recv: UnboundedReceiver<NetClientPeerEvent>,
}

#[godot_api]
impl PawkitNetClientPeer {
    #[func]
    fn new(game_id: u32, host_id: GString) -> Option<Gd<Self>> {
        let host_id = match HostId::from_str(&host_id.to_string()) {
            Ok(it) => it,
            Err(err) => {
                godot_error!("{}", err);
                return None;
            }
        };

        let (peer, recv) = NetClientPeer::create(game_id, host_id);

        return Some(Gd::from_init_fn(|_| Self { peer, recv }));
    }

    #[func]
    fn send_packet(&self, channel: i64, data: PackedByteArray) {
        self.peer.send_packet(channel as usize, data.as_slice());
    }

    #[func]
    fn poll_event(&mut self) -> Option<Gd<PawkitNetClientPeerEvent>> {
        let ev = self.recv.try_recv().ok()?;

        return Some(PawkitNetClientPeerEvent::new(ev));
    }
}

#[repr(i32)]
#[derive(
    Debug, Copy, Clone, GodotConvert, Var, Export, Default, PartialEq, Eq, PartialOrd, Ord,
)]
#[godot(via = i32)]
enum PawkitNetClientPeerEventType {
    #[default]
    Connected = 0,
    Disconnected = 1,
    ConnectionFailed = 2,
    PacketReceived = 3,
}

#[derive(GodotClass)]
#[class(init, base=Object)]
struct PawkitNetClientPeerEvent {
    #[export]
    event_type: PawkitNetClientPeerEventType,
    #[export]
    data: PackedByteArray,
    #[export]
    channel: i64,
}

#[godot_api]
impl PawkitNetClientPeerEvent {
    #[constant]
    const CONNECTED: i32 = PawkitNetClientPeerEventType::Connected as i32;
    #[constant]
    const DISCONNECTED: i32 = PawkitNetClientPeerEventType::Disconnected as i32;
    #[constant]
    const CONNECTION_FAILED: i32 = PawkitNetClientPeerEventType::ConnectionFailed as i32;
    #[constant]
    const PACKET_RECIEVED: i32 = PawkitNetClientPeerEventType::PacketReceived as i32;

    fn new(ev: NetClientPeerEvent) -> Gd<Self> {
        let (event_type, data, channel) = match ev {
            NetClientPeerEvent::Connected => (
                PawkitNetClientPeerEventType::Connected,
                PackedByteArray::new(),
                0,
            ),
            NetClientPeerEvent::Disconnected => (
                PawkitNetClientPeerEventType::Disconnected,
                PackedByteArray::new(),
                0,
            ),
            NetClientPeerEvent::ConnectionFailed => (
                PawkitNetClientPeerEventType::ConnectionFailed,
                PackedByteArray::new(),
                0,
            ),
            NetClientPeerEvent::PacketReceived { data, channel } => (
                PawkitNetClientPeerEventType::PacketReceived,
                PackedByteArray::from(data.deref()),
                channel as i64,
            ),
        };

        return Gd::from_init_fn(|_| Self {
            event_type,
            data,
            channel,
        });
    }
}
