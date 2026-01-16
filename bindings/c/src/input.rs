use std::{ffi::c_char, fmt::Debug, mem::forget, ptr::null_mut};

use num_enum::TryFromPrimitive;
use pawkit_input::{
    DeviceId,
    binding::{
        AnalogBinding, AnalogBindingKind, BindingList, BoundAxis, BoundButton, DigitalBinding,
        VectorBinding, VectorBindingKind,
        axis::{GamepadAxis, MouseAxis},
        button::{GamepadButton, KeyboardButton, MouseButton},
        map::BindingMap,
    },
    manager::{InputManager, RawInputFrame},
    state::{InputFamily, InputState},
};
use pawkit_interner::InternString;
use serde::Serialize;

use crate::{
    c_enum, cstr_to_str, drop_from_heap, move_to_heap, move_to_stack, ptr_to_ref, ptr_to_ref_mut,
    ptr_to_slice, set_if_valid, str_to_cstr,
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
    axis: CAxis,
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
    button: CButton,
    axis: CAxis,
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
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CAnalogBinding {
    family: CInputFamily,
    binding: CBoundAxis,
    deadzone: f32,
    scale: f32,
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

unsafe fn convert_bound_button<TButton, TAxis>(
    button: CBoundButton,
) -> Option<BoundButton<TButton, TAxis>>
where
    TButton:
        Sized + Debug + Clone + Copy + PartialEq + Serialize + TryFromPrimitive<Primitive = u8>,
    TAxis: Sized + Debug + Clone + Copy + PartialEq + Serialize + TryFromPrimitive<Primitive = u8>,
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
    TButton:
        Sized + Debug + Clone + Copy + PartialEq + Serialize + TryFromPrimitive<Primitive = u8>,
    TAxis: Sized + Debug + Clone + Copy + PartialEq + Serialize + TryFromPrimitive<Primitive = u8>,
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

fn convert_digital_binding(binding: CDigitalBinding) -> Option<DigitalBinding> {
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

fn convert_analog_binding(binding: CAnalogBinding) -> Option<AnalogBinding> {
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

fn convert_vector_binding(binding: CVectorBinding) -> Option<VectorBinding> {
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

type CBindingMap = *mut BindingMap;
type CInputState = *mut InputState;
type CInputManager = *mut InputManager;

c_enum!(InputError {
    ERROR_OK,
    ERROR_INVALID_STRING,
    ERROR_INVALID_JSON,
});

unsafe fn ok(error: *mut InputError) {
    unsafe {
        set_if_valid(error, ERROR_OK);
    }
}

unsafe fn err(error: *mut InputError, value: InputError) {
    unsafe {
        set_if_valid(error, value);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_create() -> CBindingMap {
    unsafe {
        return move_to_heap(BindingMap::new());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_destroy(map: CBindingMap) {
    unsafe {
        drop_from_heap(map);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_load(
    cstr: *const c_char,
    len: usize,
    error: *mut InputError,
) -> CBindingMap {
    unsafe {
        let Some(str) = cstr_to_str(cstr, len) else {
            err(error, ERROR_INVALID_STRING);

            return null_mut();
        };

        let value = match BindingMap::load(str) {
            Ok(it) => it,
            Err(_) => {
                err(error, ERROR_INVALID_JSON);

                return null_mut();
            }
        };

        ok(error);

        return move_to_heap(value);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_save(map: CBindingMap, len: *mut usize) -> *const c_char {
    unsafe {
        let Some(map) = ptr_to_ref(map) else {
            return null_mut();
        };

        let Some(len) = ptr_to_ref_mut(len) else {
            return null_mut();
        };

        return str_to_cstr(&map.save(), len);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_register_digital_binding(
    map: CBindingMap,
    name: *const u8,
    bindings: *const CDigitalBinding,
    len: usize,
) {
    unsafe {
        let Some(map) = ptr_to_ref_mut(map) else {
            return;
        };

        let Some(bindings) = ptr_to_slice(bindings, len) else {
            return;
        };

        let Some(name) = InternString::from_raw(name) else {
            return;
        };

        let bindings = bindings
            .iter()
            .cloned()
            .map(convert_digital_binding)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        map.register_binding(name.clone(), BindingList::Digital(bindings));

        forget(name);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_register_analog_binding(
    map: CBindingMap,
    name: *const u8,
    bindings: *const CAnalogBinding,
    len: usize,
) {
    unsafe {
        let Some(map) = ptr_to_ref_mut(map) else {
            return;
        };

        let Some(bindings) = ptr_to_slice(bindings, len) else {
            return;
        };

        let Some(name) = InternString::from_raw(name) else {
            return;
        };

        let bindings = bindings
            .iter()
            .cloned()
            .map(convert_analog_binding)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        map.register_binding(name.clone(), BindingList::Analog(bindings));

        forget(name);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_register_vector_binding(
    map: CBindingMap,
    name: *const u8,
    bindings: *const CVectorBinding,
    len: usize,
) {
    unsafe {
        let Some(map) = ptr_to_ref_mut(map) else {
            return;
        };

        let Some(bindings) = ptr_to_slice(bindings, len) else {
            return;
        };

        let Some(name) = InternString::from_raw(name) else {
            return;
        };

        let bindings = bindings
            .iter()
            .cloned()
            .map(convert_vector_binding)
            .filter(Option::is_some)
            .map(Option::unwrap)
            .collect();

        map.register_binding(name.clone(), BindingList::Vector(bindings));

        forget(name);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_binding_map_ensure_prototype(
    map: CBindingMap,
    prototype: CBindingMap,
) {
    unsafe {
        let Some(map) = ptr_to_ref_mut(map) else {
            return;
        };

        let Some(prototype) = ptr_to_ref(prototype) else {
            return;
        };

        map.ensure_prototype(prototype);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_create() -> CInputState {
    unsafe {
        return move_to_heap(InputState::new());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_destroy(state: CInputState) {
    unsafe {
        drop_from_heap(state);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_connect_keyboard(state: CInputState) -> DeviceId {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return DeviceId::null();
        };

        return state.connect_keyboard();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_connect_mouse(state: CInputState) -> DeviceId {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return DeviceId::null();
        };

        return state.connect_mouse();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_connect_gamepad(state: CInputState) -> DeviceId {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return DeviceId::null();
        };

        return state.connect_gamepad();
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_disconnect_device(state: CInputState, device: DeviceId) {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return;
        };

        state.disconnect_device(&device);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_set_button(
    state: CInputState,
    device: DeviceId,
    button: CButton,
    value: bool,
) {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return;
        };

        let Some(family) = state.device_family(&device) else {
            return;
        };

        match family {
            InputFamily::Keyboard => {
                let Ok(button) = KeyboardButton::try_from_primitive(button) else {
                    return;
                };

                state.set_keyboard_button(&device, button, value);
            }

            InputFamily::Mouse => {
                let Ok(button) = MouseButton::try_from_primitive(button) else {
                    return;
                };

                state.set_mouse_button(&device, button, value);
            }

            InputFamily::Gamepad => {
                let Ok(button) = GamepadButton::try_from_primitive(button) else {
                    return;
                };

                state.set_gamepad_button(&device, button, value);
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_state_set_axis(
    state: CInputState,
    device: DeviceId,
    button: CAxis,
    value: f32,
) {
    unsafe {
        let Some(state) = ptr_to_ref_mut(state) else {
            return;
        };

        let Some(family) = state.device_family(&device) else {
            return;
        };

        match family {
            InputFamily::Keyboard => {}

            InputFamily::Mouse => {
                let Ok(axis) = MouseAxis::try_from_primitive(button) else {
                    return;
                };

                state.set_mouse_axis(&device, axis, value);
            }

            InputFamily::Gamepad => {
                let Ok(axis) = GamepadAxis::try_from_primitive(button) else {
                    return;
                };

                state.set_gamepad_axis(&device, axis, value);
            }
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_create(map: CBindingMap) -> CInputManager {
    unsafe {
        let Some(map) = move_to_stack(map) else {
            return null_mut();
        };

        return move_to_heap(InputManager::new(map));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_destroy(manager: CInputManager) {
    unsafe {
        drop_from_heap(manager);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_connect_device(manager: CInputManager, device: DeviceId) {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return;
        };

        manager.connect_device(device);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_disconnect_device(
    manager: CInputManager,
    device: DeviceId,
) {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return;
        };

        manager.disconnect_device(device);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_update(manager: CInputManager, state: CInputState) {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return;
        };

        let Some(state) = ptr_to_ref(state) else {
            return;
        };

        manager.update(state);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_get(
    manager: CInputManager,
    name: *const u8,
    frame: *mut RawInputFrame,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref(manager) else {
            return false;
        };

        let Some(frame) = ptr_to_ref_mut(frame) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let Some(value) = manager.get_binding_raw(&name) else {
            forget(name);

            return false;
        };

        *frame = value;

        forget(name);

        return true;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_add_digital_binding(
    manager: CInputManager,
    name: *const u8,
    binding: CDigitalBinding,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(value) = convert_digital_binding(binding) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let result = manager.add_digital_binding(name.clone(), value).is_ok();

        forget(name);

        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_add_analog_binding(
    manager: CInputManager,
    name: *const u8,
    binding: CAnalogBinding,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(value) = convert_analog_binding(binding) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let result = manager.add_analog_binding(name.clone(), value).is_ok();

        forget(name);

        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_add_vector_binding(
    manager: CInputManager,
    name: *const u8,
    binding: CVectorBinding,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(value) = convert_vector_binding(binding) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let result = manager.add_vector_binding(name.clone(), value).is_ok();

        forget(name);

        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_remove_digital_binding(
    manager: CInputManager,
    name: *const u8,
    binding: CDigitalBinding,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(value) = convert_digital_binding(binding) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let result = manager.remove_digital_binding(name.clone(), value).is_ok();

        forget(name);

        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_remove_analog_binding(
    manager: CInputManager,
    name: *const u8,
    binding: CAnalogBinding,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(value) = convert_analog_binding(binding) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let result = manager.remove_analog_binding(name.clone(), value).is_ok();

        forget(name);

        return result;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_input_manager_remove_vector_binding(
    manager: CInputManager,
    name: *const u8,
    binding: CVectorBinding,
) -> bool {
    unsafe {
        let Some(manager) = ptr_to_ref_mut(manager) else {
            return false;
        };

        let Some(value) = convert_vector_binding(binding) else {
            return false;
        };

        let Some(name) = InternString::from_raw(name) else {
            return false;
        };

        let result = manager.remove_vector_binding(name.clone(), value).is_ok();

        forget(name);

        return result;
    }
}
