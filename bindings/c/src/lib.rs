#![feature(decl_macro)]

use std::{
    ffi::{c_char, CString},
    ptr,
};

pub mod input;
pub mod logger;
pub mod net;

macro c_enum {
    (
        $ty:ident : $base_ty:ty {
            $($rest:tt),* $(,)?
        }
    ) => {
        pub type $ty = $base_ty;
        c_enum!(@impl $base_ty, 0, $($rest),*);
    },

    (
        $ty:ident {
            $($rest:tt),* $(,)?
        }
    ) => {
        pub type $ty = i32;
        c_enum!(@impl i32, 0, $($rest),*);
    },

    (@impl $base_ty:ty, $_idx:expr,) => {},

    (@impl $base_ty:ty, $_idx:expr,
        $name:ident = $val:literal, $($rest:tt),*
    ) => {
        pub const $name: $base_ty = $val;
        c_enum!(@impl $base_ty, ($val + 1), $($rest),*);
    },

    (@impl $base_ty:ty, $idx:expr,
        $name:ident, $($rest:tt),*
    ) => {
        pub const $name: $base_ty = $idx;
        c_enum!(@impl $base_ty, ($idx + 1), $($rest),*);
    },

    (@impl $base_ty:ty, $_idx:expr,
        $name:ident = $val:literal
    ) => {
        pub const $name: $base_ty = $val;
    },

    (@impl $base_ty:ty, $idx:expr,
        $name:ident
    ) => {
        pub const $name: $base_ty = $idx;
    },
}

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

unsafe fn drop_from_heap<T>(ptr: *mut T) {
    if !ptr.is_aligned() || ptr.is_null() {
        return;
    }

    drop(Box::from_raw(ptr));
}

unsafe fn ptr_to_slice<'a, T>(ptr: *const T, size: usize) -> Option<&'a [T]> {
    if !ptr.is_aligned() || ptr.is_null() || size == 0 {
        return None;
    }

    return Some(std::slice::from_raw_parts(ptr, size));
}

unsafe fn ptr_to_ref_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    if !ptr.is_aligned() || ptr.is_null() {
        return None;
    }

    return Some(&mut *ptr);
}

unsafe fn ptr_to_ref<'a, T>(ptr: *const T) -> Option<&'a T> {
    if !ptr.is_aligned() || ptr.is_null() {
        return None;
    }

    return Some(&*ptr);
}
