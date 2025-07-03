use godot::prelude::*;
use pawkit_logger::{LoggerCallbacks, set_logger_callbacks};

pub mod net;

struct PawkitGodot;

#[gdextension]
unsafe impl ExtensionLibrary for PawkitGodot {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Editor {
            set_logger_callbacks(Box::new(GodotLoggerCallbacks));
        }
    }
}

struct GodotLoggerCallbacks;

impl LoggerCallbacks for GodotLoggerCallbacks {
    fn print_to_console(&self, s: &str) {
        godot_print!("{}", s);
    }
}
