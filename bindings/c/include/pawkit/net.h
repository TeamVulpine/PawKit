#pragma once

#include <cstddef>
#include <cstdint>
#ifdef __cplusplus
extern "C" {
#endif

typedef void *pawkit_net_host_peer;

typedef void *pawkit_net_host_event;

typedef enum {
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_CONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_DISCONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECIEVED,
    PAWKIT_NET_HOST_EVENT_TYPE_HOST_ID_UPDATED,
} pawkit_net_host_event_type;

pawkit_net_host_peer pawkit_net_host_peer_create(char *url, uint32_t game_id);
void pawkit_net_host_peer_destroy(pawkit_net_host_peer peer);

void pawkit_net_host_peer_send_packet(pawkit_net_host_peer peer, size_t client_id, uint8_t *data, size_t size);

pawkit_net_host_event pawkit_net_host_poll_event(pawkit_net_host_peer peer);
void pawkit_net_host_event_free(pawkit_net_host_event evt);

pawkit_net_host_event_type pawkit_net_host_event_get_type(pawkit_net_host_event evt);
size_t pawkit_net_host_event_get_client_id(pawkit_net_host_event evt);
/// Ownership is retained by the event.
uint8_t *pawkit_net_host_event_get_data(pawkit_net_host_event evt, size_t *len);

#ifdef __cplusplus
}

namespace PawKit::Networking {
}

#undef LOGFUNC

#endif

#undef NAMESPACE
