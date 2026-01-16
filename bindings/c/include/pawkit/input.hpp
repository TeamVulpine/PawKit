// Temporarily unavailable.
// If you need a C++ wrapper, open a PR.

// #pragma once

// #include "input.h"
// #include "util.h"

// #include <memory>
// #include <span>
// #include <optional>
// #include <string_view>
// #include <utility>
// #include <variant>

// namespace PawKit::Input {
//     enum struct MouseAxis : pawkit_input_mouseaxis_t {
//         DeltaX,
//         DeltaY,
//         WheelX,
//         WheelY,
//     };

//     enum struct JoyAxis : pawkit_input_joyaxis_t {
//         LeftX,
//         LeftY,
//         RightX,
//         RightY,
//     };

//     enum struct KeyButton : pawkit_input_keybutton_t {
//         A, B, C, D,
//         E, F, G, H,
//         I, J, K, L,
//         M, N, O, P,
//         Q, R, S, T,
//         U, V, W, X,
//         Y, Z,

//         Number0, Number1, Number2,
//         Number3, Number4, Number5,
//         Number6, Number7, Number8,
//         Number9,

//         Up, Down, Left, Right,

//         Tilde, Grave, Minus, Plus,
//         LeftBracket, RightBracket, Semicolon,
//         Quote, Comma, Period,
//         Slash, Backslash,

//         LeftShift, RightShift,
//         LeftControl, RightControl,
//         LeftAlt, RightAlt,
//         LeftMeta, RightMeta,

//         Menu, Enter, Escape, Space,
//         Tab, Backspace, Insert,
//         Delete, PageUp, PageDown,
//         Home, End, CapsLock,
//         ScrollLock, PrintScreen, Pause,
//         NumLock, Clear, Sleep,

//         Numpad0, Numpad1, Numpad2,
//         Numpad3, Numpad4, Numpad5,
//         Numpad6, Numpad7, Numpad8,
//         Numpad9, NumpadDivide,
//         NumpadMultiply, NumpadMinus,
//         NumpadPlus, NumpadDecimal,
//         NumpadPeriod, NumpadEnter,

//         F1, F2, F3, F4,
//         F5, F6, F7, F8,
//         F9, F10, F11, F12,
//         F13, F14, F15, F16,
//         F17, F18, F19, F20,
//         F21, F22, F23, F24,
//     };

//     enum struct MouseButton : pawkit_input_mousebutton_t {
//         Left,
//         Right,
//         Middle,
//         Side1,
//         Side2
//     };

//     enum struct JoyButton : pawkit_input_joybutton_t {
//         South,
//         East,
//         West,
//         North,
//         Back,
//         Guide,
//         Start,
//         LeftStick,
//         RightStick,
//         LeftShoulder,
//         RightShoulder,
//         DpadUp,
//         DpadDown,
//         DpadLeft,
//         DpadRight,
//         Misc1,
//         RightPaddle1,
//         LeftPaddle1,
//         RightPaddle2,
//         LeftPaddle2,
//         Touchpad,
//         Misc2,
//         Misc3,
//         Misc4,
//         Misc5,
//         Misc6,
//     };

//     enum struct Family : pawkit_input_family_t {
//        Key,
//        Mouse,
//        Joy, 
//     };

//     template <typename TAxis>
//     struct AnalogButton final {
//         TAxis axis;
//         pawkit_f32 threshold;
//     };

//     using BoundKeyButton = KeyButton;
//     using BoundMouseButton = std::variant<MouseButton, AnalogButton<MouseAxis>>;
//     using BoundJoyButton = std::variant<JoyButton, AnalogButton<JoyAxis>>;

//     template <typename TButton>
//     struct MultiDigitalAxis final {
//         TButton positive;
//         TButton negative;
//     };

//     using BoundKeyAxis = std::variant<KeyButton, MultiDigitalAxis<KeyButton>>;
//     using BoundMouseAxis = std::variant<MouseAxis, MouseButton, MultiDigitalAxis<MouseButton>>;
//     using BoundJoyAxis = std::variant<JoyAxis, JoyButton, MultiDigitalAxis<JoyButton>>;

//     template <Family TFamily>
//     struct FamilyTraits;

//     template <>
//     struct FamilyTraits<Family::Key> {
//         using Button = KeyButton;
//         using Axis = void;
//         using BoundButton = BoundKeyButton;
//         using BoundAxis = BoundKeyAxis;

//         static pawkit_input_bound_button_t GetDigital(BoundButton button) {
//             return {
//                 .type = PAWKIT_INPUT_BOUND_BUTTON_TYPE_DIGITAL,
//                 .button = pawkit_input_button_t(button),
//             };
//         }

