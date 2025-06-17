#pragma once

#include <assert.h>
#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void *pawkit_net_host_peer;

typedef void *pawkit_net_host_event;

typedef enum {
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_CONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_DISCONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED,
    PAWKIT_NET_HOST_EVENT_TYPE_HOST_ID_UPDATED,
} pawkit_net_host_event_type;

pawkit_net_host_peer pawkit_net_host_peer_create(char *server_url, pawkit_u32 game_id, bool request_proxy);
void pawkit_net_host_peer_destroy(pawkit_net_host_peer peer);

char const *pawkit_net_host_peer_get_host_id(pawkit_net_host_peer peer);
void pawkit_net_host_peer_free_host_id(char const *id);

void pawkit_net_host_peer_send_packet(pawkit_net_host_peer peer, pawkit_usize peer_id, pawkit_u8 *data, pawkit_usize size);

pawkit_net_host_event pawkit_net_host_peer_poll_event(pawkit_net_host_peer peer);
void pawkit_net_host_event_free(pawkit_net_host_event evt);

pawkit_net_host_event_type pawkit_net_host_event_get_type(pawkit_net_host_event evt);
pawkit_usize pawkit_net_host_event_get_peer_id(pawkit_net_host_event evt);
/// Ownership is retained by the event. Can be NULL.
pawkit_u8 const *pawkit_net_host_event_get_data(pawkit_net_host_event evt, pawkit_usize *size);

typedef void *pawkit_net_client_peer;

typedef void *pawkit_net_client_event;

typedef enum {
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_DISCONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTION_FAILED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED,
} pawkit_net_client_event_type;

pawkit_net_client_peer pawkit_net_client_peer_create(char *host_id, pawkit_u32 game_id);
void pawkit_net_client_peer_destroy(pawkit_net_client_peer peer);

void pawkit_net_client_peer_send_packet(pawkit_net_client_peer peer, pawkit_u8 *data, pawkit_usize size);

pawkit_net_client_event pawkit_net_client_peer_poll_event(pawkit_net_client_peer peer);
void pawkit_net_client_event_free(pawkit_net_client_event evt);

pawkit_net_client_event_type pawkit_net_client_event_get_type(pawkit_net_client_event evt);
/// Ownership is retained by the event. Can be NULL.
pawkit_u8 const *pawkit_net_client_event_get_data(pawkit_net_client_event evt, pawkit_usize *size);

#ifdef __cplusplus
}

#include <string>
#include <span>

namespace PawKit::Networking {
    struct NetHostPeerEvent : OpaqueShared<pawkit_net_host_event> {
        friend struct NetHostPeer;

        private:
        NetHostPeerEvent(pawkit_net_host_event evt) : OpaqueShared(evt, pawkit_net_host_event_free) {}

        public:
        NetHostPeerEvent() : OpaqueShared(nullptr, NullDeleter) {}

        pawkit_net_host_event_type GetType() {
            return pawkit_net_host_event_get_type(Get());
        }

        pawkit_usize GetPeerId() {
            return pawkit_net_host_event_get_peer_id(Get());
        }

        std::span<const pawkit_u8> GetData() {
            if (GetType() != PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED)
                return {};

            pawkit_usize size = 0;

            pawkit_u8 const *data = pawkit_net_host_event_get_data(Get(), &size);

            if (!data)
                return {};

            return std::span(data, size);
        }
    };

    struct NetHostPeer : OpaqueShared<pawkit_net_host_peer> {
        public:
        inline NetHostPeer(std::string &&serverUrl, pawkit_u32 gameId, bool requestProxy) :
            OpaqueShared(
                pawkit_net_host_peer_create(serverUrl.data(), gameId, requestProxy),
                pawkit_net_host_peer_destroy
            )
        {}

        inline void SendPacket(pawkit_usize peerId, pawkit_u8 *data, pawkit_usize size) {
            pawkit_net_host_peer_send_packet(Get(), peerId, data, size);
        }

        inline void SendPacket(pawkit_usize peerId, std::span<pawkit_u8> data) {
            SendPacket(peerId, data.data(), data.size());
        }

        inline bool PollEvent(NetHostPeerEvent &evt) {
            pawkit_net_host_event rawEvt = pawkit_net_host_peer_poll_event(Get());
            if (rawEvt == nullptr)
                return false;

            evt = NetHostPeerEvent(rawEvt);

            return true;
        }

        inline std::string GetHostId() {
            char const* rawId = pawkit_net_host_peer_get_host_id(Get());

            std::string id = rawId;

            pawkit_net_host_peer_free_host_id(rawId);

            return id;
        }
    };

    struct NetClientPeerEvent : OpaqueShared<pawkit_net_client_event> {
        friend struct NetClientPeer;

        private:
        NetClientPeerEvent(pawkit_net_client_event evt) :
            OpaqueShared(evt, pawkit_net_client_event_free)
        {}

        public:
        NetClientPeerEvent() : OpaqueShared(nullptr, NullDeleter) {}

        pawkit_net_client_event_type GetType() {
            return pawkit_net_client_event_get_type(Get());
        }

        std::span<const pawkit_u8> GetData() {
            if (GetType() != PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED)
                return {};

            pawkit_usize size = 0;
            pawkit_u8 const *data = pawkit_net_client_event_get_data(Get(), &size);
            if (!data)
                return {};

            return std::span(data, data + size);
        }
    };

    struct NetClientPeer : OpaqueShared<pawkit_net_client_peer> {
        inline NetClientPeer(std::string &&hostId, pawkit_u32 gameId) :
            OpaqueShared(
                pawkit_net_client_peer_create(hostId.data(), gameId),
                pawkit_net_client_peer_destroy
            )
        {}

        inline void SendPacket(pawkit_u8 *data, pawkit_usize size) {
            pawkit_net_client_peer_send_packet(Get(), data, size);
        }

        inline void SendPacket(std::span<pawkit_u8> data) {
            SendPacket(data.data(), data.size());
        }

        inline bool PollEvent(NetClientPeerEvent& evt) {
            pawkit_net_client_event rawEvt = pawkit_net_client_peer_poll_event(Get());
            if (rawEvt == nullptr)
                return false;

            evt = NetClientPeerEvent(rawEvt);
            return true;
        }
    };
}

#endif
