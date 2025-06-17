---@meta

---@class pawkit.input
local input = {}

---@class pawkit.input.keyboard_buttons
input.keyboard_buttons = {
    A = "A", B = "B", C = "C", D = "D", E = "E", F = "F", G = "G", H = "H", I = "I", J = "J", K = "K", L = "L", M = "M",
    N = "N", O = "O", P = "P", Q = "Q", R = "R", S = "S", T = "T", U = "U", V = "V", W = "W", X = "X", Y = "Y", Z = "Z",

    Number0 = "Number0", Number1 = "Number1", Number2 = "Number2", Number3 = "Number3",
    Number4 = "Number4", Number5 = "Number5", Number6 = "Number6", Number7 = "Number7",
    Number8 = "Number8", Number9 = "Number9",

    Up = "Up", Down = "Down", Left = "Left", Right = "Right",

    Tilde = "Tilde", Grave = "Grave", Minus = "Minus", Plus = "Plus",
    LeftBracket = "LeftBracket", RightBracket = "RightBracket",
    Semicolon = "Semicolon", Quote = "Quote", Comma = "Comma",
    Period = "Period", Slash = "Slash", BackSlash = "BackSlash",

    LeftShift = "LeftShift", RightShift = "RightShift",
    LeftControl = "LeftControl", RightControl = "RightControl",
    LeftAlt = "LeftAlt", RightAlt = "RightAlt",
    LeftMeta = "LeftMeta", RightMeta = "RightMeta",

    Menu = "Menu", Enter = "Enter", Escape = "Escape", Space = "Space",
    Tab = "Tab", Backspace = "Backspace", Insert = "Insert",
    Delete = "Delete", PageUp = "Up", PageDown = "Down",
    Home = "Home", End = "End", CapsLock = "CapsLock", ScrollLock = "ScrollLock",
    PrintScreen = "PrintScreen", Pause = "Pause", NumLock = "NumLock",
    Clear = "Clear", Sleep = "Sleep",

    Numpad0 = "Numpad0", Numpad1 = "Numpad1", Numpad2 = "Numpad2", Numpad3 = "Numpad3",
    Numpad4 = "Numpad4", Numpad5 = "Numpad5", Numpad6 = "Numpad6", Numpad7 = "Numpad7",
    Numpad8 = "Numpad8", Numpad9 = "Numpad9", NumpadDivide = "NumpadDivide",
    NumpadMultiply = "NumpadMultiply", NumpadMinus = "Minus",
    NumpadPlus = "Plus", NumpadDecimal = "NumpadDecimal",
    NumpadPeriod = "Period", NumpadEnter = "Enter",

    F1 = "F1", F2 = "F2", F3 = "F3", F4 = "F4", F5 = "F5", F6 = "F6", F7 = "F7", F8 = "F8", F9 = "F9",
    F10 = "F10", F11 = "F11", F12 = "F12", F13 = "F13", F14 = "F14", F15 = "F15", F16 = "F16",
    F17 = "F17", F18 = "F18", F19 = "F19", F20 = "F20", F21 = "F21", F22 = "F22", F23 = "F23",
    F24 = "F24",
}

---@class pawkit.input.mouse_buttons
input.mouse_buttons = {
    Left = "Left",
    Right = "Right",
    Middle = "Middle",
    Side1 = "Side1",
    Side2 = "Side2",
}

---@class pawkit.input.gamepad_buttons
input.gamepad_buttons = {
    South = "South",
    East = "East",
    West = "West",
    North = "North",
    Back = "Back",
    Guide = "Guide",
    Start = "Start",
    LeftStick = "LeftStick",
    RightStick = "RightStick",
    LeftShoulder = "LeftShoulder",
    RightShoulder = "RightShoulder",
    DpadUp = "Up",
    DpadDown = "Down",
    DpadLeft = "Left",
    DpadRight = "Right",
    Misc1 = "Misc1",
    RightPaddle1 = "RightPaddle1",
    LeftPaddle1 = "LeftPaddle1",
    RightPaddle2 = "RightPaddle2",
    LeftPaddle2 = "LeftPaddle2",
    Touchpad = "Touchpad",
    Misc2 = "Misc2",
    Misc3 = "Misc3",
    Misc4 = "Misc4",
    Misc5 = "Misc5",
    Misc6 = "Misc6",
}

