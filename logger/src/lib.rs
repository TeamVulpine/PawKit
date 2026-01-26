#![feature(decl_macro)]

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod native;

use std::sync::{LazyLock, RwLock};

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub use native::*;

pub trait LoggerCallbacks: Send + Sync {
    #[allow(unused_variables)]
    fn print_to_console(&self, s: &str) {}
    #[allow(unused_variables)]
    fn print_to_logfile(&self, s: &str) {}
}

static LOGGER_CALLBACKS: LazyLock<RwLock<Box<dyn LoggerCallbacks>>> =
    LazyLock::new(|| RwLock::new(Box::new(DefaultLoggerCallbacks)));

pub fn print_to_console(s: &str) {
    LOGGER_CALLBACKS.read().unwrap().print_to_console(s);
}

pub fn print_to_logfile(s: &str) {
    LOGGER_CALLBACKS.read().unwrap().print_to_logfile(s);
}

pub fn set_logger_callbacks(callback: Box<dyn LoggerCallbacks>) {
    *LOGGER_CALLBACKS.write().unwrap() = callback;
}

macro log_badge($prefix: tt, $badge: tt, $start: tt, $msg: tt, $end: tt) {
    let time = time_string();
    let current_thread = std::thread::current();
    let thread_name = current_thread.name().unwrap_or("unnamed");
    print_to_console(&format!(concat!(ansi!$prefix, $badge, ansi!$start, " {} @ {} | {}\n", ansi!$end), time, thread_name, $msg));
    print_to_logfile(&format!(concat!($badge, " {} @ {} | {}\n"), time, thread_name, $msg));
}

pub fn info(message: &str) {
    log_badge!((reset, fg_green), "[INFO] ", (reset), message, (reset));
}

pub fn debug(message: &str) {
    log_badge!((reset, fg_blue), "[DEBUG]", (reset), message, (reset));
}

pub fn warn(message: &str) {
    log_badge!((reset, fg_yellow), "[WARN] ", (reset), message, (reset));
}

pub fn error(message: &str) {
    log_badge!((reset, fg_red), "[ERROR]", (reset), message, (reset));
}

pub fn fatal(message: &str) {
    log_badge!((reset, fg_red, bold), "[FATAL]", (), message, (reset));
}

pub macro log($f:ident, $first:literal $(, $rest:expr)*) {
    $crate::$f(&format!($first $(, $rest)*));
}
