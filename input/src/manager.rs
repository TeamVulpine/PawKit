use pawkit_bitarray::BitArray;
use pawkit_holy_array::HolyArray;

pub enum InputFamily {
    Keyboard,
    Mouse,
    Gamepad,
}

impl InputFamily {
    pub fn digital_count(&self) -> usize {
        return match self {
            Self::Keyboard => 120,
            Self::Mouse => 5,
            Self::Gamepad => 26,
        };
    }

    pub fn analog_count(&self) -> usize {
        return match self {
            Self::Keyboard => 0,
            Self::Mouse => 4,
            Self::Gamepad => 6,
        };
    }
}

pub struct InputDeviceManager {
    family: InputFamily,
    devices: HolyArray<InputDeviceState>,
}

pub struct InputDeviceState {
    raw_id: usize,
    pub digital_inputs: BitArray,
    pub analog_inputs: Box<[f32]>,
}

impl InputDeviceManager {
    pub fn new(family: InputFamily) -> Self {
        return Self {
            family,
            devices: HolyArray::new(),
        };
    }

    pub fn raw_id_to_id(&self, raw_id: usize) -> Option<usize> {
        for id in 0..self.devices.len() {
            let Some(device) = self.devices.get(id) else {
                continue;
            };

            if device.raw_id == raw_id {
                return Some(id);
            }
        }

        return None;
    }

    pub fn device_connected(&mut self, raw_id: usize) -> usize {
        return self.devices.acquire(InputDeviceState {
            raw_id,
            digital_inputs: BitArray::new(self.family.digital_count()),
            analog_inputs: vec![0f32; self.family.analog_count()].into_boxed_slice(),
        });
    }

    pub fn device_disconnected(&mut self, id: usize) {
        self.devices.release(id);
    }

    pub fn device_disconnected_raw(&mut self, raw_id: usize) {
        let Some(id) = self.raw_id_to_id(raw_id) else {
            return;
        };

        self.device_disconnected(id);
    }

    pub fn get_state(&self, id: usize) -> Option<&InputDeviceState> {
        return self.devices.get(id);
    }

    pub fn get_state_raw(&self, raw_id: usize) -> Option<&InputDeviceState> {
        let Some(id) = self.raw_id_to_id(raw_id) else {
            return None;
        };

        return self.get_state(id);
    }

    pub fn get_state_mut(&mut self, id: usize) -> Option<&mut InputDeviceState> {
        return self.devices.get_mut(id);
    }

    pub fn get_state_raw_mut(&mut self, raw_id: usize) -> Option<&mut InputDeviceState> {
        let Some(id) = self.raw_id_to_id(raw_id) else {
            return None;
        };

        return self.get_state_mut(id);
    }
}

impl InputDeviceState {
    pub fn get_analog(&self, axis: usize) -> f32 {
        return self.analog_inputs[axis];
    }

    pub fn get_digital(&self, button: usize) -> bool {
        return self.digital_inputs.get(button).unwrap();
    }

    pub fn set_analog(&mut self, axis: usize, value: f32) {
        self.analog_inputs[axis] = value
    }

    pub fn set_digital(&mut self, button: usize, value: bool) {
        if value {
            self.digital_inputs.set(button);
        } else {
            self.digital_inputs.reset(button);
        }
    }
}
