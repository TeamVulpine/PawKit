#![feature(decl_macro)]

use std::{fmt::Debug, vec};

use serde::{Deserialize, Serialize};

use crate::{
    binding_map::{BindingList, BindingMap, DefaultBindingMap},
    bindings::{
        AnalogBinding, AnalogBindingKind, BoundAxis, BoundButton, DefaultBinding, DigitalBinding, VectorBinding, VectorBindingKind
    },
    manager::{InputDeviceManager, InputFamily},
};

pub mod binding_map;
pub mod bindings;
pub mod manager;

pub struct InputManager {
    bindings: DefaultBindingMap,
    pub keyboard_manager: InputDeviceManager,
    pub mouse_manager: InputDeviceManager,
    pub gamepad_manager: InputDeviceManager,
}

impl InputManager {
    pub fn new(mut bindings: DefaultBindingMap) -> Self {
        // Lock the bindings. The user is expected to fill in the map before passing it into the manager.
        bindings.lock();

        Self {
            bindings,
            keyboard_manager: InputDeviceManager::new(InputFamily::Keyboard),
            mouse_manager: InputDeviceManager::new(InputFamily::Mouse),
            gamepad_manager: InputDeviceManager::new(InputFamily::Gamepad),
        }
    }

    pub fn create_handler(&self) -> InputHandler<'_> {
        let bindings = self
            .bindings
            .new_instance()
            .expect("The binding map should be locked by now.");

        let mut frames = vec![];

        frames.reserve(bindings.values.len());

