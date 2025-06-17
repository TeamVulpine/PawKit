#![feature(decl_macro)]

use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Deref, DerefMut},
    sync::RwLock,
};

use pawkit_holy_array::HolyArray;
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

pub struct InputDeviceManagers {
    pub keyboard_manager: InputDeviceManager,
    pub mouse_manager: InputDeviceManager,
    pub gamepad_manager: InputDeviceManager,
}

impl InputDeviceManagers {
    pub fn new() -> Self {
        return Self {
            keyboard_manager: InputDeviceManager::new(InputFamily::Keyboard),
            mouse_manager: InputDeviceManager::new(InputFamily::Mouse),
            gamepad_manager: InputDeviceManager::new(InputFamily::Gamepad),
        };
    }
}

pub struct InputManager {
    pub bindings: DefaultBindingMap,
    pub devices: InputDeviceManagers,
    handlers: RwLock<HolyArray<InputHandler>>,
}

impl InputManager {
    pub fn new() -> Self {
        return Self {
            bindings: DefaultBindingMap::new(),
            devices: InputDeviceManagers::new(),
            handlers: RwLock::new(HolyArray::new()),
        };
    }

    pub fn create_handler(&self) -> Option<usize> {
        let index = self.bindings.new_instance().ok()?;

        let bindings = self.bindings.get_map(index)?;

        let mut frames = Vec::with_capacity(bindings.values.len());

        for binding in &bindings.values {
            let frame = match &*binding {
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

            frames.push(RwLock::new(frame));
        }

        let Ok(mut handlers) = self.handlers.write() else {
            return None;
        };

        return Some(handlers.acquire(InputHandler {
            connected_keyboards: Vec::new(),
            connected_mice: Vec::new(),
            connected_gamepads: Vec::new(),
            frames: frames.into_boxed_slice(),
        }));
    }

    pub fn destroy_handler(&self, device: usize) {
        let Ok(mut handlers) = self.handlers.write() else {
            return;
        };

        handlers.release(device);
    }

    pub fn update(&self) {
        let Ok(handlers) = self.handlers.read() else {
            return;
        };

        for i in 0..handlers.len() {
            let Some(handler) = handlers.get(i) else {
                continue;
            };

            let Some(map) = self.bindings.get_map(i) else {
                continue;
            };

            handler.update(&self.devices, &map);
        }
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
pub struct InputHandler {
    connected_keyboards: Vec<usize>,
    connected_mice: Vec<usize>,
    connected_gamepads: Vec<usize>,
    frames: Box<[RwLock<InputFrame>]>,
}

impl InputHandler {
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

    fn get_binding_digital(&self, managers: &InputDeviceManagers, button: &DigitalBinding) -> bool {
        return match button {
            DigitalBinding::Gamepad(gamepad) => {
                Self::get_bound_button(&managers.gamepad_manager, gamepad, &self.connected_gamepads)
            }

            DigitalBinding::Keyboard(keyboard) => Self::get_bound_button(
                &managers.keyboard_manager,
                keyboard,
                &self.connected_keyboards,
            ),

            DigitalBinding::Mouse(mouse) => {
                Self::get_bound_button(&managers.mouse_manager, mouse, &self.connected_mice)
            }
        };
    }

    fn get_binding_analog(&self, managers: &InputDeviceManagers, axis: &AnalogBinding) -> f32 {
        return match &axis.axis {
            AnalogBindingKind::Gamepad(gamepad) => {
                Self::get_bound_axis(&managers.gamepad_manager, gamepad, &self.connected_gamepads)
            }

            AnalogBindingKind::Keyboard(keyboard) => Self::get_bound_axis(
                &managers.keyboard_manager,
                keyboard,
                &self.connected_keyboards,
            ),

            AnalogBindingKind::Mouse(mouse) => {
                Self::get_bound_axis(&managers.mouse_manager, mouse, &self.connected_mice)
            }
        };
    }

    fn get_binding_vector(
        &self,
        managers: &InputDeviceManagers,
        vec: &VectorBinding,
    ) -> (f32, f32) {
        return match &vec.axes {
            VectorBindingKind::Gamepad { x, y } => (
                Self::get_bound_axis(&managers.gamepad_manager, x, &self.connected_gamepads),
                Self::get_bound_axis(&managers.gamepad_manager, y, &self.connected_gamepads),
            ),

            VectorBindingKind::Keyboard { x, y } => (
                Self::get_bound_axis(&managers.keyboard_manager, x, &self.connected_keyboards),
                Self::get_bound_axis(&managers.keyboard_manager, y, &self.connected_keyboards),
            ),

            VectorBindingKind::Mouse { x, y } => (
                Self::get_bound_axis(&managers.mouse_manager, x, &self.connected_mice),
                Self::get_bound_axis(&managers.mouse_manager, y, &self.connected_mice),
            ),
        };
    }

    fn vec_len_squared(v: (f32, f32)) -> f32 {
        return v.0 * v.0 + v.1 * v.1;
    }

    fn vec_sub(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
        return (a.0 - b.0, a.1 - b.1);
    }

    pub fn update(&self, managers: &InputDeviceManagers, bindings: &BindingMap) {
        for (index, value) in bindings.values.iter().enumerate() {
            let frame = &self.frames[index];

            match &*value {
                BindingList::Analog(bindings) => {
                    let Ok(bindings) = bindings.read() else {
                        continue;
                    };

                    let Ok(mut frame) = frame.write() else {
                        continue;
                    };

                    let old = if let InputFrame::Analog { value, .. } = &*frame {
                        *value
                    } else {
                        0f32
                    };
                    let mut value = 0f32;

                    for binding in &*bindings {
                        let analog = self.get_binding_analog(managers, binding);

                        if analog.abs() > value.abs() {
                            value = analog;
                        }
                    }

                    *frame = InputFrame::Analog {
                        value,
                        delta: value - old,
                    };
                }

                BindingList::Digital(bindings) => {
                    let Ok(bindings) = bindings.read() else {
                        continue;
                    };

                    let Ok(mut frame) = frame.write() else {
                        continue;
                    };

                    let old = if let InputFrame::Digital { value, .. } = &*frame {
                        *value
                    } else {
                        false
                    };
                    let mut value = false;

                    for binding in &*bindings {
                        if self.get_binding_digital(managers, binding) {
                            value = true;
                            break;
                        }
                    }

                    *frame = InputFrame::Digital {
                        value,
                        just_pressed: value && !old,
                        just_released: !value && old,
                    };
                }

                BindingList::Vector(bindings) => {
                    let Ok(bindings) = bindings.read() else {
                        continue;
                    };

                    let Ok(mut frame) = frame.write() else {
                        continue;
                    };

                    let old = if let InputFrame::Vector { value, .. } = &*frame {
                        *value
                    } else {
                        (0f32, 0f32)
                    };
                    let mut value = (0f32, 0f32);

                    for binding in &*bindings {
                        let vec = self.get_binding_vector(managers, binding);

                        if Self::vec_len_squared(vec) > Self::vec_len_squared(value) {
                            value = vec;
                        }
                    }

                    *frame = InputFrame::Vector {
                        value,
                        delta: Self::vec_sub(value, old),
                    };
                }
            };
        }
    }

    pub fn get_frame(&self, index: HashMap<String, usize>, name: &str) -> Option<InputFrame> {
        let index = match index.get(name) {
            Some(index) => index,
            None => return None,
        };

        return Some(self.frames[*index].read().ok()?.clone());
    }

    pub fn connect_device(&mut self, family: InputFamily, id: usize) {
        match family {
            InputFamily::Gamepad => &mut self.connected_gamepads,
            InputFamily::Keyboard => &mut self.connected_keyboards,
            InputFamily::Mouse => &mut self.connected_mice,
        }
        .push(id);
    }

    pub fn connect_device_raw(
        &mut self,
        managers: &InputDeviceManagers,
        family: InputFamily,
        id: usize,
    ) {
        let Some(id) = match family {
            InputFamily::Gamepad => &managers.gamepad_manager,
            InputFamily::Keyboard => &managers.keyboard_manager,
            InputFamily::Mouse => &managers.mouse_manager,
        }
        .raw_id_to_id(id) else {
            return;
        };

        self.connect_device(family, id);
    }

    pub fn disconnect_device(&mut self, family: InputFamily, id: usize) {
        let vec = match family {
            InputFamily::Gamepad => &mut self.connected_gamepads,
            InputFamily::Keyboard => &mut self.connected_keyboards,
            InputFamily::Mouse => &mut self.connected_mice,
        };

        vec.iter().position(|it| *it == id).map(|it| vec.remove(it));
    }

    pub fn disconnect_device_raw(
        &mut self,
        managers: &InputDeviceManagers,
        family: InputFamily,
        id: usize,
    ) {
        let Some(id) = match family {
            InputFamily::Gamepad => &managers.gamepad_manager,
            InputFamily::Keyboard => &managers.keyboard_manager,
            InputFamily::Mouse => &managers.mouse_manager,
        }
        .raw_id_to_id(id) else {
            return;
        };

        self.disconnect_device(family, id);
    }
}
