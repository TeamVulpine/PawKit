use std::{ffi::c_char, fmt::Debug, ptr::null_mut};

use num_enum::TryFromPrimitive;
use pawkit_input::{
    InputFrame, InputManager,
    bindings::{
        AnalogBinding, AnalogBindingKind, BoundAxis, BoundButton, DefaultBindingType,
        DigitalBinding, VectorBinding, VectorBindingKind,
    },
    manager::{InputDeviceState, InputFamily},
};
use serde::Serialize;

use crate::{
    c_enum, cstr_to_str, drop_from_heap, move_to_heap, ptr_to_ref, ptr_to_ref_mut, ptr_to_slice,
};

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
struct CBoundButton {
    kind: CBoundButtonKind,
    value: CBoundButtonUnion,
}

type CButton = u8;
type CAxis = u8;

#[repr(C)]
#[derive(Clone, Copy)]
union CBoundButtonUnion {
    button: CButton,
    analog: CAnalogSpec,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CAnalogSpec {
    axis: CAxis, // CAxis
    threshold: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CBoundAxis {
    kind: CBoundAxisKind,
    value: CBoundAxisUnion,
}

#[repr(C)]
#[derive(Clone, Copy)]
union CBoundAxisUnion {
    button: CButton, // CButton
    axis: CAxis,     // CAxis
    digital: CMultiDigitalAxis,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CMultiDigitalAxis {
    negative: CButton,
    positive: CButton,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CDigitalBinding {
    family: CInputFamily,
    binding: CBoundButton,
    _padding: u32,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CAnalogBinding {
    family: CInputFamily,
    binding: CBoundAxis,
    deadzone: f32,
    scale: f32,
    _padding: [u32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CVectorBinding {
    family: CInputFamily,
    x: CBoundAxis,
    y: CBoundAxis,
    deadzone: f32,
    scale_x: f32,
    scale_y: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
union CBinding {
    digital: CDigitalBinding,
    analog: CAnalogBinding,
    vector: CVectorBinding,
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
    unsafe {
        match button.kind {
            BOUND_BUTTON_TYPE_DIGITAL => {
                return Some(BoundButton::Digital(
                    TButton::try_from_primitive(button.value.button).ok()?,
                ));
            }

            BOUND_BUTTON_TYPE_ANALOG => {
                return Some(BoundButton::Analog {
                    axis: TAxis::try_from_primitive(button.value.analog.axis).ok()?,
                    threshold: button.value.analog.threshold,
                });
            }

            _ => return None,
        }
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
    unsafe {
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
}

unsafe fn convert_digital_binding(binding: CDigitalBinding) -> Option<DigitalBinding> {
    unsafe {
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
}

unsafe fn convert_analog_binding(binding: CAnalogBinding) -> Option<AnalogBinding> {
    unsafe {
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
}

unsafe fn convert_vector_binding(binding: CVectorBinding) -> Option<VectorBinding> {
    unsafe {
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
}

type CInputManager = *mut InputManager;

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_create() -> CInputManager {
    unsafe {
        return move_to_heap(InputManager::new());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_destroy(manager: CInputManager) {
    unsafe {
        drop_from_heap(manager);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_register_binding(
    manager: CInputManager,
    name: *const c_char,
    name_len: usize,
    kind: CBindingKind,
    bindings: *const CBinding,
    size: usize,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(name) = cstr_to_str(name, name_len) else {
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
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_lock_bindings(manager: CInputManager) {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return;
        };

        manager.lock();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_device_connected(
    manager: CInputManager,
    family: CInputFamily,
    id: usize,
) {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return;
        };

        match family {
            INPUT_FAMILY_KEY => manager.devices.keyboard_manager.device_connected(id),
            INPUT_FAMILY_MOUSE => manager.devices.mouse_manager.device_connected(id),
            INPUT_FAMILY_JOY => manager.devices.gamepad_manager.device_connected(id),
            _ => 0,
        };
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_device_disconnected(
    manager: CInputManager,
    family: CInputFamily,
    id: usize,
) {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return;
        };

        match family {
            INPUT_FAMILY_KEY => manager.devices.keyboard_manager.device_disconnected_raw(id),
            INPUT_FAMILY_MOUSE => manager.devices.mouse_manager.device_disconnected_raw(id),
            INPUT_FAMILY_JOY => manager.devices.gamepad_manager.device_disconnected_raw(id),
            _ => {}
        };
    }
}

type CInputDeviceState = *mut InputDeviceState;

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_get_state(
    manager: CInputManager,
    family: CInputFamily,
    id: usize,
) -> CInputDeviceState {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return null_mut();
        };

        return match family {
            INPUT_FAMILY_KEY => manager.devices.keyboard_manager.get_state_raw(id),
            INPUT_FAMILY_MOUSE => manager.devices.mouse_manager.get_state_raw(id),
            INPUT_FAMILY_JOY => manager.devices.gamepad_manager.get_state_raw(id),
            _ => None,
        }
        .map(|it| it.as_ref() as *const InputDeviceState as CInputDeviceState)
        .unwrap_or(null_mut());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_set_button(
    state: CInputDeviceState,
    button: CButton,
    value: bool,
) {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return;
        };

        state.set_digital(button as usize, value);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_set_axis(
    state: CInputDeviceState,
    axis: CAxis,
    value: f32,
) {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return;
        };

        state.set_analog(axis as usize, value);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_create_handler(
    manager: CInputManager,
    in_handler: &mut usize,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return false;
        };

        let Some(handler) = manager.create_handler() else {
            return false;
        };

        *in_handler = handler;

        return true;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_destroy_handler(manager: CInputManager, handler: usize) {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return;
        };

        manager.destroy_handler(handler);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_update(manager: CInputManager) {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return;
        };

        manager.update();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_get_frame(
    manager: CInputManager,
    handler: usize,
    name: *const c_char,
    name_len: usize,
    in_frame: *mut InputFrame,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return false;
        };

        let Some(name) = cstr_to_str(name, name_len) else {
            return false;
        };

        let Some(frame) = manager.get_frame(handler, name) else {
            return false;
        };

        *in_frame = frame;

        return true;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_connect_device_to_handler(
    manager: CInputManager,
    handler: usize,
    family: CInputFamily,
    id: usize,
) {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return;
        };

        let Ok(family) = InputFamily::try_from_primitive(family) else {
            return;
        };

        manager.connect_device_to_handler_raw(handler, family, id);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_disconnect_device_from_handler(
    manager: CInputManager,
    handler: usize,
    family: CInputFamily,
    id: usize,
) {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return;
        };

        let Ok(family) = InputFamily::try_from_primitive(family) else {
            return;
        };

        manager.disconnect_device_from_handler(handler, family, id);
    }
}