        for binding in &bindings.values {
            let value = match binding {
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

            frames.push(value);
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

#[repr(C)]
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
/// It manages it's own bindings, and keeps track of the devices it's using.
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
                let axis = *axis;

                for device in connected_devices {
                    let Some(device) = manager.get_state(*device) else {
                        continue;
                    };

                    let analog = device.get_analog(axis.into());

                    if analog.abs() > value.abs() {
                        value = analog;
                    }
                }
            }

            BoundAxis::Digital(button) => {
                let button = *button;

                for device in connected_devices {
                    let Some(device) = manager.get_state(*device) else {
                        continue;
                    };

                    let analog = if device.get_digital(button.into()) {
                        1f32
                    } else {
                        0f32
                    };

                    if analog.abs() > value.abs() {
                        value = analog;
                    }
                }
            }

            BoundAxis::MultiDigital { negative, positive } => {
                let negative = *negative;
                let positive = *positive;

                for device in connected_devices {
                    let Some(device) = manager.get_state(*device) else {
                        continue;
                    };

                    let positive = if device.get_digital(positive.into()) {
                        1f32
                    } else {
                        0f32
                    };

                    let negative = if device.get_digital(negative.into()) {
                        1f32
                    } else {
                        0f32
                    };

                    let analog = positive - negative;

                    if analog.abs() > value.abs() {
                        value = analog;
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
                let axis = *axis;

                for device in connected_devices {
                    let Some(device) = manager.get_state(*device) else {
                        continue;
                    };

                    if device.get_analog(axis.into()) > *threshold {
                        return true;
                    }
                }
            }

            BoundButton::Digital(button) => {
                let button = *button;

                for device in connected_devices {
                    let Some(device) = manager.get_state(*device) else {
                        continue;
                    };

                    if device.get_digital(button.into()) {
                        return true;
                    }
                }
            }
        }

        return false;
    }

    fn get_binding_digital(&self, button: &DigitalBinding) -> bool {
        match button {
            DigitalBinding::Gamepad(gamepad) => {
                return Self::get_bound_button(
                    &self.manager.gamepad_manager,
                    gamepad,
                    &self.connected_gamepads,
                );
            }

            DigitalBinding::Keyboard(keyboard) => {
                return Self::get_bound_button(
                    &self.manager.keyboard_manager,
                    keyboard,
                    &self.connected_keyboards,
                );
            }

            DigitalBinding::Mouse(mouse) => {
                return Self::get_bound_button(
                    &self.manager.mouse_manager,
                    mouse,
                    &self.connected_mice,
                );
            }
        }
    }

    fn get_binding_analog(&self, axis: &AnalogBinding) -> f32 {
        match &axis.axis {
            AnalogBindingKind::Gamepad(gamepad) => {
                return Self::get_bound_axis(
                    &self.manager.gamepad_manager,
                    gamepad,
                    &self.connected_gamepads,
                );
            }

            AnalogBindingKind::Keyboard(keyboard) => {
                return Self::get_bound_axis(
                    &self.manager.keyboard_manager,
                    keyboard,
                    &self.connected_keyboards,
                );
            }

            AnalogBindingKind::Mouse(mouse) => {
                return Self::get_bound_axis(
                    &self.manager.mouse_manager,
                    mouse,
                    &self.connected_mice,
                );
            }
        }
    }

    fn get_binding_vector(&self, vec: &VectorBinding) -> (f32, f32) {
        match &vec.axes {
            VectorBindingKind::Gamepad { x, y } => {
                return (
                    Self::get_bound_axis(
                        &self.manager.gamepad_manager,
                        x,
                        &self.connected_gamepads,
                    ),
                    Self::get_bound_axis(
                        &self.manager.gamepad_manager,
                        y,
                        &self.connected_gamepads,
                    ),
                );
            }

            VectorBindingKind::Keyboard { x, y } => {
                return (
                    Self::get_bound_axis(
                        &self.manager.keyboard_manager,
                        x,
                        &self.connected_keyboards,
                    ),
                    Self::get_bound_axis(
                        &self.manager.keyboard_manager,
                        y,
                        &self.connected_keyboards,
                    ),
                );
            }

            VectorBindingKind::Mouse { x, y } => {
                return (
                    Self::get_bound_axis(&self.manager.mouse_manager, x, &self.connected_mice),
                    Self::get_bound_axis(&self.manager.mouse_manager, y, &self.connected_mice),
                );
            }
        }
    }

    fn vec_len_squared(v: (f32, f32)) -> f32 {
        return v.0 * v.0 + v.1 * v.1;
    }
    
    fn vec_sub(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        return (a.0 - b.0, a.1 - b.1);
    }

    pub fn update(&mut self) {
        for (index, value) in self.bindings.values.iter().enumerate() {
            // SAFETY: `self.frames` has the same size as `self.bindings.values`.
            let frame = unsafe { self.frames.get_unchecked(index) };

            match value {
                BindingList::Analog(bindings) => {
                    let InputFrame::Analog {
                        value: old_value,
                        ..
                    } = *frame
                    else {
                        continue;
                    };

                    let mut value = 0f32;
                    for binding in bindings {
                        let analog = self.get_binding_analog(binding);

                        if analog.abs() > value.abs() {
                            value = analog;
                        } 
                    }

                    // SAFETY: `self.frames` has the same size as `self.bindings.values`.
                    *unsafe { self.frames.get_unchecked_mut(index) } = InputFrame::Analog {
                        value,
                        delta: value - old_value,
                    };
                }

                BindingList::Digital(bindings) => {
                    let InputFrame::Digital {
                        value: old_value,
                        ..
                    } = *frame
                    else {
                        continue;
                    };

                    let mut value = false;
                    for binding in bindings {
                        let digital = self.get_binding_digital(binding);

                        if digital {
                            value = true;
                            break;
                        } 
                    }

                    // SAFETY: `self.frames` has the same size as `self.bindings.values`.
                    *unsafe { self.frames.get_unchecked_mut(index) } = InputFrame::Digital {
                        value,
                        just_pressed: value && !old_value,
                        just_released: !value && old_value,
                    };
                }

                BindingList::Vector(bindings) => {
                    let InputFrame::Vector {
                        value: old_value,
                        ..
                    } = *frame
                    else {
                        continue;
                    };

                    let mut value = (0f32, 0f32);
                    for binding in bindings {
                        let vector = self.get_binding_vector(binding);

                        if Self::vec_len_squared(vector) > Self::vec_len_squared(value) {
                            value = vector;
                        } 
                    }

                    // SAFETY: `self.frames` has the same size as `self.bindings.values`.
                    *unsafe { self.frames.get_unchecked_mut(index) } = InputFrame::Vector {
                        value,
                        delta: Self::vec_sub(value, old_value),
                    };
                }
            }
        }
    }

    pub fn get_frame(&self, name: &str) -> Option<&InputFrame> {
        let index = *self.bindings.default.index.get(name)?;

        // SAFETY: `self.frames` has the same size as `self.bindings.values`,
        // and `self.bindings.default.index` only contains indices into that slice.
        unsafe {
            return Some(self.frames.get_unchecked(index));
        }
    }
    
    pub fn connect_device(&mut self, family: InputFamily, id: usize) {
        match family {
            InputFamily::Gamepad => &mut self.connected_gamepads,
            InputFamily::Keyboard => &mut self.connected_keyboards,
            InputFamily::Mouse => &mut self.connected_mice,
        }.push(id);
    }

    pub fn disconnect_device(&mut self, family: InputFamily, id: usize) {
        let vec = match family {
            InputFamily::Gamepad => &mut self.connected_gamepads,
            InputFamily::Keyboard => &mut self.connected_keyboards,
            InputFamily::Mouse => &mut self.connected_mice,
        };

        let Some(index) = vec.iter().position(|it| *it == id) else {
            return;
        };

        vec.remove(index);
    }
}
