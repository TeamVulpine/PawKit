use mlua::prelude::*;
use pawkit_input::{
    bindings::{
        axis::{GamepadAxis, MouseAxis},
        button::{GamepadButton, KeyboardButton, MouseButton},
    },
    manager::InputFamily,
    InputManager,
};

use crate::lua_enum;

lua_enum!(keyboard_buttons : str {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    Number0, Number1, Number2, Number3,
    Number4, Number5, Number6, Number7,
    Number8, Number9,

    Up, Down, Left, Right,

    Tilde, Grave, Minus, Plus,
    LeftBracket, RightBracket,
    Semicolon, Quote, Comma,
    Period, Slash, BackSlash,

    LeftShift, RightShift,
    LeftControl, RightControl,
    LeftAlt, RightAlt,
    LeftMeta, RightMeta,

    Menu, Enter, Escape, Space,
    Tab, Backspace, Insert,
    Delete, PageUp, PageDown,
    Home, End, CapsLock, ScrollLock,
    PrintScreen, Pause, NumLock,
    Clear, Sleep,

    Numpad0, Numpad1, Numpad2, Numpad3,
    Numpad4, Numpad5, Numpad6, Numpad7,
    Numpad8, Numpad9, NumpadDivide,
    NumpadMultiply, NumpadMinus,
    NumpadPlus, NumpadDecimal,
    NumpadPeriod, NumpadEnter,

    F1, F2, F3, F4, F5, F6, F7, F8, F9,
    F10, F11, F12, F13, F14, F15, F16,
    F17, F18, F19, F20, F21, F22, F23,
    F24,
});

lua_enum!(mouse_buttons : str {
    Left,
    Right,
    Middle,
    Side1,
    Side2,
});

lua_enum!(gamepad_buttons : str {
    South,
    East,
    West,
    North,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    Misc1,
    RightPaddle1,
    LeftPaddle1,
    RightPaddle2,
    LeftPaddle2,
    Touchpad,
    Misc2,
    Misc3,
    Misc4,
    Misc5,
    Misc6,
});

lua_enum!(mouse_axes : str {
    DeltaX,
    DeltaY,
    WheelX,
    WheelY,
});

lua_enum!(gamepad_axes : str {
    LeftX,
    LeftY,
    RightX,
    RightY,
    LeftTrigger,
    RightTrigger,
});

lua_enum!(input_family : str {
    Keyboard,
    Mouse,
    Gamepad,
});

pub(super) fn init(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    exports.set("keyboard_buttons", keyboard_buttons(lua)?)?;
    exports.set("mouse_buttons", mouse_buttons(lua)?)?;
    exports.set("gamepad_buttons", gamepad_buttons(lua)?)?;

    exports.set("mouse_axes", mouse_axes(lua)?)?;
    exports.set("gamepad_axes", gamepad_axes(lua)?)?;

    exports.set("families", input_family(lua)?)?;

    exports.set("create_manager", lua.create_function(LuaInputManager::new)?)?;

    return Ok(exports);
}

struct LuaInputManager {
    manager: InputManager,
}

impl LuaInputManager {
    fn new(_lua: &Lua, _args: ()) -> LuaResult<Self> {
        return Ok(Self {
            manager: InputManager::new(),
        });
    }

    fn register_digital_binding(
        lua: &Lua,
        this: &mut Self,
        args: (String, LuaValue),
    ) -> LuaResult<()> {
        let bindings: Vec<_> = lua.from_value(args.1)?;

        if let Err(err) = this.manager.register_digital(&args.0, &bindings) {
            return Err(LuaError::RuntimeError(err.to_string()));
        }

        return Ok(());
    }

    fn register_analog_binding(
        lua: &Lua,
        this: &mut Self,
        args: (String, LuaValue),
    ) -> LuaResult<()> {
        let bindings: Vec<_> = lua.from_value(args.1)?;

        if let Err(err) = this.manager.register_analog(&args.0, &bindings) {
            return Err(LuaError::RuntimeError(err.to_string()));
        }

        return Ok(());
    }

    fn register_vector_binding(
        lua: &Lua,
        this: &mut Self,
        args: (String, LuaValue),
    ) -> LuaResult<()> {
        let bindings: Vec<_> = lua.from_value(args.1)?;

        if let Err(err) = this.manager.register_vector(&args.0, &bindings) {
            return Err(LuaError::RuntimeError(err.to_string()));
        }

        return Ok(());
    }

    fn lock_bindings(_lua: &Lua, this: &mut Self, _args: ()) -> LuaResult<()> {
        this.manager.lock();

        return Ok(());
    }

    fn device_connected(lua: &Lua, this: &Self, args: (LuaValue, usize)) -> LuaResult<usize> {
        let family = lua.from_value(args.0)?;

        let id = match family {
            InputFamily::Keyboard => &this.manager.devices.keyboard_manager,
            InputFamily::Mouse => &this.manager.devices.mouse_manager,
            InputFamily::Gamepad => &this.manager.devices.gamepad_manager,
        }
        .device_connected(args.1);

        return Ok(id);
    }

