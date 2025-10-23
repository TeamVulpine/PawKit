#pragma once

#include "net.h"

#include <string_view>
#include <string>
#include <span>

namespace PawKit::Networking {
    struct NetHostPeerEvent final {
        enum struct Type : pawkit_net_host_event_type_t {
            PeerConnected,
            PeerDisconnected,
            PacketReceived,
            HostIdUpdated,
        };

        ~NetHostPeerEvent() {
            pawkit_net_host_event_free(*this);
        };

        NetHostPeerEvent() = delete;
        NetHostPeerEvent(NetHostPeerEvent const &copy) = delete;
        NetHostPeerEvent(NetHostPeerEvent &&move) = delete;

        operator pawkit_net_host_event_t () {
            return reinterpret_cast<pawkit_net_host_event_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetHostPeerEvent *From(pawkit_net_host_event_t event) {
            return reinterpret_cast<NetHostPeerEvent *>(event);
        }

        Type GetType() {
            return Type(pawkit_net_host_event_get_type(*this));
        }

        pawkit_usize GetPeerId() {
            return pawkit_net_host_event_get_peer_id(*this);
        }

        std::span<pawkit_u8 const> GetData() {
            if (GetType() != Type::PacketReceived)
                return {};

            pawkit_usize size;

            pawkit_u8 const *data = pawkit_net_host_event_get_data(*this, &size);

            return {data, size};
        }
    };

    struct NetHostPeer final {
        ~NetHostPeer() {
            pawkit_net_host_peer_free(*this);
        };

        NetHostPeer() = delete;
        NetHostPeer(NetHostPeer const &copy) = delete;
        NetHostPeer(NetHostPeer &&move) = delete;

        operator pawkit_net_host_peer_t () {
            return reinterpret_cast<pawkit_net_host_peer_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetHostPeer *From(pawkit_net_host_peer_t event) {
            return reinterpret_cast<NetHostPeer *>(event);
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

    struct NetClientPeerEvent final {
        enum struct Type : pawkit_net_client_event_type_t {
            Connected,
            Disconnected,
            ConnectionFailed,
            PacketReceived,
        };

        ~NetClientPeerEvent() {
            pawkit_net_client_event_free(*this);
        };

        NetClientPeerEvent() = delete;
        NetClientPeerEvent(NetClientPeerEvent const &copy) = delete;
        NetClientPeerEvent(NetClientPeerEvent &&move) = delete;

        operator pawkit_net_client_event_t () {
            return reinterpret_cast<pawkit_net_client_event_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetClientPeerEvent *From(pawkit_net_client_event_t event) {
            return reinterpret_cast<NetClientPeerEvent *>(event);
        }

        Type GetType() {
            return Type(pawkit_net_client_event_get_type(*this));
        }

        std::span<pawkit_u8 const> GetData() {
            if (GetType() != Type::PacketReceived)
                return {};

            pawkit_usize size;
            pawkit_u8 const *data = pawkit_net_client_event_get_data(*this, &size);

            return {data, size};
        }
    };

    struct NetClientPeer final {
        ~NetClientPeer() {
            pawkit_net_client_peer_free(*this);
        };

        NetClientPeer() = delete;
        NetClientPeer(NetClientPeer const &copy) = delete;
        NetClientPeer(NetClientPeer &&move) = delete;

        operator pawkit_net_client_peer_t () {
            return reinterpret_cast<pawkit_net_client_peer_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static NetClientPeer *From(pawkit_net_client_peer_t event) {
            return reinterpret_cast<NetClientPeer *>(event);
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
