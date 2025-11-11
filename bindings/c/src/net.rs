use std::{
    ffi::c_char,
    ptr::{self, null, null_mut},
    str::FromStr,
};

use pawkit_net::{NetClientPeerEvent, NetHostPeerEvent, SimpleNetClientPeer, SimpleNetHostPeer};
use pawkit_net_signaling::model::{ChannelConfiguration, HostId};

use crate::{
    c_enum, cstr_to_str, disown_str_to_cstr, drop_from_heap, move_to_heap, ptr_to_ref,
    ptr_to_ref_mut, ptr_to_slice,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct CChannelConfig {
    ordered: bool,
    retries: u16,
}

impl Into<ChannelConfiguration> for CChannelConfig {
    fn into(self) -> ChannelConfiguration {
        return ChannelConfiguration {
            ordered: self.ordered,
            reliability: if self.retries == u16::MAX {
                None
            } else {
                Some(self.retries.try_into().unwrap())
            },
        };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_create(
    server_url: *const c_char,
    server_url_len: usize,
    game_id: u32,
    request_proxy: bool,
    channels: *const CChannelConfig,
    channels_size: usize,
) -> *mut SimpleNetHostPeer {
    unsafe {
        let Some(server_url) = cstr_to_str(server_url, server_url_len) else {
            return null_mut();
        };

        let Some(channels) = ptr_to_slice(channels, channels_size) else {
            return null_mut();
        };

        return move_to_heap(SimpleNetHostPeer::create(
            server_url,
            game_id,
            request_proxy,
            channels
                .iter()
                .map(|config: &CChannelConfig| CChannelConfig::into(*config))
                .collect(),
        ));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_free(peer: *mut SimpleNetHostPeer) {
    unsafe {
        if peer.is_null() {
            return;
        }

        drop_from_heap(peer);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_get_host_id(
    peer: *mut SimpleNetHostPeer,
    len: *mut usize,
) -> *const c_char {
    unsafe {
        if peer.is_null() {
            return ptr::null();
        }

        let Some(len) = ptr_to_ref_mut(len) else {
            return null_mut();
        };

        let peer = &*peer;

        return disown_str_to_cstr(&peer.get_host_id().to_string(), len);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_send_packet(
    peer: *mut SimpleNetHostPeer,
    client_id: usize,
    channel: usize,
    data: *const u8,
    size: usize,
) {
    unsafe {
        let Some(peer) = ptr_to_ref(peer) else {
            return;
        };

        let Some(data) = ptr_to_slice(data, size) else {
            return;
        };

        peer.send_packet(client_id, channel, data);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_poll_event(
    peer: *mut SimpleNetHostPeer,
) -> *mut NetHostPeerEvent {
    unsafe {
        if peer.is_null() {
            return null_mut();
        }

        let Some(evt) = (*peer).next_event() else {
            return null_mut();
        };

        return move_to_heap(evt);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_event_free(evt: *mut NetHostPeerEvent) {
    unsafe {
        if evt.is_null() {
            return;
        }

        drop_from_heap(evt);
    }
}

c_enum!(CHostPeerEventType : u8 {
    HOST_PEER_CONNECTED,
    HOST_PEER_DISCONNECTED,
    HOST_PACKET_RECEIVED,
    HOST_ID_UPDATED,
});

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_event_get_type(
    evt: *mut NetHostPeerEvent,
) -> CHostPeerEventType {
    unsafe {
        if evt.is_null() {
            return 255;
        }

        return match &*evt {
            NetHostPeerEvent::PeerConnected { peer_id: _ } => HOST_PEER_CONNECTED,
            NetHostPeerEvent::PeerDisconnected { peer_id: _ } => HOST_PEER_DISCONNECTED,
            NetHostPeerEvent::PacketReceived {
                peer_id: _,
                data: _,
                channel: _,
            } => HOST_PACKET_RECEIVED,
            NetHostPeerEvent::HostIdUpdated => HOST_ID_UPDATED,
        };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_event_get_peer_id(evt: *mut NetHostPeerEvent) -> usize {
    unsafe {
        if evt.is_null() {
            return usize::MAX;
        }

        return match &*evt {
            NetHostPeerEvent::PeerConnected { peer_id } => *peer_id,
            NetHostPeerEvent::PeerDisconnected { peer_id } => *peer_id,
            NetHostPeerEvent::PacketReceived {
                peer_id,
                data: _,
                channel: _,
            } => *peer_id,
            NetHostPeerEvent::HostIdUpdated => usize::MAX,
        };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_event_get_data(
    evt: *mut NetHostPeerEvent,
    len: *mut usize,
) -> *const u8 {
    unsafe {
        if evt.is_null() || len.is_null() {
            return null();
        }

        let NetHostPeerEvent::PacketReceived {
            peer_id: _,
            data,
            channel: _,
        } = &*evt
        else {
            return null();
        };

        *len = data.len();

        return data.as_ptr();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_peer_create(
    host_id: *const c_char,
    host_id_len: usize,
    game_id: u32,
) -> *mut SimpleNetClientPeer {
    unsafe {
        let Some(host_id) = cstr_to_str(host_id, host_id_len) else {
            return null_mut();
        };

        let Ok(host_id) = HostId::from_str(host_id) else {
            return null_mut();
        };

        return move_to_heap(SimpleNetClientPeer::create(game_id, host_id));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_peer_free(peer: *mut SimpleNetClientPeer) {
    unsafe {
        if peer.is_null() {
            return;
        }

        drop_from_heap(peer);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_peer_send_packet(
    peer: *mut SimpleNetClientPeer,
    channel: usize,
    data: *const u8,
    size: usize,
) {
    unsafe {
        let Some(peer) = ptr_to_ref(peer) else {
            return;
        };

        let Some(data) = ptr_to_slice(data, size) else {
            return;
        };

        peer.send_packet(channel, data);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_peer_poll_event(
    peer: *mut SimpleNetClientPeer,
) -> *mut NetClientPeerEvent {
    unsafe {
        if peer.is_null() {
            return null_mut();
        }

        let Some(evt) = (*peer).next_event() else {
            return null_mut();
        };

        return move_to_heap(evt);
    }
}

c_enum!(CClientPeerEventType : u8 {
    CLIENT_CONNECTED,
    CLIENT_DISCONNECTED,
    CLIENT_CONNECTION_FAILED,
    CLIENT_PACKET_RECEIVED,
});

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_event_get_type(
    evt: *mut NetClientPeerEvent,
) -> CClientPeerEventType {
    unsafe {
        if evt.is_null() {
            return 255;
        }

        return match &*evt {
            NetClientPeerEvent::Connected => CLIENT_CONNECTED,
            NetClientPeerEvent::Disconnected => CLIENT_DISCONNECTED,
            NetClientPeerEvent::ConnectionFailed => CLIENT_CONNECTION_FAILED,
            NetClientPeerEvent::PacketReceived {
                data: _,
                channel: _,
            } => CLIENT_PACKET_RECEIVED,
        };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_event_get_data(
    evt: *mut NetClientPeerEvent,
    len: *mut usize,
) -> *const u8 {
    unsafe {
        if evt.is_null() || len.is_null() {
            return null();
        }

        let NetClientPeerEvent::PacketReceived { data, channel: _ } = &*evt else {
            return null();
        };

        *len = data.len();

        return data.as_ptr();
    }
}
