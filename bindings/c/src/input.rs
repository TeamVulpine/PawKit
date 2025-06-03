use std::{ffi::c_char, fmt::Debug};

use num_enum::TryFromPrimitive;
use pawkit_input::{
    bindings::{
        AnalogBinding, AnalogBindingKind, BoundAxis, BoundButton, DefaultBindingType,
        DigitalBinding, VectorBinding, VectorBindingKind,
    },
    InputManager,
};
use serde::Serialize;

use crate::{c_enum, cstr_to_str, drop_from_heap, move_to_heap, ptr_to_ref_mut, ptr_to_slice};

c_enum!(CMouseAxisKind: u8 {
    MOUSEAXIS_DELTA_X,
    MOUSEAXIS_DELTA_Y,
    MOUSEAXIS_WHEEL_X,
    MOUSEAXIS_WHEEL_Y,
});

c_enum!(CJoyAxisKind: u8 {
    JOYAXIS_LEFT_X,
    JOYAXIS_LEFT_Y,
    JOYAXIS_RIGHT_X,
    JOYAXIS_RIGHT_Y,
    JOYAXIS_LEFT_TRIGGER,
    JOYAXIS_RIGHT_TRIGGER,
});

c_enum!(CKeyButtonKind: u8 {
    KEYBUTTON_A, KEYBUTTON_B, KEYBUTTON_C, KEYBUTTON_D,
    KEYBUTTON_E, KEYBUTTON_F, KEYBUTTON_G, KEYBUTTON_H,
    KEYBUTTON_I, KEYBUTTON_J, KEYBUTTON_K, KEYBUTTON_L,
    KEYBUTTON_M, KEYBUTTON_N, KEYBUTTON_O, KEYBUTTON_P,
    KEYBUTTON_Q, KEYBUTTON_R, KEYBUTTON_S, KEYBUTTON_T,
    KEYBUTTON_U, KEYBUTTON_V, KEYBUTTON_W, KEYBUTTON_X,
    KEYBUTTON_Y, KEYBUTTON_Z,

    KEYBUTTON_NUMBER_0, KEYBUTTON_NUMBER_1, KEYBUTTON_NUMBER_2,
    KEYBUTTON_NUMBER_3, KEYBUTTON_NUMBER_4, KEYBUTTON_NUMBER_5,
    KEYBUTTON_NUMBER_6, KEYBUTTON_NUMBER_7, KEYBUTTON_NUMBER_8,
    KEYBUTTON_NUMBER_9,

    KEYBUTTON_UP, KEYBUTTON_DOWN, KEYBUTTON_LEFT, KEYBUTTON_RIGHT,

    KEYBUTTON_TILDE, KEYBUTTON_GRAVE, KEYBUTTON_MINUS, KEYBUTTON_PLUS,
    KEYBUTTON_LEFT_BRACKET, KEYBUTTON_RIGHT_BRACKET, KEYBUTTON_SEMICOLON,
    KEYBUTTON_QUOTE, KEYBUTTON_COMMA, KEYBUTTON_PERIOD,
    KEYBUTTON_SLASH, KEYBUTTON_BACKSLASH,

    KEYBUTTON_LEFT_SHIFT, KEYBUTTON_RIGHT_SHIFT,
    KEYBUTTON_LEFT_CONTROL, KEYBUTTON_RIGHT_CONTROL,
    KEYBUTTON_LEFT_ALT, KEYBUTTON_RIGHT_ALT,
    KEYBUTTON_LEFT_META, KEYBUTTON_RIGHT_META,

    KEYBUTTON_MENU, KEYBUTTON_ENTER, KEYBUTTON_ESCAPE, KEYBUTTON_SPACE,
    KEYBUTTON_TAB, KEYBUTTON_BACKSPACE, KEYBUTTON_INSERT,
    KEYBUTTON_DELETE, KEYBUTTON_PAGE_UP, KEYBUTTON_PAGE_DOWN,
    KEYBUTTON_HOME, KEYBUTTON_END, KEYBUTTON_CAPS_LOCK,
    KEYBUTTON_SCROLL_LOCK, KEYBUTTON_PRINT_SCREEN, KEYBUTTON_PAUSE,
    KEYBUTTON_NUM_LOCK, KEYBUTTON_CLEAR, KEYBUTTON_SLEEP,

    KEYBUTTON_NUMPAD_0, KEYBUTTON_NUMPAD_1, KEYBUTTON_NUMPAD_2,
    KEYBUTTON_NUMPAD_3, KEYBUTTON_NUMPAD_4, KEYBUTTON_NUMPAD_5,
    KEYBUTTON_NUMPAD_6, KEYBUTTON_NUMPAD_7, KEYBUTTON_NUMPAD_8,
    KEYBUTTON_NUMPAD_9, KEYBUTTON_NUMPAD_DIVIDE,
    KEYBUTTON_NUMPAD_MULTIPLY, KEYBUTTON_NUMPAD_MINUS,
    KEYBUTTON_NUMPAD_PLUS, KEYBUTTON_NUMPAD_DECIMAL,
    KEYBUTTON_NUMPAD_PERIOD, KEYBUTTON_NUMPAD_ENTER,

    KEYBUTTON_F1, KEYBUTTON_F2, KEYBUTTON_F3, KEYBUTTON_F4,
    KEYBUTTON_F5, KEYBUTTON_F6, KEYBUTTON_F7, KEYBUTTON_F8,
    KEYBUTTON_F9, KEYBUTTON_F10, KEYBUTTON_F11, KEYBUTTON_F12,
    KEYBUTTON_F13, KEYBUTTON_F14, KEYBUTTON_F15, KEYBUTTON_F16,
    KEYBUTTON_F17, KEYBUTTON_F18, KEYBUTTON_F19, KEYBUTTON_F20,
    KEYBUTTON_F21, KEYBUTTON_F22, KEYBUTTON_F23, KEYBUTTON_F24,
});

