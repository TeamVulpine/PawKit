use std::ffi::c_char;

use crate::cstr_to_str;

#[no_mangle]
unsafe extern "C" fn pawkit_logger_info(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::info(message);
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_debug(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::debug(message);
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_warn(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::warn(message);
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_error(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::error(message);
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_fatal(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::fatal(message);
}
