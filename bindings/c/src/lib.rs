use std::ffi::c_char;

pub mod logger;

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
