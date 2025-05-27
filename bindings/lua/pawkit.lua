---@meta

---@class pawkit
local pawkit = {}

---@class pawkit.logger
local logger = {}

---@param message string
function logger.print_to_console(message) end

---@param message string
function logger.print_to_logfile(message) end

---@param message string
function logger.info(message) end

---@param message string
function logger.debug(message) end

---@param message string
function logger.warn(message) end

---@param message string
function logger.error(message) end

---@param message string
function logger.fatal(message) end

pawkit.logger = logger

---@class pawkit.net
local net = {}

---@param server_url string
---@param game_id integer
---@param request_proxy boolean|nil
---@return pawkit.net.NetHostPeer
function net.host(server_url, game_id, request_proxy) end

---@param game_id integer
---@param host_id string
---@return pawkit.net.NetClientPeer
function net.connect(game_id, host_id) end

---@class pawkit.net.host_events
---@field peer_connected integer
---@field peer_disconnected integer
---@field packet_recieved integer
---@field host_id_updated integer
net.host_events = {
  peer_connected = 0,
  peer_disconnected = 1,
  packet_recieved = 2,
  host_id_updated = 3
}

---@class pawkit.net.client_events
---@field connected integer
---@field disconnected integer
---@field connection_failed integer
---@field packet_received integer
net.client_events = {
  connected = 0,
  disconnected = 1,
  connection_failed = 2,
  packet_received = 3
}

---@class pawkit.net.NetHostPeer
local NetHostPeer = {}

---@param peer_id integer
---@param data string
function NetHostPeer:send_packet(peer_id, data) end

function NetHostPeer:shutdown() end

---@return pawkit.net.NetHostPeerEvent|nil
function NetHostPeer:next_event() end

---@return string
function NetHostPeer:get_host_id() end

---@class pawkit.net.NetHostPeerEvent
local NetHostPeerEvent = {}

---@return integer
function NetHostPeerEvent:get_type() end

---@return integer|nil
function NetHostPeerEvent:get_peer_id() end

---@return string|nil
function NetHostPeerEvent:get_data() end

---@class pawkit.net.NetClientPeer
local NetClientPeer = {}

---@param data string
function NetClientPeer:send_packet(data) end

function NetClientPeer:disconnect() end

---@return pawkit.net.NetClientPeerEvent|nil
function NetClientPeer:next_event() end

---@class pawkit.net.NetClientPeerEvent
local NetClientPeerEvent = {}

---@return integer
function NetClientPeerEvent:get_type() end

---@return string|nil
function NetClientPeerEvent:get_data() end

pawkit.net = net

return pawkit