---@class pawkit.input.mouse_axes
input.mouse_axes = {
    DeltaX = "X",
    DeltaY = "Y",
    WheelX = "X",
    WheelY = "Y",
}

---@class pawkit.input.gamepad_axes
input.gamepad_axes = {
    LeftX = "X",
    LeftY = "Y",
    RightX = "X",
    RightY = "Y",
    LeftTrigger = "LeftTrigger",
    RightTrigger = "RightTrigger",
}

---@class pawkit.input.families
input.families = {
    Keyboard = "Keyboard",
    Mouse = "Mouse",
    Gamepad = "Gamepad",
}

---@alias KeyboardButton
---| "A"
---| "B"
---| "C"
---| "D"
---| "E"
---| "F"
---| "G"
---| "H"
---| "I"
---| "J"
---| "K"
---| "L"
---| "M"
---| "N"
---| "O"
---| "P"
---| "Q"
---| "R"
---| "S"
---| "T"
---| "U"
---| "V"
---| "W"
---| "X"
---| "Y"
---| "Z"
---| "Number0"
---| "Number1"
---| "Number2"
---| "Number3"
---| "Number4"
---| "Number5"
---| "Number6"
---| "Number7"
---| "Number8"
---| "Number9"
---| "Up"
---| "Down"
---| "Left"
---| "Right"
---| "Tilde"
---| "Grave"
---| "Minus"
---| "Plus"
---| "LeftBracket"
---| "RightBracket"
---| "Semicolon"
---| "Quote"
---| "Comma"
---| "Period"
---| "Slash"
---| "BackSlash"
---| "LeftShift"
---| "RightShift"
---| "LeftControl"
---| "RightControl"
---| "LeftAlt"
---| "RightAlt"
---| "LeftMeta"
---| "RightMeta"
---| "Menu"
---| "Enter"
---| "Escape"
---| "Space"
---| "Tab"
---| "Backspace"
---| "Insert"
---| "Delete"
---| "PageUp"
---| "PageDown"
---| "Home"
---| "End"
---| "CapsLock"
---| "ScrollLock"
---| "PrintScreen"
---| "Pause"
---| "NumLock"
---| "Clear"
---| "Sleep"
---| "Numpad0"
---| "Numpad1"
---| "Numpad2"
---| "Numpad3"
---| "Numpad4"
---| "Numpad5"
---| "Numpad6"
---| "Numpad7"
---| "Numpad8"
---| "Numpad9"
---| "NumpadDivide"
---| "NumpadMultiply"
---| "NumpadMinus"
---| "NumpadPlus"
---| "NumpadDecimal"
---| "NumpadPeriod"
---| "NumpadEnter"
---| "F1"
---| "F2"
---| "F3"
---| "F4"
---| "F5"
---| "F6"
---| "F7"
---| "F8"
---| "F9"
---| "F10"
---| "F11"
---| "F12"
---| "F13"
---| "F14"
---| "F15"
---| "F16"
---| "F17"
---| "F18"
---| "F19"
---| "F20"
---| "F21"
---| "F22"
---| "F23"
---| "F24"

---@alias MouseButton
---| "Left"
---| "Right"
---| "Middle"
---| "Side1"
---| "Side2"

---@alias GamepadButton
---| "South"
---| "East"
---| "West"
---| "North"
---| "Back"
---| "Guide"
---| "Start"
---| "LeftStick"
---| "RightStick"
---| "LeftShoulder"
---| "RightShoulder"
---| "DpadUp"
---| "DpadDown"
---| "DpadLeft"
---| "DpadRight"
---| "Misc1"
---| "RightPaddle1"
---| "LeftPaddle1"
---| "RightPaddle2"
---| "LeftPaddle2"
---| "Touchpad"
---| "Misc2"
---| "Misc3"
---| "Misc4"
---| "Misc5"
---| "Misc6"

