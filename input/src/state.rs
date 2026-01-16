use std::{collections::HashMap, fmt::Debug};

use bitvec::{array::BitArray, slice::BitSlice};
use pawkit_crockford::Ulid;
use serde::Serialize;

use crate::{
    DeviceId,
    binding::{
        AnalogBinding, AnalogBindingKind, BoundAxis, BoundButton, DigitalBinding, VectorBinding,
        VectorBindingKind,
        axis::{GamepadAxis, MouseAxis},
        button::{GamepadButton, KeyboardButton, MouseButton},
    },
    length_squared,
};

enum DeviceState {
    Keyboard(BitArray<[u8; 15]>),
    Mouse(BitArray<[u8; 1]>, [f32; 4]),
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

impl DeviceState {
    const EMPTY: [f32; 0] = [];

    fn family(&self) -> InputFamily {
        return match self {
            Self::Keyboard(_) => InputFamily::Keyboard,
            Self::Mouse(_, _) => InputFamily::Mouse,
            Self::Gamepad(_, _) => InputFamily::Gamepad,
        };
    }

    fn digital(&self) -> &BitSlice<u8> {
        return match self {
            Self::Keyboard(b) => b,
            Self::Mouse(b, _) => b,
            Self::Gamepad(b, _) => b,
        };
    }

    fn analog(&self) -> &[f32] {
        return match self {
            Self::Keyboard(_) => &Self::EMPTY,
            Self::Mouse(_, a) => a,
            Self::Gamepad(_, a) => a,
        };
    }
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
        return self.connect_device(DeviceState::Mouse(BitArray::new([0]), [0f32; 4]));
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

    fn get_digital_single<TButton, TAxis>(
        &self,
        digital: &BitSlice<u8>,
        analog: &[f32],
        button: BoundButton<TButton, TAxis>,
    ) -> bool
    where
        TButton: Debug + Copy + PartialEq + Serialize + Into<usize>,
        TAxis: Debug + Copy + PartialEq + Serialize + Into<usize>,
    {
        match button {
            BoundButton::Digital(button) => return digital[button.into()],
            BoundButton::Analog { axis, threshold } => return analog[axis.into()] > threshold,
        }
    }

    pub(crate) fn get_digital(&self, device: &DeviceId, bindings: &[DigitalBinding]) -> Option<bool> {
        let device = self.devices.get(&device.0)?;
        let family = device.family();
        let digital = device.digital();
        let analog = device.analog();

        for binding in bindings {
            match binding {
                DigitalBinding::Keyboard(button) if family == InputFamily::Keyboard => {
                    if self.get_digital_single(digital, analog, *button) {
                        return Some(true);
                    }
                }

                DigitalBinding::Mouse(button) if family == InputFamily::Mouse => {
                    if self.get_digital_single(digital, analog, *button) {
                        return Some(true);
                    }
                }

                DigitalBinding::Gamepad(button) if family == InputFamily::Gamepad => {
                    if self.get_digital_single(digital, analog, *button) {
                        return Some(true);
                    }
                }

                _ => continue,
            }
        }

        return Some(false);
    }

    pub(crate) fn get_analog_single<TButton, TAxis>(
        &self,
        digital: &BitSlice<u8>,
        analog: &[f32],
        axis: BoundAxis<TButton, TAxis>,
    ) -> f32
    where
        TButton: Debug + Copy + PartialEq + Serialize + Into<usize>,
        TAxis: Debug + Copy + PartialEq + Serialize + Into<usize>,
    {
        match axis {
            BoundAxis::Analog(axis) => return analog[axis.into()],
            BoundAxis::Digital(button) => return if digital[button.into()] { 1f32 } else { 0f32 },
            BoundAxis::MultiDigital { negative, positive } => {
                let mut value = 0f32;

                if digital[positive.into()] {
                    value += 1f32;
                }

                if digital[negative.into()] {
                    value -= 1f32;
                }

                return value;
            }
        }
    }

    pub(crate) fn get_analog(&self, device: &DeviceId, bindings: &[AnalogBinding]) -> Option<f32> {
        let device = self.devices.get(&device.0)?;
        let family = device.family();
        let digital = device.digital();
        let analog = device.analog();

        let mut value = 0f32;

        for binding in bindings {
            match binding.axis {
                AnalogBindingKind::Keyboard(axis) if family == InputFamily::Keyboard => {
                    value = value.max(self.get_analog_single(digital, analog, axis));
                }

                AnalogBindingKind::Mouse(axis) if family == InputFamily::Mouse => {
                    value = value.max(self.get_analog_single(digital, analog, axis));
                }

                AnalogBindingKind::Gamepad(axis) if family == InputFamily::Gamepad => {
                    value = value.max(self.get_analog_single(digital, analog, axis));
                }

                _ => continue,
            }
        }

        return Some(value);
    }

    pub(crate) fn get_vector(&self, device: &DeviceId, bindings: &[VectorBinding]) -> Option<[f32; 2]> {
        let device = self.devices.get(&device.0)?;
        let family = device.family();
        let digital = device.digital();
        let analog = device.analog();

        let mut value = [0f32; 2];
        let mut value_len_sqr = 0f32;

        for binding in bindings {
            match binding.axes {
                VectorBindingKind::Keyboard { x, y } if family == InputFamily::Keyboard => {
                    let current = [
                        self.get_analog_single(digital, analog, x),
                        self.get_analog_single(digital, analog, y),
                    ];

                    let current_len_sqr = length_squared(current);

                    if current_len_sqr > value_len_sqr {
                        value = current;
                        value_len_sqr = current_len_sqr;
                    }
                }

                VectorBindingKind::Mouse { x, y } if family == InputFamily::Mouse => {
                    let current = [
                        self.get_analog_single(digital, analog, x),
                        self.get_analog_single(digital, analog, y),
                    ];

                    let current_len_sqr = length_squared(current);

                    if current_len_sqr > value_len_sqr {
                        value = current;
                        value_len_sqr = current_len_sqr;
                    }
                }

                VectorBindingKind::Gamepad { x, y } if family == InputFamily::Gamepad => {
                    let current = [
                        self.get_analog_single(digital, analog, x),
                        self.get_analog_single(digital, analog, y),
                    ];

                    let current_len_sqr = length_squared(current);

                    if current_len_sqr > value_len_sqr {
                        value = current;
                        value_len_sqr = current_len_sqr;
                    }
                }

                _ => continue,
            }
        }

        return Some(value);
    }
}
