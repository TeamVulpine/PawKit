use std::ffi::c_char;

use pawkit_logger::{DefaultLoggerCallbacks, LoggerCallbacks};

use crate::{cstr_to_str, str_to_cstr};

#[repr(C)]
struct CLoggerCallbacks {
    print_to_console: Option<extern "C" fn(*const c_char, usize)>,
    print_to_logfile: Option<extern "C" fn(*const c_char, usize)>,
}

impl LoggerCallbacks for CLoggerCallbacks {
    fn print_to_console(&self, s: &str) {
        let Some(print) = self.print_to_console else {
            return;
        };

        unsafe {
            let mut len: usize = 0;
            let cstr = str_to_cstr(s, &mut len);

            print(cstr, len);
        }
    }

    fn print_to_logfile(&self, s: &str) {
        let Some(print) = self.print_to_logfile else {
            return;
        };

        unsafe {
            let mut len: usize = 0;
            let cstr = str_to_cstr(s, &mut len);

            print(cstr, len);
        }
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_set_logger_callbacks(callback: CLoggerCallbacks) {
    pawkit_logger::set_logger_callbacks(Box::new(callback));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_reset_logger_callbacks() {
    pawkit_logger::set_logger_callbacks(Box::new(DefaultLoggerCallbacks));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_print_to_console(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::print_to_console(message);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_print_to_logfile(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::print_to_logfile(message);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_info(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::info(message);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_debug(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::debug(message);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_warn(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::warn(message);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_error(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::error(message);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_logger_fatal(message: *const c_char, message_len: usize) {
    unsafe {
        let Some(message) = cstr_to_str(message, message_len) else {
            return;
        };
        pawkit_logger::fatal(message);
    }
}
