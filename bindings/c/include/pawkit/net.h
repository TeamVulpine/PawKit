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
pawkit_u8 *pawkit_net_host_event_get_data(pawkit_net_host_event evt, pawkit_usize *size);

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
pawkit_u8 *pawkit_net_client_event_get_data(pawkit_net_client_event evt, pawkit_usize *size);

#ifdef __cplusplus
}

#include <string>
#include <array>
#include <vector>
#include <optional>

namespace PawKit::Networking {
    struct NetHostPeerEvent {
        friend struct NetHostPeer;

        private:
        pawkit_net_host_event evt {nullptr};

        NetHostPeerEvent(pawkit_net_host_event evt) : evt(evt) {}

        public:
        NetHostPeerEvent() : evt(nullptr) {}

        inline operator pawkit_net_host_event () const {
            return evt;
        }

        ~NetHostPeerEvent() {
            pawkit_net_host_event_free(evt);
        }

        pawkit_net_host_event_type GetType() {
            return pawkit_net_host_event_get_type(evt);
        }

        pawkit_usize GetPeerId() {
            return pawkit_net_host_event_get_peer_id(evt);
        }

        std::optional<std::vector<pawkit_u8>> GetData() {
            if (GetType() != PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED)
                return std::nullopt;

            pawkit_usize size = 0;

            pawkit_u8 *data = pawkit_net_host_event_get_data(evt, &size);

            if (!data)
                return std::nullopt;

            return std::vector<pawkit_u8>(data, data + size);
        }
    };

    struct NetHostPeer {
        private:
        pawkit_net_host_peer peer {nullptr};

        public:
        inline NetHostPeer(std::string &&serverUrl, pawkit_u32 gameId, bool requestProxy) :
            peer(pawkit_net_host_peer_create(serverUrl.data(), gameId, requestProxy))
        {
            assert(peer != nullptr);
        }

        inline operator pawkit_net_host_peer () const {
            return peer;
        }

        inline ~NetHostPeer() {
            pawkit_net_host_peer_destroy(peer);
        }

        inline void SendPacket(pawkit_usize peerId, pawkit_u8 *data, pawkit_usize size) {
            pawkit_net_host_peer_send_packet(peer, peerId, data, size);
        }

        inline void SendPacket(pawkit_usize peerId, std::vector<pawkit_u8> &&data) {
            SendPacket(peerId, data.data(), data.size());
        }

        template <pawkit_usize TSize>
        inline void SendPacket(pawkit_usize peerId, std::array<pawkit_u8, TSize> &&data) {
            SendPacket(peerId, data.data(), data.size());
        }

        inline bool PollEvent(NetHostPeerEvent &evt) {
            pawkit_net_host_event rawEvt = pawkit_net_host_peer_poll_event(peer);
            if (rawEvt == nullptr)
                return false;

            evt = NetHostPeerEvent(rawEvt);

            return true;
        }

        inline std::string GetHostId() {
            char const* rawId = pawkit_net_host_peer_get_host_id(peer);

            std::string id = rawId;

            pawkit_net_host_peer_free_host_id(rawId);

            return id;
        }
    };

    struct NetClientPeerEvent {
        friend struct NetClientPeer;

        private:
        pawkit_net_client_event evt {nullptr};

        NetClientPeerEvent(pawkit_net_client_event evt) : evt(evt) {}

        public:
        NetClientPeerEvent() : evt(nullptr) {}

        inline operator pawkit_net_client_event() const {
            return evt;
        }

        ~NetClientPeerEvent() {
            pawkit_net_client_event_free(evt);
        }

        pawkit_net_client_event_type GetType() {
            return pawkit_net_client_event_get_type(evt);
        }

        std::optional<std::vector<pawkit_u8>> GetData() {
            if (GetType() != PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED)
                return std::nullopt;

            pawkit_usize size = 0;
            pawkit_u8* data = pawkit_net_client_event_get_data(evt, &size);
            if (!data)
                return std::nullopt;

            return std::vector<pawkit_u8>(data, data + size);
        }
    };

    struct NetClientPeer {
        private:
        pawkit_net_client_peer peer {nullptr};

        public:
        inline NetClientPeer(std::string&& hostId, pawkit_u32 gameId) :
            peer(pawkit_net_client_peer_create(hostId.data(), gameId))
        {
            assert(peer != nullptr);
        }

        inline operator pawkit_net_client_peer() const {
            return peer;
        }

        inline ~NetClientPeer() {
            pawkit_net_client_peer_destroy(peer);
        }

        inline void SendPacket(pawkit_u8* data, pawkit_usize size) {
            pawkit_net_client_peer_send_packet(peer, data, size);
        }

        inline void SendPacket(std::vector<pawkit_u8>&& data) {
            SendPacket(data.data(), data.size());
        }

        template <pawkit_usize TSize>
        inline void SendPacket(std::array<pawkit_u8, TSize>&& data) {
            SendPacket(data.data(), data.size());
        }

        inline bool PollEvent(NetClientPeerEvent& evt) {
            pawkit_net_client_event rawEvt = pawkit_net_client_peer_poll_event(peer);
            if (rawEvt == nullptr)
                return false;

            evt = NetClientPeerEvent(rawEvt);
            return true;
        }
    };
}

#endif
