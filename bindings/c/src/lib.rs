#![feature(decl_macro, generic_atomic)]

use std::{
    ffi::{c_char, CString},
    ptr,
};

mod fs;
mod input;
mod logger;
mod net;

macro c_enum {
    (
        $ty:ident : $base_ty:ty {
            $($rest:tt),* $(,)?
        }
    ) => {
        type $ty = $base_ty;
        c_enum!(@impl $base_ty, 0, $($rest),*);
    },

    (
        $ty:ident {
            $($rest:tt),* $(,)?
        }
    ) => {
        type $ty = i32;
        c_enum!(@impl i32, 0, $($rest),*);
    },

    (@impl $base_ty:ty, $_idx:expr,) => {},

    (@impl $base_ty:ty, $_idx:expr,
        $name:ident = $val:literal, $($rest:tt),*
    ) => {
        const $name: $base_ty = $val;
        c_enum!(@impl $base_ty, ($val + 1), $($rest),*);
    },

    (@impl $base_ty:ty, $idx:expr,
        $name:ident, $($rest:tt),*
    ) => {
        const $name: $base_ty = $idx;
        c_enum!(@impl $base_ty, ($idx + 1), $($rest),*);
    },

    (@impl $base_ty:ty, $_idx:expr,
        $name:ident = $val:literal
    ) => {
        const $name: $base_ty = $val;
    },

    (@impl $base_ty:ty, $idx:expr,
        $name:ident
    ) => {
        const $name: $base_ty = $idx;
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

unsafe fn move_to_stack<T>(ptr: *mut T) -> Option<T> {
    if !ptr.is_aligned() || ptr.is_null() {
        return None;
    }

    return Some(*Box::from_raw(ptr));
}

unsafe fn ptr_to_slice_mut<'a, T>(ptr: *mut T, size: usize) -> Option<&'a mut [T]> {
    if !ptr.is_aligned() || ptr.is_null() || size == 0 {
        return None;
    }

    return Some(std::slice::from_raw_parts_mut(ptr, size));
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

unsafe fn move_slice_to_heap<T: Copy>(slice: &[T], size: &mut usize) -> *mut T {
    let boxed: Box<[T]> = slice.to_vec().into_boxed_slice();

    *size = boxed.len();

    let ptr = Box::into_raw(boxed) as *mut T;

    return ptr;
}

unsafe fn drop_slice_from_heap<T>(ptr: *mut T, size: usize) {
    if ptr.is_null() || !ptr.is_aligned() || size == 0 {
        return;
    }

    let slice = std::slice::from_raw_parts_mut(ptr, size);
    drop(Box::from_raw(slice));
}

unsafe fn set_if_valid<T>(ptr: *mut T, value: T) {
    let Some(ptr) = ptr_to_ref_mut(ptr) else {
        return;
    };

    *ptr = value;
}

#[no_mangle]
unsafe extern "C" fn pawkit_free_string(str: *const c_char) {
    drop_cstr(str);
}

#[no_mangle]
unsafe extern "C" fn pawkit_free_array(slice: *mut u8, size: usize) {
    drop_slice_from_heap(slice, size);
}
