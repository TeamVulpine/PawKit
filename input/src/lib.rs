use crate::{
    binding_map::{BindingMap, DefaultBindingMap},
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
        return InputHandler {
            manager: self,
            bindings: self
                .bindings
                .new_instance()
                .expect("The binding map should be locked by now."),
            connected_keyboards: Vec::new(),
            connected_mice: Vec::new(),
            connected_gamepads: Vec::new(),
        };
    }
}

/// An InputHandler is analogous to a "player".
pub struct InputHandler<'a> {
    pub manager: &'a InputManager,
    bindings: BindingMap<'a>,
    connected_keyboards: Vec<usize>,
    connected_mice: Vec<usize>,
    connected_gamepads: Vec<usize>,
}
