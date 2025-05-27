#![cfg(not(target_arch = "wasm32"))]

use mlua::prelude::*;

mod logger;
mod net;

#[mlua::lua_module]
fn pawkit(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("logger", logger::init(lua)?)?;
    exports.set("net", net::init(lua)?)?;

    return Ok(exports);
}
