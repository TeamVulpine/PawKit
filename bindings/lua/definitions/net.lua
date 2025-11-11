---@meta

---@class pawkit.net
local net = {}

---@class ChannelConfiguration
---@field ordered boolean
---@field retries integer|nil

---@param server_url string
---@param game_id integer
---@param request_proxy boolean|nil
---@param channels ChannelConfiguration[]
---@return pawkit.net.NetHostPeer
function net.host(server_url, game_id, request_proxy, channels)
end

---@param game_id integer
---@param host_id string
---@return pawkit.net.NetClientPeer
function net.connect(game_id, host_id)
end

---@class pawkit.net.host_events
net.host_events = {
    PeerConnected = 0,
    PeerDisconnected = 1,
    PacketRecieved = 2,
    HostIdUpdated = 3
}

---@class pawkit.net.client_events
net.client_events = {
    Connected = 0,
    Disconnected = 1,
    ConnectionFailed = 2,
    PacketRecieved = 3
}

---@class pawkit.net.NetHostPeer
local NetHostPeer = {}

---@param peer_id integer
---@param channel integer
---@param data string
function NetHostPeer:send_packet(peer_id, channel, data)
end

function NetHostPeer:shutdown()
end

---@return pawkit.net.NetHostPeerEvent|nil
function NetHostPeer:next_event()
end

---@return string
function NetHostPeer:get_host_id()
end

---@class pawkit.net.NetHostPeerEvent
local NetHostPeerEvent = {}

---@return integer
function NetHostPeerEvent:get_type()
end

---@return integer|nil
function NetHostPeerEvent:get_peer_id()
end

---@return string|nil
function NetHostPeerEvent:get_data()
end

---@class pawkit.net.NetClientPeer
local NetClientPeer = {}

---@param data string
---@param channel integer
function NetClientPeer:send_packet(channel, data)
end

function NetClientPeer:disconnect()
end

---@return pawkit.net.NetClientPeerEvent|nil
function NetClientPeer:next_event()
end

---@class pawkit.net.NetClientPeerEvent
local NetClientPeerEvent = {}

---@return integer
function NetClientPeerEvent:get_type()
end

---@return string|nil
function NetClientPeerEvent:get_data()
end

return net
