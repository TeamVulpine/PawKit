#pragma once

#include <assert.h>
#include <stddef.h>
#include <stdint.h>

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

pawkit_net_host_peer pawkit_net_host_peer_create(char *server_url, uint32_t game_id);
void pawkit_net_host_peer_destroy(pawkit_net_host_peer peer);

char const *pawkit_net_host_peer_get_host_id(pawkit_net_host_peer peer);
void pawkit_net_host_peer_free_host_id(char const *id);

void pawkit_net_host_peer_send_packet(pawkit_net_host_peer peer, size_t peer_id, uint8_t *data, size_t size);

pawkit_net_host_event pawkit_net_host_peer_poll_event(pawkit_net_host_peer peer);
void pawkit_net_host_event_free(pawkit_net_host_event evt);

pawkit_net_host_event_type pawkit_net_host_event_get_type(pawkit_net_host_event evt);
size_t pawkit_net_host_event_get_peer_id(pawkit_net_host_event evt);
/// Ownership is retained by the event. Can be NULL.
uint8_t *pawkit_net_host_event_get_data(pawkit_net_host_event evt, size_t *size);

typedef void *pawkit_net_client_peer;

typedef void *pawkit_net_client_event;

typedef enum {
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_DISCONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTION_FAILED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED,
} pawkit_net_client_event_type;

pawkit_net_client_peer pawkit_net_client_peer_create(char *host_id, uint32_t game_id);
void pawkit_net_client_peer_destroy(pawkit_net_client_peer peer);

void pawkit_net_client_peer_send_packet(pawkit_net_client_peer peer, uint8_t *data, size_t size);

pawkit_net_client_event pawkit_net_client_peer_poll_event(pawkit_net_client_peer peer);
void pawkit_net_client_event_free(pawkit_net_client_event evt);

pawkit_net_client_event_type pawkit_net_client_event_get_type(pawkit_net_client_event evt);
/// Ownership is retained by the event. Can be NULL.
uint8_t *pawkit_net_client_event_get_data(pawkit_net_client_event evt, size_t *size);

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

        size_t GetPeerId() {
            return pawkit_net_host_event_get_peer_id(evt);
        }

        std::optional<std::vector<uint8_t>> GetData() {
            if (GetType() != PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED)
                return std::nullopt;

            size_t size = 0;

            uint8_t *data = pawkit_net_host_event_get_data(evt, &size);

            if (!data)
                return std::nullopt;

            return std::vector<uint8_t>(data, data + size);
        }
    };

    struct NetHostPeer {
        private:
        pawkit_net_host_peer peer {nullptr};

        public:
        inline NetHostPeer(std::string &&serverUrl, uint32_t gameId) :
            peer(pawkit_net_host_peer_create(serverUrl.data(), gameId))
        {
            assert(peer != nullptr);
        }

        inline operator pawkit_net_host_peer () const {
            return peer;
        }

        inline ~NetHostPeer() {
            pawkit_net_host_peer_destroy(peer);
        }

        inline void SendPacket(size_t peerId, uint8_t *data, size_t size) {
            pawkit_net_host_peer_send_packet(peer, peerId, data, size);
        }

        inline void SendPacket(size_t peerId, std::vector<uint8_t> &&data) {
            SendPacket(peerId, data.data(), data.size());
        }

        template <size_t TSize>
        inline void SendPacket(size_t peerId, std::array<uint8_t, TSize> &&data) {
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

        std::optional<std::vector<uint8_t>> GetData() {
            if (GetType() != PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED)
                return std::nullopt;

            size_t size = 0;
            uint8_t* data = pawkit_net_client_event_get_data(evt, &size);
            if (!data)
                return std::nullopt;

            return std::vector<uint8_t>(data, data + size);
        }
    };

    struct NetClientPeer {
        private:
        pawkit_net_client_peer peer {nullptr};

        public:
        inline NetClientPeer(std::string&& hostId, uint32_t gameId) :
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

        inline void SendPacket(uint8_t* data, size_t size) {
            pawkit_net_client_peer_send_packet(peer, data, size);
        }

        inline void SendPacket(std::vector<uint8_t>&& data) {
            SendPacket(data.data(), data.size());
        }

        template <size_t TSize>
        inline void SendPacket(std::array<uint8_t, TSize>&& data) {
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
