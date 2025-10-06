use std::{
    ffi::c_char,
    ptr::{self, null, null_mut},
    str::FromStr,
};

use pawkit_net::{NetClientPeerEvent, NetHostPeerEvent, SimpleNetClientPeer, SimpleNetHostPeer};
use pawkit_net_signaling::model::HostId;

use crate::{
    c_enum, cstr_to_str, disown_str_to_cstr, drop_cstr, drop_from_heap, move_to_heap, ptr_to_ref,
    ptr_to_slice,
};

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_create(
    server_url: *const c_char,
    game_id: u32,
    request_proxy: bool,
) -> *mut SimpleNetHostPeer {
    unsafe {
        let Some(server_url) = cstr_to_str(server_url) else {
            return null_mut();
        };

        return move_to_heap(SimpleNetHostPeer::create(
            server_url,
            game_id,
            request_proxy,
        ));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_destroy(peer: *mut SimpleNetHostPeer) {
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
) -> *const c_char {
    unsafe {
        if peer.is_null() {
            return ptr::null();
        }

        let peer = &*peer;

        return disown_str_to_cstr(&peer.get_host_id().to_string());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_free_host_id(s: *const c_char) {
    unsafe {
        drop_cstr(s);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_host_peer_send_packet(
    peer: *mut SimpleNetHostPeer,
    client_id: usize,
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

        peer.send_packet(client_id, data);
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

c_enum!(CHostPeerEventType {
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
            return -1;
        }

        return match &*evt {
            NetHostPeerEvent::PeerConnected { peer_id: _ } => HOST_PEER_CONNECTED,
            NetHostPeerEvent::PeerDisconnected { peer_id: _ } => HOST_PEER_DISCONNECTED,
            NetHostPeerEvent::PacketReceived {
                peer_id: _,
                data: _,
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
            NetHostPeerEvent::PacketReceived { peer_id, data: _ } => *peer_id,
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

        let NetHostPeerEvent::PacketReceived { peer_id: _, data } = &*evt else {
            return null();
        };

        *len = data.len();

        return data.as_ptr();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_peer_create(
    host_id_str: *const c_char,
    game_id: u32,
) -> *mut SimpleNetClientPeer {
    unsafe {
        let Some(host_id_str) = cstr_to_str(host_id_str) else {
            return null_mut();
        };

        let Ok(host_id) = HostId::from_str(host_id_str) else {
            return null_mut();
        };

        return move_to_heap(SimpleNetClientPeer::create(game_id, host_id));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_net_client_peer_destroy(peer: *mut SimpleNetClientPeer) {
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

        peer.send_packet(data);
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

c_enum!(CClientPeerEventType {
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
            return -1;
        }

        return match &*evt {
            NetClientPeerEvent::Connected => CLIENT_CONNECTED,
            NetClientPeerEvent::Disconnected => CLIENT_DISCONNECTED,
            NetClientPeerEvent::ConnectionFailed => CLIENT_CONNECTION_FAILED,
            NetClientPeerEvent::PacketReceived { data: _ } => CLIENT_PACKET_RECEIVED,
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

        let NetClientPeerEvent::PacketReceived { data } = &*evt else {
            return null();
        };

        *len = data.len();

        return data.as_ptr();
    }
}
