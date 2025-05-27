use std::{
    ffi::c_char,
    ptr::{self, null, null_mut},
    str::FromStr,
};

use pawkit_net::{NetClientPeerEvent, NetHostPeerEvent, SimpleNetClientPeer, SimpleNetHostPeer};
use pawkit_net_signaling::model::HostId;

use crate::{
    cstr_to_str, disown_str_to_cstr, drop_cstr, drop_from_heap, move_to_heap, ptr_to_slice,
};

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_create(
    server_url: *const c_char,
    game_id: u32,
    request_proxy: bool
) -> *mut SimpleNetHostPeer {
    let Some(server_url) = cstr_to_str(server_url) else {
        return null_mut();
    };

    return move_to_heap(SimpleNetHostPeer::create(server_url, game_id, request_proxy));
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_destroy(peer: *mut SimpleNetHostPeer) {
    if peer.is_null() {
        return;
    }

    drop_from_heap(peer);
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_get_host_id(
    peer: *mut SimpleNetHostPeer,
) -> *const c_char {
    if peer.is_null() {
        return ptr::null();
    }

    let peer = &*peer;

    return disown_str_to_cstr(&peer.get_host_id().to_string());
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_free_host_id(s: *const c_char) {
    drop_cstr(s);
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_send_packet(
    peer: *mut SimpleNetHostPeer,
    client_id: usize,
    data: *const u8,
    size: usize,
) {
    if peer.is_null() || data.is_null() || size == 0 {
        return;
    }

    (*peer).send_packet(client_id, ptr_to_slice(data, size));
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_poll_event(
    peer: *mut SimpleNetHostPeer,
) -> *mut NetHostPeerEvent {
    if peer.is_null() {
        return null_mut();
    }

    let Some(evt) = (*peer).next_event() else {
        return null_mut();
    };

    return move_to_heap(evt);
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_event_free(evt: *mut NetHostPeerEvent) {
    if evt.is_null() {
        return;
    }

    drop_from_heap(evt);
}

const HOST_PEER_CONNECTED: i32 = 0;
const HOST_PEER_DISCONNECTED: i32 = 1;
const HOST_PACKET_RECEIVED: i32 = 2;
const HOST_ID_UPDATED: i32 = 3;

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_event_get_type(evt: *mut NetHostPeerEvent) -> i32 {
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

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_event_get_peer_id(evt: *mut NetHostPeerEvent) -> usize {
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

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_event_get_data(
    evt: *mut NetHostPeerEvent,
    len: *mut usize,
) -> *const u8 {
    if evt.is_null() || len.is_null() {
        return null();
    }

    let NetHostPeerEvent::PacketReceived { peer_id: _, data } = &*evt else {
        return null();
    };

    *len = data.len();

    return data.as_ptr();
}

#[no_mangle]
pub unsafe extern "C" fn pawkit_net_client_peer_create(
    host_id_str: *const c_char,
    game_id: u32,
) -> *mut SimpleNetClientPeer {
    let Some(host_id_str) = cstr_to_str(host_id_str) else {
        return null_mut();
    };

    let Ok(host_id) = HostId::from_str(host_id_str) else {
        return null_mut();
    };

    move_to_heap(SimpleNetClientPeer::create(game_id, host_id))
}

#[no_mangle]
pub unsafe extern "C" fn pawkit_net_client_peer_destroy(peer: *mut SimpleNetClientPeer) {
    if peer.is_null() {
        return;
    }

    drop_from_heap(peer);
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_client_peer_send_packet(
    peer: *mut SimpleNetClientPeer,
    data: *const u8,
    size: usize,
) {
    if peer.is_null() || data.is_null() || size == 0 {
        return;
    }

    (*peer).send_packet(ptr_to_slice(data, size));
}

#[no_mangle]
pub unsafe extern "C" fn pawkit_net_client_peer_poll_event(
    peer: *mut SimpleNetClientPeer,
) -> *mut NetClientPeerEvent {
    if peer.is_null() {
        return null_mut();
    }

    let Some(evt) = (*peer).next_event() else {
        return null_mut();
    };

    move_to_heap(evt)
}

const CLIENT_CONNECTED: i32 = 0;
const CLIENT_DISCONNECTED: i32 = 1;
const CLIENT_CONNECTION_FAILED: i32 = 2;
const CLIENT_PACKET_RECEIVED: i32 = 3;

#[no_mangle]
pub unsafe extern "C" fn pawkit_net_client_event_get_type(evt: *mut NetClientPeerEvent) -> i32 {
    if evt.is_null() {
        return -1;
    }

    match &*evt {
        NetClientPeerEvent::Connected => CLIENT_CONNECTED,
        NetClientPeerEvent::Disconnected => CLIENT_DISCONNECTED,
        NetClientPeerEvent::ConnectionFailed => CLIENT_CONNECTION_FAILED,
        NetClientPeerEvent::PacketReceived { data: _ } => CLIENT_PACKET_RECEIVED,
    }
}

#[no_mangle]
pub unsafe extern "C" fn pawkit_net_client_event_get_data(
    evt: *mut NetClientPeerEvent,
    len: *mut usize,
) -> *const u8 {
    if evt.is_null() || len.is_null() {
        return null();
    }

    let NetClientPeerEvent::PacketReceived { data } = &*evt else {
        return null();
    };

    *len = data.len();

    return data.as_ptr();
}
