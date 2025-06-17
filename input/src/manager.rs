use std::sync::{Arc, RwLock};

use num_enum::TryFromPrimitive;
use pawkit_bitarray::BitArray;
use pawkit_holy_array::HolyArray;
use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, TryFromPrimitive,
)]
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
    devices: RwLock<HolyArray<Arc<InputDeviceState>>>,
}

pub struct InputDeviceState {
    raw_id: usize,
    pub digital_inputs: RwLock<BitArray>,
    pub analog_inputs: RwLock<Box<[f32]>>,
}

impl InputDeviceManager {
    pub fn new(family: InputFamily) -> Self {
        return Self {
            family,
            devices: RwLock::new(HolyArray::new()),
        };
    }

    pub fn raw_id_to_id(&self, raw_id: usize) -> Option<usize> {
        let devices = self.devices.read().ok()?;
        
        for id in 0..devices.len() {
            let Some(device) = devices.get(id) else {
                continue;
            };

            if device.raw_id == raw_id {
                return Some(id);
            }
        }

        return None;
    }

    pub fn device_connected(&self, raw_id: usize) -> usize {
        let mut devices = self.devices.write().unwrap();

        return devices.acquire(Arc::new(InputDeviceState {
            raw_id,
            digital_inputs: RwLock::new(BitArray::new(self.family.digital_count())),
            analog_inputs: RwLock::new(vec![0f32; self.family.analog_count()].into_boxed_slice()),
        }));
    }

    pub fn device_disconnected(&self, id: usize) {
        let mut devices = self.devices.write().unwrap();

        devices.release(id);
    }

    pub fn device_disconnected_raw(&self, raw_id: usize) {
        let Some(id) = self.raw_id_to_id(raw_id) else {
            return;
        };

        self.device_disconnected(id);
    }

    pub fn get_state(&self, id: usize) -> Option<Arc<InputDeviceState>> {
        let devices = self.devices.read().ok()?;

        return devices.get(id).map(Arc::clone);
    }

    pub fn get_state_raw(&self, raw_id: usize) -> Option<Arc<InputDeviceState>> {
        let Some(id) = self.raw_id_to_id(raw_id) else {
            return None;
        };

        return self.get_state(id);
    }
}

impl InputDeviceState {
    pub fn get_analog(&self, axis: usize) -> f32 {
        let analog = self.analog_inputs.read().unwrap();

        return analog[axis];
    }

    pub fn get_digital(&self, button: usize) -> bool {  
        let digital = self.digital_inputs.read().unwrap();

        return digital.get(button).unwrap();
    }

    pub fn set_analog(&self, axis: usize, value: f32) {
        let mut analog = self.analog_inputs.write().unwrap();

        analog[axis] = value
    }

    pub fn set_digital(&self, button: usize, value: bool) {
        let mut digital = self.digital_inputs.write().unwrap();

        if value {
            digital.set(button);
        } else {
            digital.reset(button);
        }
    }
}