c_enum!(CMouseButtonKind: u8 {
    MOUSEBUTTON_LEFT,
    MOUSEBUTTON_RIGHT,
    MOUSEBUTTON_MIDDLE,
    MOUSEBUTTON_SIDE1,
    MOUSEBUTTON_SIDE2,
});

c_enum!(CJoyButtonKind: u8 {
    JOYBUTTON_SOUTH,
    JOYBUTTON_EAST,
    JOYBUTTON_WEST,
    JOYBUTTON_NORTH,
    JOYBUTTON_BACK,
    JOYBUTTON_GUIDE,
    JOYBUTTON_START,
    JOYBUTTON_LEFT_STICK,
    JOYBUTTON_RIGHT_STICK,
    JOYBUTTON_LEFT_SHOULDER,
    JOYBUTTON_RIGHT_SHOULDER,
    JOYBUTTON_DPAD_UP,
    JOYBUTTON_DPAD_DOWN,
    JOYBUTTON_DPAD_LEFT,
    JOYBUTTON_DPAD_RIGHT,
    JOYBUTTON_MISC1,
    JOYBUTTON_RIGHT_PADDLE1,
    JOYBUTTON_LEFT_PADDLE1,
    JOYBUTTON_RIGHT_PADDLE2,
    JOYBUTTON_LEFT_PADDLE2,
    JOYBUTTON_TOUCHPAD,
    JOYBUTTON_MISC2,
    JOYBUTTON_MISC3,
    JOYBUTTON_MISC4,
    JOYBUTTON_MISC5,
    JOYBUTTON_MISC6,
});

c_enum!(CInputFamily: u8 {
    INPUT_FAMILY_KEY,
    INPUT_FAMILY_MOUSE,
    INPUT_FAMILY_JOY,
});

c_enum!(CBoundButtonKind: u8 {
    BOUND_BUTTON_TYPE_DIGITAL,
    BOUND_BUTTON_TYPE_ANALOG,
});

c_enum!(CBoundAxisKind: u8 {
    BOUND_AXIS_TYPE_ANALOG,
    BOUND_AXIS_TYPE_DIGITAL,
    BOUND_AXIS_TYPE_MULTI_DIGITAL,
});

c_enum!(CBindingKind: u8 {
    BINDING_DIGITAL,
    BINDING_ANALOG,
    BINDING_VECTOR,
});

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CBoundButton {
    pub kind: CBoundButtonKind,
    pub value: CBoundButtonUnion,
}

type CButton = u8;
type CAxis = u8;

