use std::{
    ffi::{c_char, CString},
    ptr,
};

pub mod logger;
pub mod net;

unsafe fn cstr_to_str<'a>(cstr: *const c_char) -> Option<&'a str> {
    use std::ffi::CStr;
    if cstr.is_null() {
        return None;
    }

    let cstr = CStr::from_ptr(cstr);
    return match cstr.to_str() {
        Ok(str) => Some(str),
        Err(_) => None,
    };
}

unsafe fn disown_str_to_cstr(s: &str) -> *const c_char {
    let Ok(c_str) = CString::new(s) else {
        return ptr::null();
    };

    return c_str.into_raw() as *const c_char;
}

unsafe fn drop_cstr(s: *const c_char) {
    if s.is_null() {
        return;
    }

    let _ = CString::from_raw(s as *mut c_char);
}

unsafe fn move_to_heap<T>(value: T) -> *mut T {
    return Box::into_raw(Box::new(value));
}

unsafe fn drop_from_heap<T>(value: *mut T) {
    if !value.is_null() {
        drop(Box::from_raw(value));
    }
}

unsafe fn ptr_to_slice<'a, T>(ptr: *const T, size: usize) -> &'a [T] {
    return std::slice::from_raw_parts(ptr, size);
}
