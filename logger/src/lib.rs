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

pub fn info(s: &str) {
    log_badge!((reset, fg_green), "INFO ", (reset), s, (reset));
}

pub fn debug(s: &str) {
    log_badge!((reset, fg_blue), "DEBUG", (reset), s, (reset));
}

pub fn warn(s: &str) {
    log_badge!((reset, fg_yellow), "WARN ", (reset), s, (reset));
}

pub fn error(s: &str) {
    log_badge!((reset, fg_red), "ERROR", (reset), s, (reset));
}

pub fn fatal(s: &str) {
    log_badge!((reset, fg_red, bold), "FATAL", (), s, (reset));
}