    fn device_disconnected(lua: &Lua, this: &Self, args: (LuaValue, usize)) -> LuaResult<()> {
        let family = lua.from_value(args.0)?;

        match family {
            InputFamily::Keyboard => &this.manager.devices.keyboard_manager,
            InputFamily::Mouse => &this.manager.devices.mouse_manager,
            InputFamily::Gamepad => &this.manager.devices.gamepad_manager,
        }
        .device_disconnected(args.1);

        return Ok(());
    }

    fn set_button(
        lua: &Lua,
        this: &Self,
        args: (LuaValue, usize, LuaValue, bool),
    ) -> LuaResult<()> {
        let family = lua.from_value(args.0)?;

        let Some(state) = match family {
            InputFamily::Keyboard => &this.manager.devices.keyboard_manager,
            InputFamily::Mouse => &this.manager.devices.mouse_manager,
            InputFamily::Gamepad => &this.manager.devices.gamepad_manager,
        }
        .get_state(args.1) else {
            return Err(LuaError::RuntimeError("Index does not exist.".into()));
        };

        let index = match family {
            InputFamily::Keyboard => lua.from_value::<KeyboardButton>(args.2)? as usize,
            InputFamily::Mouse => lua.from_value::<MouseButton>(args.2)? as usize,
            InputFamily::Gamepad => lua.from_value::<GamepadButton>(args.2)? as usize,
        };

        state.set_digital(index, args.3);

        return Ok(());
    }

    fn set_axis(lua: &Lua, this: &Self, args: (LuaValue, usize, LuaValue, f32)) -> LuaResult<()> {
        let family = lua.from_value(args.0)?;

        let Some(state) = match family {
            InputFamily::Keyboard => &this.manager.devices.keyboard_manager,
            InputFamily::Mouse => &this.manager.devices.mouse_manager,
            InputFamily::Gamepad => &this.manager.devices.gamepad_manager,
        }
        .get_state(args.1) else {
            return Err(LuaError::RuntimeError("Index does not exist.".into()));
        };

        let index = match family {
            InputFamily::Keyboard => {
                return Err(LuaError::RuntimeError(
                    "Keyboard does not have any axes".into(),
                ))
            }
            InputFamily::Mouse => lua.from_value::<MouseAxis>(args.2)? as usize,
            InputFamily::Gamepad => lua.from_value::<GamepadAxis>(args.2)? as usize,
        };

        state.set_analog(index, args.3);

        return Ok(());
    }

    fn create_handler(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<Option<usize>> {
        return Ok(this.manager.create_handler());
    }

    fn destroy_handler(_lua: &Lua, this: &Self, args: (usize,)) -> LuaResult<()> {
        this.manager.destroy_handler(args.0);

        return Ok(());
    }

    fn update(_lua: &Lua, this: &Self, _args: ()) -> LuaResult<()> {
        this.manager.update();

        return Ok(());
    }

    fn get_frame(lua: &Lua, this: &Self, args: (usize, String)) -> LuaResult<Option<LuaValue>> {
        let Some(frame) = this.manager.get_frame(args.0, &args.1) else {
            return Ok(None);
        };

        return Ok(Some(lua.to_value(&frame)?));
    }

    fn connect_device_to_handler(
        lua: &Lua,
        this: &Self,
        args: (usize, LuaValue, usize),
    ) -> LuaResult<()> {
        let family = lua.from_value(args.1)?;

        this.manager
            .connect_device_to_handler(args.0, family, args.2);

        return Ok(());
    }

    fn disconnect_device_from_handler(
        lua: &Lua,
        this: &Self,
        args: (usize, LuaValue, usize),
    ) -> LuaResult<()> {
        let family = lua.from_value(args.1)?;

        this.manager
            .disconnect_device_from_handler(args.0, family, args.2);

        return Ok(());
    }
}

impl LuaUserData for LuaInputManager {
    fn add_methods<M: LuaUserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("register_digital_binding", Self::register_digital_binding);
        methods.add_method_mut("register_analog_binding", Self::register_analog_binding);
        methods.add_method_mut("register_vector_binding", Self::register_vector_binding);
        methods.add_method_mut("lock_bindings", Self::lock_bindings);

        methods.add_method("device_connected", Self::device_connected);
        methods.add_method("device_disconnected", Self::device_disconnected);

        methods.add_method("set_button", Self::set_button);
        methods.add_method("set_axis", Self::set_axis);

        methods.add_method("create_handler", Self::create_handler);
        methods.add_method("destroy_handler", Self::destroy_handler);

        methods.add_method("update", Self::update);
        methods.add_method("get_frame", Self::get_frame);

        methods.add_method("connect_device_to_handler", Self::connect_device_to_handler);
        methods.add_method(
            "disconnect_device_from_handler",
            Self::disconnect_device_from_handler,
        );
    }
}
