use std::collections::HashMap;

use bitvec::array::BitArray;
use pawkit_crockford::Ulid;

use crate::{
    DeviceId,
    binding::{
        AnalogBinding, AnalogBindingKind, BoundAxis, BoundButton, DigitalBinding, axis::{GamepadAxis, MouseAxis}, button::{GamepadButton, KeyboardButton, MouseButton}
    },
};

enum DeviceState {
    Keyboard(BitArray<[u8; 15]>),
    Mouse(BitArray<u8>, [f32; 4]),
    Gamepad(BitArray<[u8; 4]>, [f32; 6]),
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputFamily {
    Keyboard,
    Mouse,
    Gamepad,
}

pub struct InputState {
    devices: HashMap<Ulid, DeviceState>,
}

impl InputState {
    pub fn new() -> Self {
        return Self {
            devices: HashMap::new(),
        };
    }

    #[inline(always)]
    fn connect_device(&mut self, state: DeviceState) -> DeviceId {
        let id = Ulid::new();

        self.devices.insert(id, state);

        return DeviceId(id);
    }

    pub fn device_family(&self, device: &DeviceId) -> Option<InputFamily> {
        let device = self.devices.get(&device.0)?;

        match device {
            DeviceState::Keyboard(_) => return Some(InputFamily::Keyboard),
            DeviceState::Mouse(_, _) => return Some(InputFamily::Mouse),
            DeviceState::Gamepad(_, _) => return Some(InputFamily::Gamepad),
        }
    }

    pub fn connect_keyboard(&mut self) -> DeviceId {
        return self.connect_device(DeviceState::Keyboard(BitArray::new([0; 15])));
    }

    pub fn connect_mouse(&mut self) -> DeviceId {
        return self.connect_device(DeviceState::Mouse(BitArray::new(0), [0f32; 4]));
    }

    pub fn connect_gamepad(&mut self) -> DeviceId {
        return self.connect_device(DeviceState::Gamepad(BitArray::new([0; 4]), [0f32; 6]));
    }

    pub fn disconnect_device(&mut self, device: &DeviceId) -> bool {
        return self.devices.remove(&device.0).is_some();
    }

    pub fn set_keyboard_button(
        &mut self,
        device: &DeviceId,
        button: KeyboardButton,
        value: bool,
    ) -> bool {
        let Some(device) = self.devices.get_mut(&device.0) else {
            return false;
        };

        let DeviceState::Keyboard(buttons) = device else {
            return false;
        };

        buttons.set(button as usize, value);

        return true;
    }

    pub fn set_mouse_button(
        &mut self,
        device: &DeviceId,
        button: MouseButton,
        value: bool,
    ) -> bool {
        let Some(device) = self.devices.get_mut(&device.0) else {
            return false;
        };

        let DeviceState::Mouse(buttons, _) = device else {
            return false;
        };

        buttons.set(button as usize, value);

        return true;
    }

    pub fn set_gamepad_button(
        &mut self,
        device: &DeviceId,
        button: GamepadButton,
        value: bool,
    ) -> bool {
        let Some(device) = self.devices.get_mut(&device.0) else {
            return false;
        };

        let DeviceState::Gamepad(buttons, _) = device else {
            return false;
        };

        buttons.set(button as usize, value);

        return true;
    }

    pub fn set_mouse_axis(&mut self, device: &DeviceId, axis: MouseAxis, value: f32) -> bool {
        let Some(device) = self.devices.get_mut(&device.0) else {
            return false;
        };

        let DeviceState::Mouse(_, axes) = device else {
            return false;
        };

        axes[axis as usize] = value;

        return true;
    }

    pub fn set_gamepad_axis(&mut self, device: &DeviceId, axis: GamepadAxis, value: f32) -> bool {
        let Some(device) = self.devices.get_mut(&device.0) else {
            return false;
        };

        let DeviceState::Gamepad(_, axes) = device else {
            return false;
        };

        axes[axis as usize] = value;

        return true;
    }

    pub fn get_keyboard_button(&self, device: &DeviceId, button: KeyboardButton) -> Option<bool> {
        let device = self.devices.get(&device.0)?;

        let DeviceState::Keyboard(buttons) = device else {
            return None;
        };

        return buttons.get(button as usize).map(|it| *it);
    }

