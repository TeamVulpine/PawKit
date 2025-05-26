use std::ffi::c_char;

use pawkit_logger::{DefaultLoggerCallbacks, LoggerCallbacks};

use crate::{cstr_to_str, disown_str_to_cstr, drop_cstr};

#[repr(C)]
struct CLoggerCallbacks {
    print_to_console: Option<extern "C" fn(*const c_char)>,
    print_to_logfile: Option<extern "C" fn(*const c_char)>,
}

impl LoggerCallbacks for CLoggerCallbacks {
    fn print_to_console(&self, s: &str) {
        let Some(print) = self.print_to_logfile else {
            return;
        };

        unsafe {
            let cstr = disown_str_to_cstr(s);
        
            print(cstr);

            drop_cstr(cstr);
        }
    }

    fn print_to_logfile(&self, s: &str) {
        let Some(print) = self.print_to_logfile else {
            return;
        };

        unsafe {
            let cstr = disown_str_to_cstr(s);
        
            print(cstr);

            drop_cstr(cstr);
        }
    }
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_set_logger_callbacks(callback: CLoggerCallbacks) {
    pawkit_logger::set_logger_callbacks(Box::new(callback));
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_reset_logger_callbacks() {
    pawkit_logger::set_logger_callbacks(Box::new(DefaultLoggerCallbacks));
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_print_to_console(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::print_to_console(message);
}

#[no_mangle]
unsafe extern "C" fn pawkit_logger_print_to_logfile(message: *const c_char) {
    let Some(message) = cstr_to_str(message) else {
        return;
    };
    pawkit_logger::print_to_logfile(message);
}

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
