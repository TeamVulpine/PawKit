use std::ffi::c_char;

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

unsafe fn move_to_heap<T>(value: T) -> *mut T {
    return Box::into_raw(Box::new(value));
}

unsafe fn free_from_heap<T>(value: *mut T) {
    if !value.is_null() {
        drop(Box::from_raw(value));
    }
}

unsafe fn ptr_to_slice<'a, T>(ptr: *const T, size: usize) -> &'a [T] {
    return std::slice::from_raw_parts(ptr, size);
}