    pub fn get_mouse_button(&self, device: &DeviceId, button: MouseButton) -> Option<bool> {
        let device = self.devices.get(&device.0)?;

        let DeviceState::Mouse(buttons, _) = device else {
            return None;
        };

        return buttons.get(button as usize).map(|it| *it);
    }

    pub fn get_gamepad_button(&self, device: &DeviceId, button: GamepadButton) -> Option<bool> {
        let device = self.devices.get(&device.0)?;

        let DeviceState::Gamepad(buttons, _) = device else {
            return None;
        };

        return buttons.get(button as usize).map(|it| *it);
    }

    pub fn get_mouse_axis(&self, device: &DeviceId, axis: MouseAxis) -> Option<f32> {
        let device = self.devices.get(&device.0)?;

        let DeviceState::Mouse(_, axes) = device else {
            return None;
        };

        return Some(axes[axis as usize]);
    }

    pub fn get_gamepad_axis(&self, device: &DeviceId, axis: GamepadAxis) -> Option<f32> {
        let device = self.devices.get(&device.0)?;

        let DeviceState::Gamepad(_, axes) = device else {
            return None;
        };

        return Some(axes[axis as usize]);
    }

    pub fn get_digital(&self, device: &DeviceId, bindings: &[DigitalBinding]) -> Option<bool> {
        let device = self.devices.get(&device.0)?;

        for binding in bindings {
            match (device, binding) {
                (DeviceState::Keyboard(buttons), DigitalBinding::Keyboard(BoundButton::Digital(button))) => {
                    if buttons[*button as usize] {
                        return Some(true);
                    }
                }

                (DeviceState::Gamepad(buttons, _), DigitalBinding::Gamepad(BoundButton::Digital(button))) => {
                    if buttons[*button as usize] {
                        return Some(true);
                    }
                }

                (DeviceState::Gamepad(_, axes), DigitalBinding::Gamepad(BoundButton::Analog { axis, threshold })) => {
                    if axes[*axis as usize] > *threshold {
                        return Some(true);
                    }
                }

                (DeviceState::Mouse(buttons, _), DigitalBinding::Mouse(BoundButton::Digital(button))) => {
                    if buttons[*button as usize] {
                        return Some(true);
                    }
                }

                (DeviceState::Mouse(_, axes), DigitalBinding::Mouse(BoundButton::Analog { axis, threshold })) => {
                    if axes[*axis as usize] > *threshold {
                        return Some(true);
                    }
                }

                _ => continue
            }
        }
        
        return Some(false);
    }

    pub fn get_analog(&self, device: &DeviceId, bindings: &[AnalogBinding]) -> Option<f32> {
        let device = self.devices.get(&device.0)?;

        let mut value = 0f32;

        for binding in bindings {
            match (device, binding.axis) {
                (DeviceState::Keyboard(buttons), AnalogBindingKind::Keyboard(BoundAxis::Digital(button))) => {
                    todo!()
                }

                (DeviceState::Keyboard(buttons), AnalogBindingKind::Keyboard(BoundAxis::MultiDigital { positive, negative})) => {
                    todo!()
                }


                (DeviceState::Gamepad(_, axes), AnalogBindingKind::Gamepad(BoundAxis::Analog(axis))) => {
                    todo!()
                }

                (DeviceState::Gamepad(buttons, _), AnalogBindingKind::Gamepad(BoundAxis::Digital(button))) => {
                    todo!()
                }

                (DeviceState::Gamepad(buttons, _), AnalogBindingKind::Gamepad(BoundAxis::MultiDigital { positive, negative})) => {
                    todo!()
                }


                (DeviceState::Mouse(_, axes), AnalogBindingKind::Mouse(BoundAxis::Analog(axis))) => {
                    todo!()
                }

                (DeviceState::Mouse(buttons, _), AnalogBindingKind::Mouse(BoundAxis::Digital(button))) => {
                    todo!()
                }

                (DeviceState::Mouse(buttons, _), AnalogBindingKind::Mouse(BoundAxis::MultiDigital { positive, negative})) => {
                    todo!()
                }

                
                _ => continue
            }
        }
        
        todo!()
    }
}
