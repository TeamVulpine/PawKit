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
    set_if_valid(error, ERROR_OK);
}

unsafe fn result_to_option<T>(res: Result<T, VfsError>, error: *mut CVfsError) -> Option<T> {
    return match res {
        Ok(value) => Some(value),
        Err(err) => {
            set_if_valid(error, vfs_error_to_c(err));

            return None;
        }
    };
}

/// Takes ownership of the buffer, since File can't implement Clone.
/// The pointer to the buffer will no longer be valid after calling.
#[no_mangle]
unsafe extern "C" fn pawkit_vfs_zip(buf: CVfsBuffer, error: *mut CVfsError) -> CVfs {
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

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_working(error: *mut CVfsError) -> CVfs {
    let Some(vfs) = result_to_option(Vfs::working(), error) else {
        return null_mut();
    };

    ok(error);

    return move_to_heap(vfs);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_subdirectory(
    vfs: CVfs,
    subdirectory: *const c_char,
    error: *mut CVfsError,
) -> CVfs {
    let Some(vfs) = ptr_to_ref(vfs) else {
        set_if_valid(error, ERROR_INVALID_PTR);

        return null_mut();
    };

    let Some(subdirectory) = cstr_to_str(subdirectory) else {
        set_if_valid(error, ERROR_INVALID_PTR);

        return null_mut();
    };

    let Some(vfs) = result_to_option(vfs.subdirectory(subdirectory), error) else {
        return null_mut();
    };

    ok(error);

    return move_to_heap(vfs);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_destroy(vfs: CVfs) {
    drop_from_heap(vfs);
}

/// The only error here would be ERROR_INVALID_PTR so it can be omitted.
#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_from_bytes(ptr: *mut u8, size: usize) -> CVfsBuffer {
    let Some(data) = ptr_to_slice(ptr, size) else {
        return null_mut();
    };

    return move_to_heap(data.into());
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_destroy(buf: CVfsBuffer) {
    drop_from_heap(buf);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_open(
    vfs: CVfs,
    path: *const c_char,
    error: *mut CVfsError,
) -> CVfsBuffer {
    let Some(vfs) = ptr_to_ref(vfs) else {
        set_if_valid(error, ERROR_INVALID_PTR);

        return null_mut();
    };

    let Some(path) = cstr_to_str(path) else {
        set_if_valid(error, ERROR_INVALID_PTR);

        return null_mut();
    };

    let Some(buf) = result_to_option(vfs.open(path), error) else {
        return null_mut();
    };

    ok(error);

    return move_to_heap(buf);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_read(
    buf: CVfsBuffer,
    data: *mut u8,
    size: usize,
    error: *mut CVfsError,
) -> usize {
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

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_read_to_array(
    buf: CVfsBuffer,
    size: *mut usize,
    error: *mut CVfsError,
) -> *const u8 {
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

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_read_to_string(
    buf: CVfsBuffer,
    error: *mut CVfsError,
) -> *const c_char {
    let Some(buf) = ptr_to_ref_mut(buf) else {
        set_if_valid(error, ERROR_INVALID_PTR);

        return null_mut();
    };

    let mut data = String::new();

    let Ok(_) = buf.read_to_string(&mut data) else {
        set_if_valid(error, ERROR_IO);

        return null_mut();
    };

    ok(error);

    return disown_str_to_cstr(&data);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_subdirectories(vfs: CVfs, error: *mut CVfsError) -> CVfsList {
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

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_files(vfs: CVfs, error: *mut CVfsError) -> CVfsList {
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

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_files_recursive(vfs: CVfs, error: *mut CVfsError) -> CVfsList {
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

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_destroy(list: CVfsList) {
    drop_from_heap(list);
}

/// Takes ownership of the list.
/// The pointer to the buffer will no longer be valid after calling.
/// The only error here would be ERROR_INVALID_PTR so it can be omitted
#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_with_extension(
    list: CVfsList,
    extension: *const c_char,
) -> CVfsList {
    let Some(value) = move_to_stack(list) else {
        return null_mut();
    };

    let Some(extension) = cstr_to_str(extension) else {
        return null_mut();
    };

    return move_to_heap(Box::new(value.with_extension(extension)));
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_next(list: CVfsList, error: *mut CVfsError) -> *const c_char {
    let Some(value) = ptr_to_ref_mut(list) else {
        return null_mut();
    };

    let Some(result) = value.next() else {
        // The result was None instead of Some(Err) so it's technically OK.
        ok(error);

        return null_mut();
    };

    let Some(result) = result_to_option(result, error) else {
        return null_mut();
    };

    ok(error);

    return disown_str_to_cstr(&result);
}
