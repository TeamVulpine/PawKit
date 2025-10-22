#![feature(decl_macro, generic_atomic)]

use core::slice;
use std::{
    alloc::{Layout, alloc, dealloc},
    ffi::c_char,
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

unsafe fn move_to_heap<T>(value: T) -> *mut T {
    return Box::into_raw(Box::new(value));
}

unsafe fn drop_from_heap<T>(ptr: *mut T) {
    unsafe {
        if !ptr.is_aligned() || ptr.is_null() {
            return;
        }

        drop(Box::from_raw(ptr));
    }
}

unsafe fn move_to_stack<T>(ptr: *mut T) -> Option<T> {
    unsafe {
        if !ptr.is_aligned() || ptr.is_null() {
            return None;
        }

        return Some(*Box::from_raw(ptr));
    }
}

unsafe fn ptr_to_slice_mut<'a, T>(ptr: *mut T, size: usize) -> Option<&'a mut [T]> {
    unsafe {
        if !ptr.is_aligned() || ptr.is_null() || size == 0 {
            return None;
        }

        return Some(std::slice::from_raw_parts_mut(ptr, size));
    }
}

unsafe fn ptr_to_slice<'a, T>(ptr: *const T, size: usize) -> Option<&'a [T]> {
    unsafe {
        if !ptr.is_aligned() || ptr.is_null() || size == 0 {
            return None;
        }

        return Some(std::slice::from_raw_parts(ptr, size));
    }
}

unsafe fn ptr_to_ref_mut<'a, T>(ptr: *mut T) -> Option<&'a mut T> {
    unsafe {
        if !ptr.is_aligned() || ptr.is_null() {
            return None;
        }

        return Some(&mut *ptr);
    }
}

unsafe fn ptr_to_ref<'a, T>(ptr: *const T) -> Option<&'a T> {
    unsafe {
        if !ptr.is_aligned() || ptr.is_null() {
            return None;
        }

        return Some(&*ptr);
    }
}

unsafe fn move_slice_to_heap<T: Copy>(slice: &[T], size: &mut usize) -> *mut T {
    if *size == 0 {
        return std::ptr::null_mut();
    }

    unsafe {
        let layout = Layout::array::<T>(*size).unwrap();

        let ptr = alloc(layout) as *mut T;

        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        ptr::copy_nonoverlapping(slice.as_ptr(), ptr, *size);

        return ptr;
    }
}

unsafe fn drop_slice_from_heap<T>(ptr: *mut T, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }

    let layout = Layout::array::<T>(size).unwrap();

    unsafe {
        dealloc(ptr as *mut u8, layout);
    }
}

unsafe fn set_if_valid<T>(ptr: *mut T, value: T) {
    unsafe {
        let Some(ptr) = ptr_to_ref_mut(ptr) else {
            return;
        };

        *ptr = value;
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_free_array(slice: *mut u8, size: usize) {
    unsafe {
        drop_slice_from_heap(slice, size);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_free_string(str: *const c_char, size: usize) {
    unsafe {
        drop_slice_from_heap(str as *mut u8, size);
    }
}

unsafe fn cstr_to_str<'a>(cstr: *const c_char, len: usize) -> Option<&'a str> {
    if cstr.is_null() {
        return None;
    }

    let bytes = unsafe { slice::from_raw_parts(cstr as *const u8, len) };

    return std::str::from_utf8(bytes).ok();
}

unsafe fn str_to_cstr<'a>(s: &'a str, len: &mut usize) -> *const c_char {
    *len = s.len();
    return s.as_ptr() as *const c_char;
}

unsafe fn disown_str_to_cstr(s: &str, len: &mut usize) -> *const c_char {
    unsafe {
        return move_slice_to_heap(s.as_bytes(), len) as *const c_char;
    }
}
