use std::{io::Read, ops::Deref, ptr};

use mlua::prelude::*;
use pawkit_fs::{Vfs, VfsBuffer, VfsError, VfsListUtils};

pub(super) fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set(
        "buffer_from_bytes",
        lua.create_function(LuaVfsBuffer::from_bytes)?,
    )?;

    exports.set("working", lua.create_function(LuaVfs::working)?)?;
    exports.set("zip", lua.create_function(LuaVfs::zip)?)?;

    return Ok(exports);
}

fn to_lua_error(err: VfsError) -> LuaError {
    return LuaError::RuntimeError(format!("{:?}", err));
}

fn modify_in_place<T, F>(x: &mut T, f: F)
where
    F: FnOnce(T) -> T,
{
    unsafe {
        ptr::write(x, f(ptr::read(x)));
    }
}

struct LuaVfsBuffer {
    buf: VfsBuffer,
}

impl LuaVfsBuffer {
    fn from_bytes(_lua: &Lua, args: (LuaString,)) -> LuaResult<Self> {
        return Ok(Self {
            buf: args.0.as_bytes().deref().into(),
        });
    }

    fn read(lua: &Lua, this: &mut Self, _args: ()) -> LuaResult<LuaString> {
        let mut data = vec![];

        this.buf
            .read_to_end(&mut data)
            .map_err(|it| to_lua_error(VfsError::IoError(it)))?;

        return lua.create_string(data);
    }
}

impl LuaUserData for LuaVfsBuffer {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("read", Self::read);
    }
}

struct LuaVfsList {
    list: Box<dyn Iterator<Item = Result<String, VfsError>>>,
}

impl LuaVfsList {
    fn __call(_lua: &Lua, this: &mut Self, _args: ()) -> LuaResult<Option<String>> {
        let Some(value) = this.list.next() else {
            return Ok(None);
        };

        return value.map_err(to_lua_error).map(Some);
    }

    fn with_extension(_lua: &Lua, args: (LuaAnyUserData, String)) -> LuaResult<LuaAnyUserData> {
        let mut this: LuaUserDataRefMut<Self> = args.0.borrow_mut()?;

        let list = &mut this.list;

        modify_in_place(list, |it| Box::new(it.with_extension(args.1)));

        return Ok(args.0);
    }
}

impl LuaUserData for LuaVfsList {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_meta_method_mut(LuaMetaMethod::Call, Self::__call);
        methods.add_function_mut("with_extension", Self::with_extension);
    }
}

struct LuaVfs {
    vfs: Vfs,
}

impl LuaVfs {
    fn working(_lua: &Lua, _args: ()) -> LuaResult<Self> {
        return Ok(Self {
            vfs: Vfs::working().map_err(to_lua_error)?,
        });
    }

    fn zip(_lua: &Lua, args: (LuaAnyUserData,)) -> LuaResult<Self> {
        let buf: LuaVfsBuffer = args.0.take()?;

        return Ok(Self {
            vfs: Vfs::zip(buf.buf).map_err(to_lua_error)?,
        });
    }

    fn subdirectory(_lua: &Lua, this: &Self, args: (String,)) -> LuaResult<Self> {
        return Ok(Self {
            vfs: this.vfs.subdirectory(&args.0).map_err(to_lua_error)?,
        });
    }

    fn open(_lua: &Lua, this: &Self, args: (String,)) -> LuaResult<LuaVfsBuffer> {
        return Ok(LuaVfsBuffer {
            buf: this.vfs.open(&args.0).map_err(to_lua_error)?,
        });
    }

    fn list_subdirectories(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<LuaVfsList> {
        return Ok(LuaVfsList {
            list: Box::new(this.vfs.list_subdirectories().map_err(to_lua_error)?),
        });
    }

    fn list_files(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<LuaVfsList> {
        return Ok(LuaVfsList {
            list: Box::new(this.vfs.list_files().map_err(to_lua_error)?),
        });
    }

    fn list_files_recursive(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<LuaVfsList> {
        return Ok(LuaVfsList {
            list: Box::new(this.vfs.list_files_recursive().map_err(to_lua_error)?),
        });
    }
}

impl LuaUserData for LuaVfs {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("subdirectory", Self::subdirectory);
        methods.add_method("open", Self::open);
        methods.add_method("list_subdirectories", Self::list_subdirectories);
        methods.add_method("list_files", Self::list_files);
        methods.add_method("list_files_recursive", Self::list_files_recursive);
    }
}
