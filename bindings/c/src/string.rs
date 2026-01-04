use std::{ffi::c_char, mem::forget, ptr::null};

use pawkit_interner::InternString;

use crate::{cstr_to_str, ptr_to_ref_mut};

#[unsafe(no_mangle)]
pub extern "C" fn pawkit_string_from(cstr: *const c_char, len: usize) -> *const u8 {
    unsafe {
        let Some(str) = cstr_to_str(cstr, len) else {
            return null();
        };

        return InternString::new(str).into_raw();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pawkit_string_addref(string: *const u8) {
    unsafe {
        let Some(string) = InternString::from_raw(string) else {
            return;
        };
        forget(string.clone());
        forget(string);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pawkit_string_remref(string: *const u8) {
    unsafe {
        drop(InternString::from_raw(string));
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn pawkit_string_get(raw: *const u8, size: *mut usize) -> *const c_char {
    unsafe {
        let Some(string) = InternString::from_raw(raw) else {
            return null();
        };

        let Some(size) = ptr_to_ref_mut(size) else {
            return null();
        };

        *size = string.len();

        // InternString's raw pointer is defined as the start of the string data.
        return raw as *const c_char;
    }
}
