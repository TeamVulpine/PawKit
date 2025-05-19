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

void pawkit_net_host_peer_send_packet(pawkit_net_host_peer peer, size_t peer_id, uint8_t *data, size_t size);

pawkit_net_host_event pawkit_net_host_poll_event(pawkit_net_host_peer peer);
void pawkit_net_host_event_free(pawkit_net_host_event evt);

pawkit_net_host_event_type pawkit_net_host_event_get_type(pawkit_net_host_event evt);
size_t pawkit_net_host_event_get_peer_id(pawkit_net_host_event evt);
/// Ownership is retained by the event.
uint8_t *pawkit_net_host_event_get_data(pawkit_net_host_event evt, size_t *size);

#ifdef __cplusplus
}

#include <string>
#include <array>
#include <vector>

namespace PawKit::Networking {
    struct NetHostPeerEvent {
        friend struct NetHostPeer;

        private:
        pawkit_net_host_event evt;

        NetHostPeerEvent(pawkit_net_host_event evt) {
            this->evt = evt;
        }

        public:
        NetHostPeerEvent() : evt(nullptr) {}

        ~NetHostPeerEvent() {
            pawkit_net_host_event_free(evt);
        }

        size_t GetPeerId() {
            return pawkit_net_host_event_get_peer_id(evt);
        }
    };

    struct NetHostPeer {
        private:
        pawkit_net_host_peer peer;

        public:
        inline NetHostPeer(std::string &&serverUrl, uint32_t gameId) {
            peer = pawkit_net_host_peer_create(serverUrl.data(), gameId);
            assert(peer != nullptr);
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
            pawkit_net_host_event rawEvt = pawkit_net_host_poll_event(peer);
            if (rawEvt == nullptr)
                return false;

            evt = NetHostPeerEvent(rawEvt);
            
            return true;
        }
    };
}

#endif
