use serde::{Deserialize, Serialize};

#[rustfmt::skip]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum KeyboardButton {
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
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Side1,
    Side2,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum GamepadButton {
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
}
