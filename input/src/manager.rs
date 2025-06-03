use pawkit_bitarray::BitArray;
use pawkit_holy_array::HolyArray;

pub struct InputFamily {
    pub name: String,
    pub digital_count: usize,
    pub analog_count: usize,
}

pub struct InputManager {
    family: InputFamily,
    devices: HolyArray<InputDeviceState>,
}

pub struct InputDeviceState {
    pub digital_inputs: BitArray,
    pub analog_inputs: Box<[f32]>,
    raw_id: usize,
}

impl InputManager {
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
            digital_inputs: BitArray::new(self.family.digital_count),
            analog_inputs: vec![0f32; self.family.analog_count].into_boxed_slice(),
            raw_id,
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
    pub fn get_analog<TAnalog>(&self, axis: TAnalog) -> f32
    where
        TAnalog: Sized + Into<u8>,
    {
        return self.analog_inputs[axis.into() as usize];
    }

    pub fn get_digital<TDigital>(&self, button: TDigital) -> bool
    where
        TDigital: Sized + Into<u8>,
    {
        return self.digital_inputs.get(button.into() as usize).unwrap();
    }

    pub fn set_analog<TAnalog>(&mut self, axis: TAnalog, value: f32)
    where
        TAnalog: Sized + Into<u8>,
    {
        self.analog_inputs[axis.into() as usize] = value
    }

    pub fn set_digital<TDigital>(&mut self, button: TDigital, value: bool)
    where
        TDigital: Sized + Into<u8>,
    {
        let index = button.into();

        if value {
            self.digital_inputs.set(index as usize);
        } else {
            self.digital_inputs.reset(index as usize);
        }
    }
}
