#![feature(decl_macro)]

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod native;

use std::ffi::c_char;

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

fn cstr_to_str(cstr: *const c_char) -> Option<&'static str> {
    use std::ffi::CStr;
    if cstr.is_null() {
        return None;
    }

    unsafe {
        let cstr = CStr::from_ptr(cstr);
        match cstr.to_str() {
            Ok(str) => Some(str),
            Err(_) => None,
        }
    }
}

#[no_mangle]
extern "C" fn pawkit_logger_info(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    info(message);
}

#[no_mangle]
extern "C" fn pawkit_logger_debug(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    debug(message);
}

#[no_mangle]
extern "C" fn pawkit_logger_warn(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    warn(message);
}

#[no_mangle]
extern "C" fn pawkit_logger_error(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    error(message);
}

#[no_mangle]
extern "C" fn pawkit_logger_fatal(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    fatal(message);
}