---@alias MouseAxis
---| "DeltaX"
---| "DeltaY"
---| "WheelX"
---| "WheelY"

---@alias GamepadAxis
---| "LeftX"
---| "LeftY"
---| "RightX"
---| "RightY"
---| "LeftTrigger"
---| "RightTrigger"

---@alias InputFamily
---| "Keyboard"
---| "Mouse"
---| "Gamepad"

---@class BoundButtonAnalogDataKeyboard
---@field axis nil
---@field threshold number

---@class BoundButtonAnalogDataMouse
---@field axis MouseAxis
---@field threshold number

---@class BoundButtonAnalogDataGamepad
---@field axis GamepadAxis
---@field threshold number

---@class BoundButtonKeyboard
---@field Digital? KeyboardButton
---@field Analog? BoundButtonAnalogDataKeyboard

---@class BoundButtonMouse
---@field Digital? MouseButton
---@field Analog? BoundButtonAnalogDataMouse

---@class BoundButtonGamepad
---@field Digital? GamepadButton
---@field Analog? BoundButtonAnalogDataGamepad

---@class DigitalBinding
---@field Keyboard? BoundButtonKeyboard
---@field Mouse? BoundButtonMouse
---@field Gamepad? BoundButtonGamepad

---@class BoundAxisKeyboard
---@field Analog? KeyboardAxis
---@field Digital? KeyboardButton
---@field MultiDigitalNegative? KeyboardButton
---@field MultiDigitalPositive? KeyboardButton

---@class BoundAxisMouse
---@field Analog? MouseAxis
---@field Digital? MouseButton
---@field MultiDigitalNegative? MouseButton
---@field MultiDigitalPositive? MouseButton

---@class BoundAxisGamepad
---@field Analog? GamepadAxis
---@field Digital? GamepadButton
---@field MultiDigitalNegative? GamepadButton
---@field MultiDigitalPositive? GamepadButton

---@class AnalogBindingKindKeyboard
---@field axis BoundAxisKeyboard

---@class AnalogBindingKindMouse
---@field axis BoundAxisMouse

---@class AnalogBindingKindGamepad
---@field axis BoundAxisGamepad

---@class AnalogBinding
---@field Keyboard? AnalogBindingKindKeyboard
---@field Mouse? AnalogBindingKindMouse
---@field Gamepad? AnalogBindingKindGamepad
---@field deadzone number
---@field scale number

---@class VectorBindingKindKeyboard
---@field x BoundAxisKeyboard
---@field y BoundAxisKeyboard

---@class VectorBindingKindMouse
---@field x BoundAxisMouse
---@field y BoundAxisMouse

---@class VectorBindingKindGamepad
---@field x BoundAxisGamepad
---@field y BoundAxisGamepad

---@class VectorBinding
---@field Keyboard? VectorBindingKindKeyboard
---@field Mouse? VectorBindingKindMouse
---@field Gamepad? VectorBindingKindGamepad
---@field deadzone number
---@field scale [number, number]

---@alias RawDeviceId integer
---@alias DeviceId integer

---@class InputManager
local InputManager = {}

---@param name string
---@param bindings DigitalBinding[]
function InputManager:register_digital_binding(name, bindings) end

---@param name string
---@param bindings AnalogBinding[]
---@return nil
function InputManager:register_analog_binding(name, bindings) end

---@param name string
---@param bindings VectorBinding[]
---@return nil
function InputManager:register_vector_binding(name, bindings) end

---@return nil
function InputManager:lock_bindings() end

---@param family InputFamily
---@param id RawDeviceId
---@return DeviceId
function InputManager:device_connected(family, id) end

---@param family InputFamily
---@param id DeviceId
---@return nil
function InputManager:device_disconnected(family, id) end

---@param family InputFamily
---@param device DeviceId
---@param button KeyboardButton | MouseButton | GamepadButton
---@param value boolean
function InputManager:set_button(family, device, button, value) end

---@param family InputFamily
---@param device DeviceId
---@param button MouseAxis | GamepadAxis
---@param value number
function InputManager:set_axis(family, device, button, value) end

return input
