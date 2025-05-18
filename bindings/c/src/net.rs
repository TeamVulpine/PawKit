use std::{ffi::c_char, ptr::null_mut};

use pawkit_net_runtime::SimpleNetHostPeer;

use crate::{cstr_to_str, free_from_heap, move_to_heap, ptr_to_slice};

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
    free_from_heap(peer);
}

#[no_mangle]
unsafe extern "C" fn pawkit_net_host_peer_send_packet(
    peer: *mut SimpleNetHostPeer,
    client_id: usize,
    data: *const u8,
    size: usize,
) {
    if peer.is_null() {
        return;
    }

    (*peer).send_packet(client_id, ptr_to_slice(data, size));
}
