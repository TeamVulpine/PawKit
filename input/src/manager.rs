use std::collections::HashMap;

use pawkit_interner::InternString;

use crate::{
    DeviceId,
    binding::{
        AnalogBinding, BindingKind, BindingList, DigitalBinding, VectorBinding,
        map::{BindingMap, BindingMapModificaitonError},
    },
    length_squared,
    state::InputState,
};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DigitalInputFrame {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_released: bool,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AnalogInputFrame {
    pub value: f32,
    pub delta: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct VectorInputFrame {
    pub value: [f32; 2],
    pub delta: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union RawInputFrame {
    digital: DigitalInputFrame,
    analog: AnalogInputFrame,
    vector: VectorInputFrame,
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InputFrame {
    Digital(DigitalInputFrame),
    Analog(AnalogInputFrame),
    Vector(VectorInputFrame),
}

pub struct InputManager {
    bindings: BindingMap,
    devices: Vec<DeviceId>,
    frame_indices: im::HashMap<InternString, usize>,
    frames: Box<[RawInputFrame]>,
}

impl InputManager {
    pub fn new(bindings: BindingMap) -> Self {
        let mut frame_indices = HashMap::new();
        let mut frames = vec![];

        for (key, value) in &bindings {
            let index = frames.len();
            frame_indices.insert(key.clone(), index);

            match value {
                BindingList::Digital(_) => {
                    frames.push(RawInputFrame {
                        digital: DigitalInputFrame {
                            pressed: false,
                            just_pressed: false,
                            just_released: false,
                        },
                    });
                }

                BindingList::Analog(_) => {
                    frames.push(RawInputFrame {
                        analog: AnalogInputFrame {
                            value: 0f32,
                            delta: 0f32,
                        },
                    });
                }

                BindingList::Vector(_) => {
                    frames.push(RawInputFrame {
                        vector: VectorInputFrame {
                            value: [0f32; 2],
                            delta: [0f32; 2],
                        },
                    });
                }
            }
        }

        return Self {
            bindings,
            devices: vec![],
            frame_indices: frame_indices.into(),
            frames: frames.into(),
        };
    }

    pub fn connect_device(&mut self, device: DeviceId) {
        if self.devices.contains(&device) {
            return;
        }

        self.devices.push(device);
    }

    pub fn disconnect_device(&mut self, device: DeviceId) {
        self.devices.retain(|it| *it != device);
    }

    pub fn update(&mut self, state: &InputState) {
        for (name, index) in &self.frame_indices {
            let frame = &mut self.frames[*index];

            let Some(bindings) = self.bindings.get_bindings(name) else {
                continue;
            };

            match bindings {
                BindingList::Digital(bindings) => {
                    // SAFETY: We constructed the raw frame with the same type that the given name is
                    let frame = unsafe { &mut frame.digital };

                    let mut pressed = false;

                    for device in &self.devices {
                        let Some(true) = state.get_digital(&device, bindings) else {
                            continue;
                        };

                        pressed = true;
                        break;
                    }

                    let was_pressed = frame.pressed;

                    frame.pressed = pressed;
                    frame.just_pressed = !was_pressed && pressed;
                    frame.just_released = was_pressed && !pressed;
                }

                BindingList::Analog(bindings) => {
                    // SAFETY: We constructed the raw frame with the same type that the given name is
                    let frame = unsafe { &mut frame.analog };

                    let mut value = 0f32;

                    for device in &self.devices {
                        let Some(analog) = state.get_analog(&device, bindings) else {
                            continue;
                        };

                        value = value.max(analog);
                    }

                    let old_value = frame.value;

                    frame.value = value;
                    frame.delta = value - old_value;
                }

                BindingList::Vector(bindings) => {
                    // SAFETY: We constructed the raw frame with the same type that the given name is
                    let frame = unsafe { &mut frame.vector };

                    let mut value = [0f32; 2];
                    let mut value_len_sqr = 0f32;

                    for device in &self.devices {
                        let Some(analog) = state.get_vector(&device, bindings) else {
                            continue;
                        };

                        let analog_len_sqr = length_squared(analog);

                        if analog_len_sqr > value_len_sqr {
                            value = analog;
                            value_len_sqr = analog_len_sqr;
                        }
                    }

                    let old_value = frame.value;

                    frame.value = value;
                    frame.delta = [old_value[0] - value[0], old_value[1] - value[1]];
                }
            }
        }
    }

    pub unsafe fn get_binding_raw(&self, name: &InternString) -> Option<RawInputFrame> {
        let index = self.frame_indices.get(name)?;

        return Some(self.frames[*index]);
    }

    pub fn get_binding(&self, name: &InternString) -> Option<InputFrame> {
        let index = self.frame_indices.get(name)?;

        let frame = &self.frames[*index];

        match self.bindings.get_binding_kind(name)? {
            BindingKind::Digital => return Some(InputFrame::Digital(unsafe { frame.digital })),
            BindingKind::Analog => return Some(InputFrame::Analog(unsafe { frame.analog })),
            BindingKind::Vector => return Some(InputFrame::Vector(unsafe { frame.vector })),
        }
    }

    pub fn add_digital_binding(
        &mut self,
        name: InternString,
        value: DigitalBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        return self.bindings.add_digital_binding(name, value);
    }

    pub fn remove_digital_binding(
        &mut self,
        name: InternString,
        value: DigitalBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        return self.bindings.remove_digital_binding(name, value);
    }

    pub fn add_analog_binding(
        &mut self,
        name: InternString,
        value: AnalogBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        return self.bindings.add_analog_binding(name, value);
    }

    pub fn remove_analog_binding(
        &mut self,
        name: InternString,
        value: AnalogBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        return self.bindings.remove_analog_binding(name, value);
    }

    pub fn add_vector_binding(
        &mut self,
        name: InternString,
        value: VectorBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        return self.bindings.add_vector_binding(name, value);
    }

    pub fn remove_vector_binding(
        &mut self,
        name: InternString,
        value: VectorBinding,
    ) -> Result<(), BindingMapModificaitonError> {
        return self.bindings.remove_vector_binding(name, value);
    }
}
