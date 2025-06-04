#![feature(decl_macro)]

use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};

use crate::{
    binding_map::{BindingList, BindingMap, DefaultBindingMap},
    bindings::{
        AnalogBinding, AnalogBindingKind, BoundAxis, BoundButton, DigitalBinding, VectorBinding,
        VectorBindingKind,
    },
    manager::{InputDeviceManager, InputFamily},
};

pub mod binding_map;
pub mod bindings;
pub mod manager;

pub struct InputManager {
    pub bindings: DefaultBindingMap,
    pub keyboard_manager: InputDeviceManager,
    pub mouse_manager: InputDeviceManager,
    pub gamepad_manager: InputDeviceManager,
}

impl InputManager {
    pub fn new() -> Self {
        return Self {
            bindings: DefaultBindingMap::new(),
            keyboard_manager: InputDeviceManager::new(InputFamily::Keyboard),
            mouse_manager: InputDeviceManager::new(InputFamily::Mouse),
            gamepad_manager: InputDeviceManager::new(InputFamily::Gamepad),
        };
    }

    pub fn create_handler(&self) -> InputHandler<'_> {
        let bindings = self
            .bindings
            .new_instance()
            .expect("The binding map should be locked by now.");

        let mut frames = Vec::with_capacity(bindings.values.len());

        for binding in &bindings.values {
            let frame = match binding {
                BindingList::Analog(_) => InputFrame::Analog {
                    value: 0f32,
                    delta: 0f32,
                },

                BindingList::Digital(_) => InputFrame::Digital {
                    value: false,
                    just_pressed: false,
                    just_released: false,
                },

                BindingList::Vector(_) => InputFrame::Vector {
                    value: (0f32, 0f32),
                    delta: (0f32, 0f32),
                },
            };

            frames.push(frame);
        }

        return InputHandler {
            manager: self,
            bindings,
            connected_keyboards: Vec::new(),
            connected_mice: Vec::new(),
            connected_gamepads: Vec::new(),
            frames: frames.into_boxed_slice(),
        };
    }
}

impl Deref for InputManager {
    type Target = DefaultBindingMap;

    fn deref(&self) -> &Self::Target {
        return &self.bindings;
    }
}

impl DerefMut for InputManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.bindings;
    }
}

#[repr(C, u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum InputFrame {
    Digital {
        value: bool,
        just_pressed: bool,
        just_released: bool,
    },
    Analog {
        value: f32,
        delta: f32,
    },
    Vector {
        value: (f32, f32),
        delta: (f32, f32),
    },
}

/// An `InputHandler`` represents a single input consumer, typically a player.
///
/// It manages its own bindings and keeps track of the devices it's using.
pub struct InputHandler<'a> {
    pub bindings: BindingMap<'a>,
    pub manager: &'a InputManager,
    connected_keyboards: Vec<usize>,
    connected_mice: Vec<usize>,
    connected_gamepads: Vec<usize>,
    frames: Box<[InputFrame]>,
}

