#![feature(decl_macro)]
#![cfg(not(target_arch = "wasm32"))]

use mlua::prelude::*;

mod fs;
mod input;
mod logger;
mod net;

macro lua_enum {
    ($fname:ident {$($name:ident $(= $value:expr)?),* $(,)?}) => {
        #[allow(unused_assignments)]
        fn $fname(lua: &Lua) -> LuaResult<LuaTable> {
            let table = lua.create_table()?;

            let mut index = 0;

            $(
                $(index = $value;)?
                table.set(stringify!($name), index)?;
                index += 1;
            )*

            return Ok(table);
        }
    },

    ($fname:ident : str {$($name:ident),* $(,)?}) => {
        fn $fname(lua: &Lua) -> LuaResult<LuaTable> {
            let table = lua.create_table()?;

            $(
                table.set(stringify!($name), stringify!($name))?;
            )*

            return Ok(table);
        }
    }
}

#[mlua::lua_module]
pub fn pawkit(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("logger", logger::init(lua)?)?;
    exports.set("net", net::init(lua)?)?;
    exports.set("input", input::init(lua)?)?;
    exports.set("fs", fs::init(lua)?)?;

    return Ok(exports);
}