//         static pawkit_input_bound_axis_t GetAnalog(BoundAxis axis) {
//             if (auto value = std::get_if<Button>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_DIGITAL,
//                     .button = pawkit_input_button_t(*value),
//                 };
//             } else if (auto value = std::get_if<MultiDigitalAxis<Button>>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_MULTI_DIGITAL,
//                     .positive = pawkit_input_button_t(value->positive),
//                     .negative = pawkit_input_button_t(value->negative),
//                 };
//             }

//             std::unreachable();
//         }
//     };

//     template <>
//     struct FamilyTraits<Family::Mouse> {
//         using Button = MouseButton;
//         using Axis = MouseAxis;
//         using BoundButton = BoundMouseButton;
//         using BoundAxis = BoundMouseAxis;

//         static pawkit_input_bound_button_t GetDigital(BoundButton button) {
//             if (auto value = std::get_if<Button>(&button)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_BUTTON_TYPE_DIGITAL,
//                     .button = pawkit_input_button_t(*value),
//                 };
//             } else if (auto value = std::get_if<AnalogButton<Axis>>(&button)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_BUTTON_TYPE_ANALOG,
//                     .axis = pawkit_input_axis_t(value->axis),
//                     .threshold = value->threshold,
//                 };
//             }

//             std::unreachable();
//         }

//         static pawkit_input_bound_axis_t GetAnalog(BoundAxis axis) {
//             if (auto value = std::get_if<Axis>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_ANALOG,
//                     .button = pawkit_input_axis_t(*value),
//                 };
//             } else if (auto value = std::get_if<Button>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_DIGITAL,
//                     .button = pawkit_input_button_t(*value),
//                 };
//             } else if (auto value = std::get_if<MultiDigitalAxis<Button>>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_MULTI_DIGITAL,
//                     .positive = pawkit_input_button_t(value->positive),
//                     .negative = pawkit_input_button_t(value->negative),
//                 };
//             }

//             std::unreachable();
//         }
//     };

//     template <>
//     struct FamilyTraits<Family::Joy> {
//         using Button = JoyButton;
//         using Axis = JoyAxis;
//         using BoundButton = BoundJoyButton;
//         using BoundAxis = BoundJoyAxis;

//         static pawkit_input_bound_button_t GetDigital(BoundButton button) {
//             if (auto value = std::get_if<Button>(&button)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_BUTTON_TYPE_DIGITAL,
//                     .button = pawkit_input_button_t(*value),
//                 };
//             } else if (auto value = std::get_if<AnalogButton<Axis>>(&button)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_BUTTON_TYPE_ANALOG,
//                     .axis = pawkit_input_axis_t(value->axis),
//                     .threshold = value->threshold,
//                 };
//             }

//             std::unreachable();
//         }

//         static pawkit_input_bound_axis_t GetAnalog(BoundAxis axis) {
//             if (auto value = std::get_if<Axis>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_ANALOG,
//                     .button = pawkit_input_axis_t(*value),
//                 };
//             } else if (auto value = std::get_if<Button>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_DIGITAL,
//                     .button = pawkit_input_button_t(*value),
//                 };
//             } else if (auto value = std::get_if<MultiDigitalAxis<Button>>(&axis)) {
//                 return {
//                     .type = PAWKIT_INPUT_BOUND_AXIS_TYPE_MULTI_DIGITAL,
//                     .positive = pawkit_input_button_t(value->positive),
//                     .negative = pawkit_input_button_t(value->negative),
//                 };
//             }

//             std::unreachable();
//         }
//     };

//     template <Family TFamily>
//     using BoundButton = FamilyTraits<TFamily>::BoundButton;

//     template <Family TFamily>
//     using BoundAxis = FamilyTraits<TFamily>::BoundAxis;

//     template <Family TFamily>
//     using Button = FamilyTraits<TFamily>::Button;

//     template <Family TFamily>
//     using Axis = FamilyTraits<TFamily>::Axis;

//     template <Family TFamily>
//     struct DigitalBinding final {
//         BoundButton<TFamily> button;

//         operator pawkit_input_digital_binding_t () {
//             return {
//                 .family = pawkit_input_family_t(TFamily),
//                 .binding = FamilyTraits<TFamily>::GetDigital(button)
//             };
//         }
//     };

//     template <Family TFamily>
//     struct AnalogBinding final {
//         BoundAxis<TFamily> axis;
//         pawkit_f32 deadzone;
//         pawkit_f32 scale;

//         operator pawkit_input_analog_binding_t () {
//             return {
//                 .family = pawkit_input_family_t(TFamily),
//                 .binding = FamilyTraits<TFamily>::GetAnalog(axis),
//                 .deadzone = deadzone,
//                 .scale = scale,
//             };
//         }
//     };