#[repr(C)]
#[derive(Clone, Copy)]
pub union CBoundButtonUnion {
    pub button: CButton,
    pub analog: CAnalogSpec,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAnalogSpec {
    pub axis: CAxis, // CAxis
    pub threshold: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CBoundAxis {
    pub kind: CBoundAxisKind,
    pub value: CBoundAxisUnion,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CBoundAxisUnion {
    pub button: CButton, // CButton
    pub axis: CAxis,     // CAxis
    pub digital: CMultiDigitalAxis,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CMultiDigitalAxis {
    pub negative: CButton,
    pub positive: CButton,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CDigitalBinding {
    pub family: CInputFamily,
    pub binding: CBoundButton,
    pub _padding: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CAnalogBinding {
    pub family: CInputFamily,
    pub binding: CBoundAxis,
    pub deadzone: f32,
    pub scale: f32,
    pub _padding: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct CVectorBinding {
    pub family: CInputFamily,
    pub x: CBoundAxis,
    pub y: CBoundAxis,
    pub deadzone: f32,
    pub scale_x: f32,
    pub scale_y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union CBinding {
    pub digital: CDigitalBinding,
    pub analog: CAnalogBinding,
    pub vector: CVectorBinding,
}

unsafe fn convert_bound_button<TButton, TAxis>(
    button: CBoundButton,
) -> Option<BoundButton<TButton, TAxis>>
where
    TButton: Sized
        + Debug
        + Clone
        + Copy
        + PartialEq
        + PartialOrd
        + Serialize
        + TryFromPrimitive<Primitive = u8>,
    TAxis: Sized
        + Debug
        + Clone
        + Copy
        + PartialEq
        + PartialOrd
        + Serialize
        + TryFromPrimitive<Primitive = u8>,
{
    match button.kind {
        BOUND_BUTTON_TYPE_ANALOG => {
            return Some(BoundButton::Digital(
                TButton::try_from_primitive(button.value.button).ok()?,
            ));
        }

        BOUND_BUTTON_TYPE_DIGITAL => {
            return Some(BoundButton::Analog {
                axis: TAxis::try_from_primitive(button.value.analog.axis).ok()?,
                threshold: button.value.analog.threshold,
            });
        }

        _ => return None,
    }
}

unsafe fn convert_bound_axis<TButton, TAxis>(axis: CBoundAxis) -> Option<BoundAxis<TButton, TAxis>>
where
    TButton: Sized
        + Debug
        + Clone
        + Copy
        + PartialEq
        + PartialOrd
        + Serialize
        + TryFromPrimitive<Primitive = u8>,
    TAxis: Sized
        + Debug
        + Clone
        + Copy
        + PartialEq
        + PartialOrd
        + Serialize
        + TryFromPrimitive<Primitive = u8>,
{
    match axis.kind {
        BOUND_AXIS_TYPE_ANALOG => {
            return Some(BoundAxis::Analog(
                TAxis::try_from_primitive(axis.value.axis).ok()?,
            ));
        }

        BOUND_AXIS_TYPE_DIGITAL => {
            return Some(BoundAxis::Digital(
                TButton::try_from_primitive(axis.value.button).ok()?,
            ));
        }

        BOUND_AXIS_TYPE_MULTI_DIGITAL => {
            return Some(BoundAxis::MultiDigital {
                negative: TButton::try_from_primitive(axis.value.digital.negative).ok()?,
                positive: TButton::try_from_primitive(axis.value.digital.positive).ok()?,
            });
        }

        _ => return None,
    }
}

unsafe fn convert_digital_binding(binding: CDigitalBinding) -> Option<DigitalBinding> {
    match binding.family {
        INPUT_FAMILY_KEY => {
            return Some(DigitalBinding::Keyboard(convert_bound_button(
                binding.binding,
            )?));
        }

        INPUT_FAMILY_MOUSE => {
            return Some(DigitalBinding::Mouse(convert_bound_button(
                binding.binding,
            )?));
        }

        INPUT_FAMILY_JOY => {
            return Some(DigitalBinding::Gamepad(convert_bound_button(
                binding.binding,
            )?));
        }

        _ => return None,
    }
}

unsafe fn convert_analog_binding(binding: CAnalogBinding) -> Option<AnalogBinding> {
    match binding.family {
        INPUT_FAMILY_KEY => {
            return Some(AnalogBinding {
                axis: AnalogBindingKind::Keyboard(convert_bound_axis(binding.binding)?),
                deadzone: binding.deadzone,
                scale: binding.scale,
            });
        }

        INPUT_FAMILY_MOUSE => {
            return Some(AnalogBinding {
                axis: AnalogBindingKind::Mouse(convert_bound_axis(binding.binding)?),
                deadzone: binding.deadzone,
                scale: binding.scale,
            });
        }

        INPUT_FAMILY_JOY => {
            return Some(AnalogBinding {
                axis: AnalogBindingKind::Gamepad(convert_bound_axis(binding.binding)?),
                deadzone: binding.deadzone,
                scale: binding.scale,
            });
        }

        _ => return None,
    }
}

unsafe fn convert_vector_binding(binding: CVectorBinding) -> Option<VectorBinding> {
    match binding.family {
        INPUT_FAMILY_KEY => {
            return Some(VectorBinding {
                axes: VectorBindingKind::Keyboard {
                    x: convert_bound_axis(binding.x)?,
                    y: convert_bound_axis(binding.y)?,
                },
                deadzone: binding.deadzone,
                scale: (binding.scale_x, binding.scale_y),
            });
        }

        INPUT_FAMILY_MOUSE => {
            return Some(VectorBinding {
                axes: VectorBindingKind::Mouse {
                    x: convert_bound_axis(binding.x)?,
                    y: convert_bound_axis(binding.y)?,
                },
                deadzone: binding.deadzone,
                scale: (binding.scale_x, binding.scale_y),
            });
        }

        INPUT_FAMILY_JOY => {
            return Some(VectorBinding {
                axes: VectorBindingKind::Gamepad {
                    x: convert_bound_axis(binding.x)?,
                    y: convert_bound_axis(binding.y)?,
                },
                deadzone: binding.deadzone,
                scale: (binding.scale_x, binding.scale_y),
            });
        }

        _ => return None,
    }
}

#[no_mangle]
unsafe extern "C" fn pawkit_input_manager_create() -> *mut InputManager {
    return move_to_heap(InputManager::new());
}

#[no_mangle]
unsafe extern "C" fn pawkit_input_manager_destroy(manager: *mut InputManager) {
    drop_from_heap(manager);
}

#[no_mangle]
unsafe extern "C" fn pawkit_input_manager_register_binding(
    manager: *mut InputManager,
    name: *const c_char,
    kind: CBindingKind,
    bindings: *const CBinding,
    size: usize,
) -> bool {
    let Some(manager) = ptr_to_ref_mut(manager) else {
        return false;
    };

    let Some(name) = cstr_to_str(name) else {
        return false;
    };

    let Some(bindings) = ptr_to_slice(bindings, size) else {
        return false;
    };

    match kind {
        BINDING_ANALOG => {
            let mut data = vec![];
            data.reserve(size);

            for i in 0..size {
                let Some(binding) = convert_analog_binding(bindings[i].analog) else {
                    return false;
                };
                data.push(binding);
            }

            return manager
                .register_raw(name, DefaultBindingType::Analog(&data))
                .is_ok();
        }

        BINDING_DIGITAL => {
            let mut data = vec![];
            data.reserve(size);

            for i in 0..size {
                let Some(binding) = convert_digital_binding(bindings[i].digital) else {
                    return false;
                };
                data.push(binding);
            }

            return manager
                .register_raw(name, DefaultBindingType::Digital(&data))
                .is_ok();
        }

        BINDING_VECTOR => {
            let mut data = vec![];
            data.reserve(size);

            for i in 0..size {
                let Some(binding) = convert_vector_binding(bindings[i].vector) else {
                    return false;
                };
                data.push(binding);
            }

            return manager
                .register_raw(name, DefaultBindingType::Vector(&data))
                .is_ok();
        }

        _ => return false,
    }
}

#[no_mangle]
unsafe extern "C" fn pawkit_input_manager_lock_bindings(manager: *mut InputManager) {
    let Some(manager) = ptr_to_ref_mut(manager) else {
        return;
    };

    manager.lock();
}
