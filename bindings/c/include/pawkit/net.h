#pragma once

#include <assert.h>
#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct pawkit_net_host_peer *pawkit_net_host_peer_t;

typedef struct pawkit_net_host_event *pawkit_net_host_event_t;

enum {
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_CONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_DISCONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED,
    PAWKIT_NET_HOST_EVENT_TYPE_HOST_ID_UPDATED,
};

typedef pawkit_u8 pawkit_net_host_event_type_t;

pawkit_net_host_peer_t pawkit_net_host_peer_create(char const *server_url, pawkit_usize server_url_size, pawkit_u32 game_id, bool request_proxy);
void pawkit_net_host_peer_free(pawkit_net_host_peer_t peer);

char const *pawkit_net_host_peer_get_host_id(pawkit_net_host_peer_t peer, pawkit_usize *size);

void pawkit_net_host_peer_send_packet(pawkit_net_host_peer_t peer, pawkit_usize peer_id, pawkit_u8 *data, pawkit_usize size);

pawkit_net_host_event_t pawkit_net_host_peer_poll_event(pawkit_net_host_peer_t peer);
void pawkit_net_host_event_free(pawkit_net_host_event_t evt);

pawkit_net_host_event_type_t pawkit_net_host_event_get_type(pawkit_net_host_event_t evt);
pawkit_usize pawkit_net_host_event_get_peer_id(pawkit_net_host_event_t evt);
/// Ownership is retained by the event. Can be NULL.
pawkit_u8 const *pawkit_net_host_event_get_data(pawkit_net_host_event_t evt, pawkit_usize *size);

typedef struct pawkit_net_client_peer *pawkit_net_client_peer_t;

typedef struct pawkit_net_client_event *pawkit_net_client_event_t;

enum {
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_DISCONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTION_FAILED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED,
};

typedef pawkit_u8 pawkit_net_client_event_type_t;

pawkit_net_client_peer_t pawkit_net_client_peer_create(char const *host_id, pawkit_usize host_id_size, pawkit_u32 game_id);
void pawkit_net_client_peer_free(pawkit_net_client_peer_t peer);

void pawkit_net_client_peer_send_packet(pawkit_net_client_peer_t peer, pawkit_u8 *data, pawkit_usize size);

pawkit_net_client_event_t pawkit_net_client_peer_poll_event(pawkit_net_client_peer_t peer);
void pawkit_net_client_event_free(pawkit_net_client_event_t evt);

pawkit_net_client_event_type_t pawkit_net_client_event_get_type(pawkit_net_client_event_t evt);
/// Ownership is retained by the event. Can be NULL.
pawkit_u8 const *pawkit_net_client_event_get_data(pawkit_net_client_event_t evt, pawkit_usize *size);

#ifdef __cplusplus
}
#endif