//     template <Family TFamily>
//     struct VectorBinding final {
//         BoundAxis<TFamily> x;
//         BoundAxis<TFamily> y;
//         pawkit_f32 deadzone;
//         pawkit_f32 scaleX;
//         pawkit_f32 scaleY;

//         operator pawkit_input_vector_binding_t () {
//             return {
//                 .family = pawkit_input_family_t(TFamily),
//                 .x = FamilyTraits<TFamily>::GetAnalog(x),
//                 .y = FamilyTraits<TFamily>::GetAnalog(y),
//                 .deadzone = deadzone,
//                 .scale_x = scaleX,
//                 .scale_y = scaleY,
//             };
//         }
//     };

//     struct DigitalFrame final {
//         bool value;
//         bool justPressed;
//         bool justReleased;
//     };

//     struct AnalogFrame final {
//         pawkit_f32 value;
//         pawkit_f32 delta;
//     };

//     struct VectorFrame final {
//         pawkit_f32 x;
//         pawkit_f32 y;
//         pawkit_f32 deltaX;
//         pawkit_f32 deltaY;
//     };

//     using Frame = std::variant<DigitalFrame, AnalogFrame, VectorFrame>;

//     template <Family TFamily>
//     struct State final {
//         State() = delete;
//         State(State const &copy) = delete;
//         State(State &&move) = delete;
//         ~State() = delete;
//         void operator delete (void *) = delete;

//         operator pawkit_input_state_t () {
//             return reinterpret_cast<pawkit_input_state_t>(this);
//         }

//         static State *From(pawkit_input_state_t state) {
//             return reinterpret_cast<State *>(state);
//         }

//         inline void SetButton(Button<TFamily> button, bool value) {
//             pawkit_input_state_set_button(*this, pawkit_input_button_t(button), value);
//         }

//         inline void SetAxis(Axis<TFamily> axis, pawkit_f32 value) requires (TFamily != Family::Key) {
//             pawkit_input_state_set_axis(*this, pawkit_input_axis_t(axis), value);
//         }
//     }; 

//     struct Manager final  {
//         ~Manager() {
//             pawkit_input_manager_free(*this);
//         };

//         Manager() = delete;
//         Manager(Manager const &copy) = delete;
//         Manager(Manager &&move) = delete;

//         operator pawkit_input_manager_t () {
//             return reinterpret_cast<pawkit_input_manager_t>(this);
//         }

//         void operator delete (void *ptr) {
//             // Empty to avoid double free.
//         }

//         static Manager *From(pawkit_input_manager_t list) {
//             return reinterpret_cast<Manager *>(list);
//         }

//         template <Family TFamily>
//         inline bool RegisterDigitalBinding(std::string_view name, std::span<DigitalBinding<TFamily>> bindings) {
//             std::unique_ptr<pawkit_input_binding_t[]> rawBindingsPtr {new pawkit_input_binding_t[bindings.size()]};

//             for (pawkit_usize i = 0; i < bindings.size(); i++) {
//                 rawBindingsPtr[i] = {
//                     .digital = bindings[i],
//                 };
//             }

//             pawkit_input_manager_register_binding(*this, name.data(), name.size(), PAWKIT_INPUT_BINDING_DIGITAL, rawBindingsPtr.get(), bindings.size());
//         }

//         template <Family TFamily>
//         inline bool RegisterAnalogBinding(std::string_view name, std::span<AnalogBinding<TFamily>> bindings) {
//             std::unique_ptr<pawkit_input_binding_t[]> rawBindingsPtr {new pawkit_input_binding_t[bindings.size()]};

//             for (pawkit_usize i = 0; i < bindings.size(); i++) {
//                 rawBindingsPtr[i] = {
//                     .analog = bindings[i],
//                 };
//             }

//             pawkit_input_manager_register_binding(*this, name.data(), name.size(), PAWKIT_INPUT_BINDING_ANALOG, rawBindingsPtr.get(), bindings.size());
//         }

//         template <Family TFamily>
//         inline bool RegisterVectorBinding(std::string_view name, std::span<VectorBinding<TFamily>> bindings) {
//             std::unique_ptr<pawkit_input_binding_t[]> rawBindingsPtr {new pawkit_input_binding_t[bindings.size()]};

//             for (pawkit_usize i = 0; i < bindings.size(); i++) {
//                 rawBindingsPtr[i] = {
//                     .vector = bindings[i],
//                 };
//             }

//             pawkit_input_manager_register_binding(*this, name.data(), name.size(), PAWKIT_INPUT_BINDING_VECTOR, rawBindingsPtr.get(), bindings.size());
//         }

