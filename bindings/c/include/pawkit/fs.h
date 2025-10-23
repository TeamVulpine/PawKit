#pragma once

#include "assert.h"
#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct pawkit_vfs *pawkit_vfs_t;
typedef struct pawkit_vfs_buffer *pawkit_vfs_buffer_t;
typedef struct pawkit_vfs_list *pawkit_vfs_list_t;

enum {
    PAWKIT_VFS_ERROR_OK,
    PAWKIT_VFS_ERROR_INVALID_PTR,
    PAWKIT_VFS_ERROR_IO,
    PAWKIT_VFS_ERROR_ZIP,
    PAWKIT_VFS_ERROR_NOT_FOUND,
    PAWKIT_VFS_ERROR_OTHER,
};
typedef pawkit_u8 pawkit_vfs_error_t;

/// Takes ownership of the buffer, the pointer will no longer be valid
pawkit_vfs_t pawkit_vfs_zip(pawkit_vfs_buffer_t buf, pawkit_vfs_error_t *error);

pawkit_vfs_t pawkit_vfs_working(pawkit_vfs_error_t *error);

pawkit_vfs_t pawkit_vfs_subdirectory(pawkit_vfs_t vfs, char const *subdirectory, pawkit_usize subdirectory_size, pawkit_vfs_error_t *error);

void pawkit_vfs_free(pawkit_vfs_t vfs);

/// The only error would be PAWKIT_VFS_ERROR_INVALID_PTR so it is omitted.
pawkit_vfs_buffer_t pawkit_vfs_buffer_from_bytes(pawkit_u8 const *ptr, pawkit_usize size);

void pawkit_vfs_buffer_free(pawkit_vfs_buffer_t buf);

pawkit_vfs_buffer_t pawkit_vfs_open(pawkit_vfs_t vfs, char const *path, pawkit_usize path_size, pawkit_vfs_error_t *error);

pawkit_usize pawkit_vfs_buffer_read(pawkit_vfs_buffer_t buf, pawkit_u8 *data, pawkit_usize size, pawkit_vfs_error_t *error);

uint8_t const *pawkit_vfs_buffer_read_to_array(pawkit_vfs_buffer_t buf, pawkit_usize *size, pawkit_vfs_error_t *error);

char const *pawkit_vfs_buffer_read_to_string(pawkit_vfs_buffer_t buf, pawkit_usize *size, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_subdirectories(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_files(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_files_recursive(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

void pawkit_vfs_list_with_extension(pawkit_vfs_list_t list, char const *extension, pawkit_vfs_error_t *error);

char const *pawkit_vfs_list_next(pawkit_vfs_list_t list, pawkit_usize *size, pawkit_vfs_error_t *error);

void pawkit_vfs_list_free(pawkit_vfs_list_t list);

#ifdef __cplusplus
}
#endif
