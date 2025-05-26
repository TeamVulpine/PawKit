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

pub trait LoggerCallback: Send + Sync {
    fn print_to_console(&self, s: &str) {}
    fn print_to_logfile(&self, s: &str) {}
}

static LOGGER_CALLBACK: LazyLock<RwLock<Box<dyn LoggerCallback>>> = LazyLock::new(|| RwLock::new(Box::new(DefaultLoggerCallback)));

pub fn print_to_console(s: &str) {
    LOGGER_CALLBACK.read().unwrap().print_to_console(s);
}

pub fn print_to_logfile(s: &str) {
    LOGGER_CALLBACK.read().unwrap().print_to_logfile(s);
}

pub fn set_logger_callback(callback: Box<dyn LoggerCallback>) {
    *LOGGER_CALLBACK.write().unwrap() = callback;
}

macro log_badge($prefix: tt, $badge: tt, $start: tt, $msg: tt, $end: tt) {
    let time = time_string();
    print_to_console(&format!(concat!(ansi!$prefix, "[", $badge, "]", ansi!$start, " {} | {}\n", ansi!$end), time, $msg));
    print_to_logfile(&format!(concat!("[", $badge, "]", " {} | {}\n"), time, $msg));
}

pub fn info(message: &str) {
    log_badge!((reset, fg_green), "INFO ", (reset), message, (reset));
}

pub fn debug(message: &str) {
    log_badge!((reset, fg_blue), "DEBUG", (reset), message, (reset));
}

pub fn warn(message: &str) {
    log_badge!((reset, fg_yellow), "WARN ", (reset), message, (reset));
}

pub fn error(message: &str) {
    log_badge!((reset, fg_red), "ERROR", (reset), message, (reset));
}

pub fn fatal(message: &str) {
    log_badge!((reset, fg_red, bold), "FATAL", (), message, (reset));
}