//         inline void LockBindings() {
//             pawkit_input_manager_lock_bindings(*this);
//         }

//         template <Family TFamily>
//         inline void DeviceConnected(pawkit_usize id) {
//             pawkit_input_manager_device_connected(*this, pawkit_input_family_t(TFamily), id);
//         }

//         template <Family TFamily>
//         inline void DeviceDisconnected(pawkit_usize id) {
//             pawkit_input_manager_device_disconnected(*this, pawkit_input_family_t(TFamily), id);
//         }
        
//         template <Family TFamily>
//         inline State<TFamily> *GetState(pawkit_usize id) {
//             return State<TFamily>::From(pawkit_input_manager_get_state(*this, pawkit_input_family_t(TFamily), id));
//         }

//         inline pawkit_usize CreateHandler() {
//             return pawkit_input_manager_create_handler(*this);
//         }

//         inline void freeHandler(pawkit_usize handler) {
//             pawkit_input_manager_free_handler(*this, handler);
//         }

//         inline void Update() {
//             pawkit_input_manager_update(*this);
//         }

//         inline std::optional<DigitalFrame> GetDigitalFrame(pawkit_usize handler, std::string_view name) {
//             pawkit_input_frame_t rawFrame;

//             if (!pawkit_input_manager_get_frame(*this, handler, name.data(), name.size(), &rawFrame))
//                 return std::nullopt;

//             if (rawFrame.type != PAWKIT_INPUT_BINDING_DIGITAL)
//                 return std::nullopt;

//             return DigitalFrame {
//                 .value = rawFrame.digital.value,
//                 .justPressed = rawFrame.digital.just_pressed,
//                 .justReleased = rawFrame.digital.just_released,
//             };
//         }

//         inline std::optional<AnalogFrame> GetAnalogFrame(pawkit_usize handler, std::string_view name) {
//             pawkit_input_frame_t rawFrame;

//             if (!pawkit_input_manager_get_frame(*this, handler, name.data(), name.size(), &rawFrame))
//                 return std::nullopt;

//             if (rawFrame.type != PAWKIT_INPUT_BINDING_ANALOG)
//                 return std::nullopt;

//             return AnalogFrame {
//                 .value = rawFrame.analog.value,
//                 .delta = rawFrame.analog.delta,
//             };
//         }

//         inline std::optional<VectorFrame> GetVectorFrame(pawkit_usize handler, std::string_view name) {
//             pawkit_input_frame_t rawFrame;

//             if (!pawkit_input_manager_get_frame(*this, handler, name.data(), name.size(), &rawFrame))
//                 return std::nullopt;

//             if (rawFrame.type != PAWKIT_INPUT_BINDING_VECTOR)
//                 return std::nullopt;

//             return VectorFrame {
//                 .x = rawFrame.vector.x,
//                 .y = rawFrame.vector.y,
//                 .deltaX = rawFrame.vector.delta_x,
//                 .deltaY = rawFrame.vector.delta_y,
//             };
//         }

//         inline std::optional<Frame> GetFrame(pawkit_usize handler, std::string_view name) {
//             pawkit_input_frame_t rawFrame;

//             if (!pawkit_input_manager_get_frame(*this, handler, name.data(), name.size(), &rawFrame))
//                 return std::nullopt;

//             if (rawFrame.type == PAWKIT_INPUT_BINDING_DIGITAL) {
//                 return DigitalFrame {
//                     .value = rawFrame.digital.value,
//                     .justPressed = rawFrame.digital.just_pressed,
//                     .justReleased = rawFrame.digital.just_released,
//                 };
//             } else if (rawFrame.type == PAWKIT_INPUT_BINDING_ANALOG) {
//                 return AnalogFrame {
//                     .value = rawFrame.analog.value,
//                     .delta = rawFrame.analog.delta,
//                 };
//             } else if (rawFrame.type == PAWKIT_INPUT_BINDING_VECTOR) {
//                 return VectorFrame {
//                     .x = rawFrame.vector.x,
//                     .y = rawFrame.vector.y,
//                     .deltaX = rawFrame.vector.delta_x,
//                     .deltaY = rawFrame.vector.delta_y,
//                 };
//             }

//             return std::nullopt;
//         }

//         template <Family TFamily>
//         inline void ConnectDeviceToHandler(pawkit_usize handler, pawkit_usize device) {
//             pawkit_input_manager_connect_device_to_handler(*this, handler, pawkit_input_family_t(TFamily), device);
//         }

//         template <Family TFamily>
//         inline void DisconnectDeviceFromHandler(pawkit_usize handler, pawkit_usize device) {
//             pawkit_input_manager_disconnect_device_from_handler(*this, handler, pawkit_input_family_t(TFamily), device);
//         }
//     };
// }