impl<'a> InputHandler<'a> {
    fn get_bound_axis<'b, TButton, TAxis>(
        manager: &InputDeviceManager,
        axis: &BoundAxis<TButton, TAxis>,
        connected_devices: &Vec<usize>,
    ) -> f32
    where
        TButton: Sized
            + Debug
            + Clone
            + Copy
            + PartialEq
            + PartialOrd
            + Serialize
            + Deserialize<'b>
            + Into<usize>,
        TAxis: Sized
            + Debug
            + Clone
            + Copy
            + PartialEq
            + PartialOrd
            + Serialize
            + Deserialize<'b>
            + Into<usize>,
    {
        let mut value = 0f32;

        match axis {
            BoundAxis::Analog(axis) => {
                for device in connected_devices {
                    if let Some(device) = manager.get_state(*device) {
                        let analog = device.get_analog((*axis).into());
                        if analog.abs() > value.abs() {
                            value = analog;
                        }
                    }
                }
            }

            BoundAxis::Digital(button) => {
                for device in connected_devices {
                    if let Some(device) = manager.get_state(*device) {
                        let analog = if device.get_digital((*button).into()) {
                            1f32
                        } else {
                            0f32
                        };
                        if analog.abs() > value.abs() {
                            value = analog;
                        }
                    }
                }
            }

            BoundAxis::MultiDigital { negative, positive } => {
                for device in connected_devices {
                    if let Some(device) = manager.get_state(*device) {
                        let pos = if device.get_digital((*positive).into()) {
                            1f32
                        } else {
                            0f32
                        };
                        let neg = if device.get_digital((*negative).into()) {
                            1f32
                        } else {
                            0f32
                        };
                        let analog = pos - neg;
                        if analog.abs() > value.abs() {
                            value = analog;
                        }
                    }
                }
            }
        }

        return value;
    }

    fn get_bound_button<'b, TButton, TAxis>(
        manager: &InputDeviceManager,
        button: &BoundButton<TButton, TAxis>,
        connected_devices: &Vec<usize>,
    ) -> bool
    where
        TButton: Sized
            + Debug
            + Clone
            + Copy
            + PartialEq
            + PartialOrd
            + Serialize
            + Deserialize<'b>
            + Into<usize>,
        TAxis: Sized
            + Debug
            + Clone
            + Copy
            + PartialEq
            + PartialOrd
            + Serialize
            + Deserialize<'b>
            + Into<usize>,
    {
        match button {
            BoundButton::Analog { threshold, axis } => {
                for device in connected_devices {
                    if let Some(device) = manager.get_state(*device) {
                        if device.get_analog((*axis).into()) > *threshold {
                            return true;
                        }
                    }
                }
            }

            BoundButton::Digital(button) => {
                for device in connected_devices {
                    if let Some(device) = manager.get_state(*device) {
                        if device.get_digital((*button).into()) {
                            return true;
                        }
                    }
                }
            }
        }

        return false;
    }

    fn get_binding_digital(&self, button: &DigitalBinding) -> bool {
        return match button {
            DigitalBinding::Gamepad(gamepad) => Self::get_bound_button(
                &self.manager.gamepad_manager,
                gamepad,
                &self.connected_gamepads,
            ),

            DigitalBinding::Keyboard(keyboard) => Self::get_bound_button(
                &self.manager.keyboard_manager,
                keyboard,
                &self.connected_keyboards,
            ),

            DigitalBinding::Mouse(mouse) => {
                Self::get_bound_button(&self.manager.mouse_manager, mouse, &self.connected_mice)
            }
        };
    }

    fn get_binding_analog(&self, axis: &AnalogBinding) -> f32 {
        return match &axis.axis {
            AnalogBindingKind::Gamepad(gamepad) => Self::get_bound_axis(
                &self.manager.gamepad_manager,
                gamepad,
                &self.connected_gamepads,
            ),

            AnalogBindingKind::Keyboard(keyboard) => Self::get_bound_axis(
                &self.manager.keyboard_manager,
                keyboard,
                &self.connected_keyboards,
            ),

            AnalogBindingKind::Mouse(mouse) => {
                Self::get_bound_axis(&self.manager.mouse_manager, mouse, &self.connected_mice)
            }
        };
    }

    fn get_binding_vector(&self, vec: &VectorBinding) -> (f32, f32) {
        return match &vec.axes {
            VectorBindingKind::Gamepad { x, y } => (
                Self::get_bound_axis(&self.manager.gamepad_manager, x, &self.connected_gamepads),
                Self::get_bound_axis(&self.manager.gamepad_manager, y, &self.connected_gamepads),
            ),

            VectorBindingKind::Keyboard { x, y } => (
                Self::get_bound_axis(&self.manager.keyboard_manager, x, &self.connected_keyboards),
                Self::get_bound_axis(&self.manager.keyboard_manager, y, &self.connected_keyboards),
            ),

            VectorBindingKind::Mouse { x, y } => (
                Self::get_bound_axis(&self.manager.mouse_manager, x, &self.connected_mice),
                Self::get_bound_axis(&self.manager.mouse_manager, y, &self.connected_mice),
            ),
        };
    }

    fn vec_len_squared(v: (f32, f32)) -> f32 {
        return v.0 * v.0 + v.1 * v.1;
    }

    fn vec_sub(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        return (a.0 - b.0, a.1 - b.1);
    }

    pub fn update(&mut self) {
        for (index, value) in self.bindings.values.iter().enumerate() {
            let frame = &self.frames[index];

            self.frames[index] = match value {
                BindingList::Analog(bindings) => {
                    let old = if let InputFrame::Analog { value, .. } = frame {
                        *value
                    } else {
                        0f32
                    };
                    let mut value = 0f32;

                    for binding in bindings {
                        let analog = self.get_binding_analog(binding);
                        
                        if analog.abs() > value.abs() {
                            value = analog;
                        }
                    }

                    InputFrame::Analog {
                        value,
                        delta: value - old,
                    }
                }

                BindingList::Digital(bindings) => {
                    let old = if let InputFrame::Digital { value, .. } = frame {
                        *value
                    } else {
                        false
                    };
                    let mut value = false;

                    for binding in bindings {
                        if self.get_binding_digital(binding) {
                            value = true;
                            break;
                        }
                    }

                    InputFrame::Digital {
                        value,
                        just_pressed: value && !old,
                        just_released: !value && old,
                    }
                }

                BindingList::Vector(bindings) => {
                    let old = if let InputFrame::Vector { value, .. } = frame {
                        *value
                    } else {
                        (0f32, 0f32)
                    };
                    let mut value = (0f32, 0f32);

                    for binding in bindings {
                        let vec = self.get_binding_vector(binding);

                        if Self::vec_len_squared(vec) > Self::vec_len_squared(value) {
                            value = vec;
                        }
                    }

                    InputFrame::Vector {
                        value,
                        delta: Self::vec_sub(value, old),
                    }
                }
            };
        }
    }

    pub fn get_frame(&self, name: &str) -> Option<InputFrame> {
        let index = match self.bindings.default.index.get(name) {
            Some(index) => index,
            None => return None,
        };

        return Some(self.frames[*index]);
    }

    pub fn connect_device(&mut self, family: InputFamily, id: usize) {
        match family {
            InputFamily::Gamepad => &mut self.connected_gamepads,
            InputFamily::Keyboard => &mut self.connected_keyboards,
            InputFamily::Mouse => &mut self.connected_mice,
        }
        .push(id);
    }

    pub fn disconnect_device(&mut self, family: InputFamily, id: usize) {
        let vec = match family {
            InputFamily::Gamepad => &mut self.connected_gamepads,
            InputFamily::Keyboard => &mut self.connected_keyboards,
            InputFamily::Mouse => &mut self.connected_mice,
        };

        vec.iter().position(|it| *it == id).map(|it| vec.remove(it));
    }
}
