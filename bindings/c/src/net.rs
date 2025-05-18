use std::{ffi::c_char, ptr::null_mut};

use pawkit_net_runtime::{NetHostPeerEvent, SimpleNetHostPeer};

use crate::{cstr_to_str, drop_from_heap, move_to_heap, ptr_to_slice};

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_create(
    server_url: *const c_char,
    game_id: u32,
) -> *mut SimpleNetHostPeer {
    let Some(server_url) = cstr_to_str(server_url) else {
        return null_mut();
    };

    let Some(peer) = SimpleNetHostPeer::create(server_url, game_id) else {
        return null_mut();
    };

    return move_to_heap(peer);
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_destroy(peer: *mut SimpleNetHostPeer) {
    if peer.is_null() {
        return;
    }

    drop_from_heap(peer);
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
unsafe extern "C" fn pawkit_net_host_poll_event(peer: *mut SimpleNetHostPeer) -> *mut NetHostPeerEvent {
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

const PEER_CONNECTED: i32 = 0;
const PEER_DISCONNECTED: i32 = 1;
const PACKET_RECIEVED: i32 = 2;
const HOST_ID_UPDATED: i32 = 3;

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_event_get_type(evt: *mut NetHostPeerEvent) -> i32 {
    return match *evt {
        NetHostPeerEvent::PeerConnected { peer_id: _ } => PEER_CONNECTED,
        NetHostPeerEvent::PeerDisconnected { peer_id: _ } => PEER_DISCONNECTED,
        NetHostPeerEvent::PacketRecieved { peer_id: _, data: _ } => PACKET_RECIEVED,
        NetHostPeerEvent::HostIdUpdated => HOST_ID_UPDATED
    };
}
