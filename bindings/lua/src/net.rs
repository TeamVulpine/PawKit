use mlua::prelude::*;
use pawkit_net::{NetClientPeerEvent, NetHostPeerEvent, SimpleNetClientPeer, SimpleNetHostPeer};
use pawkit_net_signaling::model::HostId;

pub(crate) fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("host", lua.create_function(host)?)?;

    {
        let host_events = lua.create_table()?;
        host_events.set("peer_connected", LuaNetHostPeerEvent::PEER_CONNECTED)?;
        host_events.set("peer_disconnected", LuaNetHostPeerEvent::PEER_DISCONNECTED)?;
        host_events.set("packet_recieved", LuaNetHostPeerEvent::PACKET_RECEIVED)?;
        host_events.set("host_id_updated", LuaNetHostPeerEvent::HOST_ID_UPDATED)?;

        exports.set("host_events", host_events)?;
    }

    exports.set("connect", lua.create_function(connect)?)?;

    {
        let client_events = lua.create_table()?;
        client_events.set("connected", LuaNetClientPeerEvent::CONNECTED)?;
        client_events.set("disconnected", LuaNetClientPeerEvent::DISCONNECTED)?;
        client_events.set(
            "connection_failed",
            LuaNetClientPeerEvent::CONNECTION_FAILED,
        )?;
        client_events.set("packet_received", LuaNetClientPeerEvent::PACKET_RECEIVED)?;
        exports.set("client_events", client_events)?;
    }

    return Ok(exports);
}

fn host(_lua: &Lua, args: (String, u32)) -> LuaResult<LuaNetHostPeer> {
    return Ok(LuaNetHostPeer {
        peer: SimpleNetHostPeer::create(&args.0, args.1),
    });
}

fn connect(_lua: &Lua, args: (u32, String)) -> LuaResult<LuaNetClientPeer> {
    let game_id = args.0;
    let host_id_str = args.1;

    let host_id: HostId = host_id_str
        .parse()
        .map_err(|e| LuaError::external(format!("Invalid host ID: {}", e)))?;

    let peer = SimpleNetClientPeer::create(game_id, host_id);

    Ok(LuaNetClientPeer { peer })
}

pub struct LuaNetHostPeer {
    peer: SimpleNetHostPeer,
}

impl LuaNetHostPeer {
    fn lua_send_packet(_lua: &Lua, this: &Self, args: (usize, LuaString)) -> LuaResult<()> {
        this.peer.send_packet(args.0, &args.1.as_bytes());

        return Ok(());
    }

    fn lua_shutdown(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<()> {
        this.peer.shutdown();

        return Ok(());
    }

    fn lua_next_event(
        _lua: &Lua,
        this: &mut Self,
        _args: (),
    ) -> LuaResult<Option<LuaNetHostPeerEvent>> {
        let Some(evt) = this.peer.next_event() else {
            return Ok(None);
        };

        return Ok(Some(LuaNetHostPeerEvent { evt }));
    }

    fn lua_get_host_id(_lua: &Lua, this: &mut Self, _args: ()) -> LuaResult<String> {
        return Ok(this.peer.get_host_id().to_string());
    }
}

impl LuaUserData for LuaNetHostPeer {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("send_packet", Self::lua_send_packet);
        methods.add_method("shutdown", Self::lua_shutdown);
        methods.add_method_mut("next_event", Self::lua_next_event);
        methods.add_method_mut("get_host_id", Self::lua_get_host_id);
    }
}

struct LuaNetHostPeerEvent {
    evt: NetHostPeerEvent,
}

impl LuaNetHostPeerEvent {
    const PEER_CONNECTED: i32 = 0;
    const PEER_DISCONNECTED: i32 = 1;
    const PACKET_RECEIVED: i32 = 2;
    const HOST_ID_UPDATED: i32 = 3;

    fn lua_get_type(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<i32> {
        return Ok(match this.evt {
            NetHostPeerEvent::PeerConnected { peer_id: _ } => Self::PEER_CONNECTED,
            NetHostPeerEvent::PeerDisconnected { peer_id: _ } => Self::PEER_DISCONNECTED,
            NetHostPeerEvent::PacketReceived {
                peer_id: _,
                data: _,
            } => Self::PACKET_RECEIVED,
            NetHostPeerEvent::HostIdUpdated => Self::HOST_ID_UPDATED,
        });
    }

    fn lua_get_peer_id(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<Option<usize>> {
        return Ok(match this.evt {
            NetHostPeerEvent::PeerConnected { peer_id } => Some(peer_id),
            NetHostPeerEvent::PeerDisconnected { peer_id } => Some(peer_id),
            NetHostPeerEvent::PacketReceived { peer_id, data: _ } => Some(peer_id),
            NetHostPeerEvent::HostIdUpdated => None,
        });
    }

    fn lua_get_data(lua: &Lua, this: &Self, _args: ()) -> LuaResult<Option<LuaString>> {
        let NetHostPeerEvent::PacketReceived { peer_id: _, data } = &this.evt else {
            return Ok(None);
        };

        return Ok(Some(lua.create_string(data)?));
    }
}

impl LuaUserData for LuaNetHostPeerEvent {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_type", Self::lua_get_type);
        methods.add_method("get_peer_id", Self::lua_get_peer_id);
        methods.add_method("get_data", Self::lua_get_data);
    }
}

pub struct LuaNetClientPeer {
    peer: SimpleNetClientPeer,
}

impl LuaNetClientPeer {
    fn lua_send_packet(_lua: &Lua, this: &Self, data: LuaString) -> LuaResult<()> {
        this.peer.send_packet(&data.as_bytes());

        return Ok(());
    }

    fn lua_next_event(
        _lua: &Lua,
        this: &mut Self,
        _args: (),
    ) -> LuaResult<Option<LuaNetClientPeerEvent>> {
        let Some(evt) = this.peer.next_event() else {
            return Ok(None);
        };

        return Ok(Some(LuaNetClientPeerEvent { evt }));
    }

    fn lua_disconnect(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<()> {
        this.peer.disconnect();

        return Ok(());
    }
}

impl LuaUserData for LuaNetClientPeer {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("send_packet", Self::lua_send_packet);
        methods.add_method_mut("next_event", Self::lua_next_event);
        methods.add_method("disconnect", Self::lua_disconnect);
    }
}

struct LuaNetClientPeerEvent {
    evt: NetClientPeerEvent,
}

impl LuaNetClientPeerEvent {
    const CONNECTED: i32 = 0;
    const DISCONNECTED: i32 = 1;
    const CONNECTION_FAILED: i32 = 2;
    const PACKET_RECEIVED: i32 = 3;

    fn lua_get_type(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<i32> {
        return Ok(match this.evt {
            NetClientPeerEvent::Connected => Self::CONNECTED,
            NetClientPeerEvent::Disconnected => Self::DISCONNECTED,
            NetClientPeerEvent::ConnectionFailed => Self::CONNECTION_FAILED,
            NetClientPeerEvent::PacketReceived { data: _ } => Self::PACKET_RECEIVED,
        });
    }

    fn lua_get_data(lua: &Lua, this: &Self, _args: ()) -> LuaResult<Option<LuaString>> {
        let NetClientPeerEvent::PacketReceived { data } = &this.evt else {
            return Ok(None);
        };

        return Ok(Some(lua.create_string(data)?));
    }
}

impl LuaUserData for LuaNetClientPeerEvent {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("get_type", Self::lua_get_type);
        methods.add_method("get_data", Self::lua_get_data);
    }
}
