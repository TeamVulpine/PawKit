use std::{ffi::c_char, io::Read, ptr::null_mut};

use pawkit_fs::{Vfs, VfsBuffer, VfsError, VfsListUtils};
use zip::result::ZipError;

use crate::{
    c_enum, cstr_to_str, disown_str_to_cstr, drop_from_heap, move_slice_to_heap, move_to_heap,
    move_to_stack, ptr_to_ref, ptr_to_ref_mut, ptr_to_slice, ptr_to_slice_mut, set_if_valid,
};

type CVfs = *mut Vfs;
type CVfsBuffer = *mut VfsBuffer;

type CVfsList = *mut Box<dyn Iterator<Item = Result<String, VfsError>>>;

c_enum!(CVfsError : u8 {
    ERROR_OK,
    ERROR_INVALID_PTR,
    ERROR_IO,
    ERROR_ZIP,
    ERROR_NOT_FOUND,
    ERROR_OTHER,
});

fn vfs_error_to_c(error: VfsError) -> CVfsError {
    return match error {
        VfsError::IoError(_) | VfsError::ZipError(ZipError::Io(_)) => ERROR_IO,
        VfsError::NotFound | VfsError::ZipError(ZipError::FileNotFound) => ERROR_NOT_FOUND,
        VfsError::ZipError(_) => ERROR_ZIP,
        _ => ERROR_OTHER,
    };
}

unsafe fn ok(error: *mut CVfsError) {
    unsafe {
        set_if_valid(error, ERROR_OK);
    }
}

unsafe fn result_to_option<T>(res: Result<T, VfsError>, error: *mut CVfsError) -> Option<T> {
    unsafe {
        return match res {
            Ok(value) => Some(value),
            Err(err) => {
                set_if_valid(error, vfs_error_to_c(err));

                return None;
            }
        };
    }
}

/// Takes ownership of the buffer, since File can't implement Clone.
/// The pointer to the buffer will no longer be valid after calling.
#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_zip(buf: CVfsBuffer, error: *mut CVfsError) -> CVfs {
    unsafe {
        let Some(buf) = move_to_stack(buf) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(vfs) = result_to_option(Vfs::zip(buf), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(vfs);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_working(error: *mut CVfsError) -> CVfs {
    unsafe {
        let Some(vfs) = result_to_option(Vfs::working(), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(vfs);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_subdirectory(
    vfs: CVfs,
    subdirectory: *const c_char,
    subdirectory_len: usize,
    error: *mut CVfsError,
) -> CVfs {
    unsafe {
        let Some(vfs) = ptr_to_ref(vfs) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(subdirectory) = cstr_to_str(subdirectory, subdirectory_len) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(vfs) = result_to_option(vfs.subdirectory(subdirectory), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(vfs);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_free(vfs: CVfs) {
    unsafe {
        drop_from_heap(vfs);
    }
}

/// The only error here would be ERROR_INVALID_PTR so it can be omitted.
#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_buffer_from_bytes(ptr: *mut u8, size: usize) -> CVfsBuffer {
    unsafe {
        let Some(data) = ptr_to_slice(ptr, size) else {
            return null_mut();
        };

        return move_to_heap(data.into());
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_buffer_free(buf: CVfsBuffer) {
    unsafe {
        drop_from_heap(buf);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_open(
    vfs: CVfs,
    path: *const c_char,
    path_len: usize,
    error: *mut CVfsError,
) -> CVfsBuffer {
    unsafe {
        let Some(vfs) = ptr_to_ref(vfs) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(path) = cstr_to_str(path, path_len) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(buf) = result_to_option(vfs.open(path), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(buf);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_buffer_read(
    buf: CVfsBuffer,
    data: *mut u8,
    size: usize,
    error: *mut CVfsError,
) -> usize {
    unsafe {
        let Some(buf) = ptr_to_ref_mut(buf) else {
            set_if_valid(error, ERROR_INVALID_PTR);
            return 0;
        };

        let Some(data) = ptr_to_slice_mut(data, size) else {
            set_if_valid(error, ERROR_INVALID_PTR);
            return 0;
        };

        ok(error);

        return buf.read(data).unwrap_or(0);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_buffer_read_to_array(
    buf: CVfsBuffer,
    size: *mut usize,
    error: *mut CVfsError,
) -> *const u8 {
    unsafe {
        let Some(buf) = ptr_to_ref_mut(buf) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(size) = ptr_to_ref_mut(size) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let mut data = vec![];

        let Ok(_) = buf.read_to_end(&mut data) else {
            set_if_valid(error, ERROR_IO);

            return null_mut();
        };

        ok(error);

        return move_slice_to_heap(&data, size);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_buffer_read_to_string(
    buf: CVfsBuffer,
    len: *mut usize,
    error: *mut CVfsError,
) -> *const c_char {
    unsafe {
        let Some(buf) = ptr_to_ref_mut(buf) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(len) = ptr_to_ref_mut(len) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let mut data = String::new();

        let Ok(_) = buf.read_to_string(&mut data) else {
            set_if_valid(error, ERROR_IO);

            return null_mut();
        };

        ok(error);

        return disown_str_to_cstr(&data, len);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_list_subdirectories(vfs: CVfs, error: *mut CVfsError) -> CVfsList {
    unsafe {
        let Some(vfs) = ptr_to_ref(vfs) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(list) = result_to_option(vfs.list_subdirectories(), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(Box::new(list));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_list_files(vfs: CVfs, error: *mut CVfsError) -> CVfsList {
    unsafe {
        let Some(vfs) = ptr_to_ref(vfs) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(list) = result_to_option(vfs.list_files(), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(Box::new(list));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_list_files_recursive(vfs: CVfs, error: *mut CVfsError) -> CVfsList {
    unsafe {
        let Some(vfs) = ptr_to_ref(vfs) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(list) = result_to_option(vfs.list_files_recursive(), error) else {
            return null_mut();
        };

        ok(error);

        return move_to_heap(Box::new(list));
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_list_free(list: CVfsList) {
    unsafe {
        drop_from_heap(list);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_list_with_extension(
    list: CVfsList,
    extension: *const c_char,
    extension_len: usize,
    error: *mut CVfsError,
) {
    unsafe {
        let Some(value) = ptr_to_ref_mut(list) else {
            set_if_valid(error, ERROR_INVALID_PTR);
            return;
        };

        let Some(extension) = cstr_to_str(extension, extension_len) else {
            set_if_valid(error, ERROR_INVALID_PTR);
            return;
        };

        let old_iter = std::mem::replace(value, Box::new(std::iter::empty()));
        let new_iter = old_iter.with_extension(extension);

        *value = Box::new(new_iter);

        ok(error);
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn pawkit_vfs_list_next(
    list: CVfsList,
    len: *mut usize,
    error: *mut CVfsError,
) -> *const c_char {
    unsafe {
        let Some(value) = ptr_to_ref_mut(list) else {
            return null_mut();
        };

        let Some(len) = ptr_to_ref_mut(len) else {
            set_if_valid(error, ERROR_INVALID_PTR);

            return null_mut();
        };

        let Some(result) = value.next() else {
            *len = 0;

            // The result was None instead of Some(Err) so it's technically OK.
            ok(error);

            return null_mut();
        };

        let Some(result) = result_to_option(result, error) else {
            return null_mut();
        };

        ok(error);

        return disown_str_to_cstr(&result, len);
    }
}
