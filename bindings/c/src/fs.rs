use std::{ffi::c_char, io::Read, ptr::null_mut};

use pawkit_fs::{
    Vfs, VfsBuffer, VfsError, VfsListUtils
};

use crate::{
    cstr_to_str, disown_str_to_cstr, drop_from_heap, move_slice_to_heap, move_to_heap,
    move_to_stack, ptr_to_ref, ptr_to_ref_mut, ptr_to_slice, ptr_to_slice_mut,
};

type CVfs = *mut Vfs;
type CVfsBuffer = *mut VfsBuffer;

type CVfsList = *mut Box<dyn Iterator<Item = Result<String, VfsError>>>;

/// Takes ownership of the buffer, since File can't implement Clone.
/// The pointer to the buffer will no longer be valid after calling.
#[no_mangle]
unsafe extern "C" fn pawkit_vfs_zip(buf: CVfsBuffer) -> CVfs {
    let Some(buf) = move_to_stack(buf) else {
        return null_mut();
    };

    let Ok(vfs) = Vfs::zip(buf) else {
        return null_mut();
    };

    return move_to_heap(vfs);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_working() -> CVfs {
    let Ok(vfs) = Vfs::working() else {
        return null_mut();
    };

    return move_to_heap(vfs);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_subdirectory(vfs: CVfs, subdirectory: *const c_char) -> CVfs {
    let Some(vfs) = ptr_to_ref(vfs) else {
        return null_mut();
    };

    let Some(subdirectory) = cstr_to_str(subdirectory) else {
        return null_mut();
    };

    let Ok(vfs) = vfs.subdirectory(subdirectory) else {
        return null_mut();
    };

    return move_to_heap(vfs);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_destroy(vfs: CVfs) {
    drop_from_heap(vfs);
}

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
unsafe extern "C" fn pawkit_vfs_open(vfs: CVfs, path: *const c_char) -> CVfsBuffer {
    let Some(vfs) = ptr_to_ref(vfs) else {
        return null_mut();
    };

    let Some(path) = cstr_to_str(path) else {
        return null_mut();
    };

    let Ok(buf) = vfs.open(path) else {
        return null_mut();
    };

    return move_to_heap(buf);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_read(buf: CVfsBuffer, data: *mut u8, size: usize) -> usize {
    let Some(buf) = ptr_to_ref_mut(buf) else {
        return 0;
    };

    let Some(data) = ptr_to_slice_mut(data, size) else {
        return 0;
    };

    return buf.read(data).unwrap_or(0);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_read_to_array(
    buf: CVfsBuffer,
    size: *mut usize,
) -> *const u8 {
    let Some(buf) = ptr_to_ref_mut(buf) else {
        return null_mut();
    };

    let Some(size) = ptr_to_ref_mut(size) else {
        return null_mut();
    };

    let mut data = vec![];

    let Ok(_) = buf.read_to_end(&mut data) else {
        return null_mut();
    };

    return move_slice_to_heap(&data, size);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_buffer_read_to_string(buf: CVfsBuffer) -> *const c_char {
    let Some(buf) = ptr_to_ref_mut(buf) else {
        return null_mut();
    };

    let mut data = String::new();

    let Ok(_) = buf.read_to_string(&mut data) else {
        return null_mut();
    };

    return disown_str_to_cstr(&data);
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_subdirectories(vfs: CVfs) -> CVfsList {
    let Some(vfs) = ptr_to_ref(vfs) else {
        return null_mut();
    };

    let Ok(list) = vfs.list_subdirectories() else {
        return null_mut();
    };

    return move_to_heap(Box::new(list));
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_files(vfs: CVfs) -> CVfsList {
    let Some(vfs) = ptr_to_ref(vfs) else {
        return null_mut();
    };

    let Ok(list) = vfs.list_files() else {
        return null_mut();
    };

    return move_to_heap(Box::new(list));
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_files_recursive(vfs: CVfs) -> CVfsList {
    let Some(vfs) = ptr_to_ref(vfs) else {
        return null_mut();
    };

    let Ok(list) = vfs.list_files_recursive() else {
        return null_mut();
    };

    return move_to_heap(Box::new(list));
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_destroy(list: CVfsList) {
    drop_from_heap(list);
}

/// Takes ownership of the list.
/// The pointer to the buffer will no longer be valid after calling.
#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_with_extension(list: CVfsList, extension: *const c_char) -> CVfsList {
    let Some(value) = move_to_stack(list) else {
        return null_mut();
    };

    let Some(extension) = cstr_to_str(extension) else {
        return null_mut();
    };

    return move_to_heap(Box::new(value.with_extension(extension)));
}

#[no_mangle]
unsafe extern "C" fn pawkit_vfs_list_next(list: CVfsList) -> *const c_char {
    let Some(value) = ptr_to_ref_mut(list) else {
        return null_mut();
    };

    let Some(Ok(result)) = value.next() else {
        return null_mut();
    };

    return disown_str_to_cstr(&result);
}
