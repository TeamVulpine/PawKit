#pragma once

#include <assert.h>
#include <string_view>
#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void *pawkit_net_host_peer_t;

typedef void *pawkit_net_host_event;

enum {
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_CONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PEER_DISCONNECTED,
    PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED,
    PAWKIT_NET_HOST_EVENT_TYPE_HOST_ID_UPDATED,
};

typedef pawkit_u8 pawkit_net_host_event_type_t;

pawkit_net_host_peer_t pawkit_net_host_peer_create(char const *server_url, pawkit_usize server_url_size, pawkit_u32 game_id, bool request_proxy);
void pawkit_net_host_peer_destroy(pawkit_net_host_peer_t peer);

char const *pawkit_net_host_peer_get_host_id(pawkit_net_host_peer_t peer, pawkit_usize *size);

void pawkit_net_host_peer_send_packet(pawkit_net_host_peer_t peer, pawkit_usize peer_id, pawkit_u8 *data, pawkit_usize size);

pawkit_net_host_event pawkit_net_host_peer_poll_event(pawkit_net_host_peer_t peer);
void pawkit_net_host_event_free(pawkit_net_host_event evt);

pawkit_net_host_event_type_t pawkit_net_host_event_get_type(pawkit_net_host_event evt);
pawkit_usize pawkit_net_host_event_get_peer_id(pawkit_net_host_event evt);
/// Ownership is retained by the event. Can be NULL.
pawkit_u8 const *pawkit_net_host_event_get_data(pawkit_net_host_event evt, pawkit_usize *size);

typedef void *pawkit_net_client_peer_t;

typedef void *pawkit_net_client_event_t;

enum {
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_DISCONNECTED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_CONNECTION_FAILED,
    PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED,
};

typedef pawkit_u8 pawkit_net_client_event_type_t;

pawkit_net_client_peer_t pawkit_net_client_peer_create(char const *host_id, pawkit_usize host_id_size, pawkit_u32 game_id);
void pawkit_net_client_peer_destroy(pawkit_net_client_peer_t peer);

void pawkit_net_client_peer_send_packet(pawkit_net_client_peer_t peer, pawkit_u8 *data, pawkit_usize size);

pawkit_net_client_event_t pawkit_net_client_peer_poll_event(pawkit_net_client_peer_t peer);
void pawkit_net_client_event_free(pawkit_net_client_event_t evt);

pawkit_net_client_event_type_t pawkit_net_client_event_get_type(pawkit_net_client_event_t evt);
/// Ownership is retained by the event. Can be NULL.
pawkit_u8 const *pawkit_net_client_event_get_data(pawkit_net_client_event_t evt, pawkit_usize *size);

#ifdef __cplusplus
}

#include <string>
#include <span>

namespace PawKit::Networking {
    struct NetHostPeerEvent {
        ~NetHostPeerEvent() {
            pawkit_net_host_event_free(*this);
        };

        NetHostPeerEvent() = delete;
        NetHostPeerEvent(NetHostPeerEvent const &copy) = delete;
        NetHostPeerEvent(NetHostPeerEvent &&move) = delete;

        operator pawkit_net_host_event () {
            return static_cast<pawkit_net_host_event>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetHostPeerEvent *From(pawkit_net_host_event event) {
            return static_cast<NetHostPeerEvent *>(event);
        }

        pawkit_net_host_event_type_t GetType() {
            return pawkit_net_host_event_get_type(*this);
        }

        pawkit_usize GetPeerId() {
            return pawkit_net_host_event_get_peer_id(*this);
        }

        std::span<pawkit_u8 const> GetData() {
            if (GetType() != PAWKIT_NET_HOST_EVENT_TYPE_PACKET_RECEIVED)
                return {};

            pawkit_usize size;

            pawkit_u8 const *data = pawkit_net_host_event_get_data(*this, &size);

            return {data, size};
        }
    };

    struct NetHostPeer {
        ~NetHostPeer() {
            pawkit_net_host_event_free(*this);
        };

        NetHostPeer() = delete;
        NetHostPeer(NetHostPeer const &copy) = delete;
        NetHostPeer(NetHostPeer &&move) = delete;

        operator pawkit_net_host_peer_t () {
            return static_cast<pawkit_net_host_peer_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetHostPeer *From(pawkit_net_host_peer_t event) {
            return static_cast<NetHostPeer *>(event);
        }

        static NetHostPeer *New(std::string_view serverUrl, pawkit_u32 gameId, bool requestProxy) {
            return From(pawkit_net_host_peer_create(serverUrl.data(), serverUrl.size(), gameId, requestProxy));
        }

        inline void SendPacket(pawkit_usize peerId, pawkit_u8 *data, pawkit_usize size) {
            pawkit_net_host_peer_send_packet(*this, peerId, data, size);
        }

        inline void SendPacket(pawkit_usize peerId, std::span<pawkit_u8> data) {
            SendPacket(peerId, data.data(), data.size());
        }

        inline NetHostPeerEvent *PollEvent() {
            return NetHostPeerEvent::From(pawkit_net_host_peer_poll_event(*this));
        }

        inline std::string GetHostId() {
            pawkit_usize size;
            char const *cstr = pawkit_net_host_peer_get_host_id(*this, &size);

            std::string str {cstr, cstr + size};

            pawkit_free_string(cstr, size);

            return str;
        }
    };

    struct NetClientPeerEvent {
        ~NetClientPeerEvent() {
            pawkit_net_host_event_free(*this);
        };

        NetClientPeerEvent() = delete;
        NetClientPeerEvent(NetClientPeerEvent const &copy) = delete;
        NetClientPeerEvent(NetClientPeerEvent &&move) = delete;

        operator pawkit_net_client_event_t () {
            return static_cast<pawkit_net_client_event_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetClientPeerEvent *From(pawkit_net_client_event_t event) {
            return static_cast<NetClientPeerEvent *>(event);
        }

        pawkit_net_client_event_type_t GetType() {
            return pawkit_net_client_event_get_type(*this);
        }

        std::span<pawkit_u8 const> GetData() {
            if (GetType() != PAWKIT_NET_CLIENT_EVENT_TYPE_PACKET_RECEIVED)
                return {};

            pawkit_usize size;
            pawkit_u8 const *data = pawkit_net_client_event_get_data(*this, &size);

            return {data, size};
        }
    };

    struct NetClientPeer {
        ~NetClientPeer() {
            pawkit_net_host_event_free(*this);
        };

        NetClientPeer() = delete;
        NetClientPeer(NetClientPeer const &copy) = delete;
        NetClientPeer(NetClientPeer &&move) = delete;

        operator pawkit_net_client_peer_t () {
            return static_cast<pawkit_net_client_peer_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetClientPeer *From(pawkit_net_client_peer_t event) {
            return static_cast<NetClientPeer *>(event);
        }

        static NetClientPeer *New(std::string_view hostId, pawkit_u32 gameId) {
            return From(pawkit_net_client_peer_create(hostId.data(), hostId.size(), gameId));
        }

        inline void SendPacket(pawkit_u8 *data, pawkit_usize size) {
            pawkit_net_client_peer_send_packet(*this, data, size);
        }

        inline void SendPacket(std::span<pawkit_u8> data) {
            SendPacket(data.data(), data.size());
        }

        inline NetClientPeerEvent *PollEvent() {
            return NetClientPeerEvent::From(pawkit_net_client_peer_poll_event(*this));
        }
    };
}

#endif
