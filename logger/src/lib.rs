#![feature(decl_macro)]

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod native;

#[cfg(target_arch = "wasm32")]
use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
use native::*;

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
