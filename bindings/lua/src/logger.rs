use mlua::prelude::*;

pub(super) fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("print_to_console", lua.create_function(print_to_console)?)?;
    exports.set("print_to_logfile", lua.create_function(print_to_logfile)?)?;

    exports.set("info", lua.create_function(info)?)?;
    exports.set("debug", lua.create_function(debug)?)?;
    exports.set("warn", lua.create_function(warn)?)?;
    exports.set("error", lua.create_function(error)?)?;
    exports.set("fatal", lua.create_function(fatal)?)?;

    return Ok(exports);
}

fn print_to_console(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::print_to_console(&args.0);

    return Ok(());
}

fn print_to_logfile(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::print_to_logfile(&args.0);

    return Ok(());
}

fn info(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::info(&args.0);

    return Ok(());
}

fn debug(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::debug(&args.0);

    return Ok(());
}

fn warn(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::warn(&args.0);

    return Ok(());
}

fn error(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::error(&args.0);

    return Ok(());
}

fn fatal(_lua: &Lua, args: (String,)) -> LuaResult<()> {
    pawkit_logger::fatal(&args.0);

    return Ok(());
}
