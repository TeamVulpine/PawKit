#pragma once

#include "assert.h"
#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void *pawkit_vfs_t;
typedef void *pawkit_vfs_buffer_t;
typedef void *pawkit_vfs_list_t;

/// Takes ownership of the buffer, the pointer will no longer be valid
pawkit_vfs_t pawkit_vfs_zip(pawkit_vfs_buffer_t buf);

pawkit_vfs_t pawkit_vfs_working();

pawkit_vfs_t pawkit_vfs_subdirectory(pawkit_vfs_t vfs, char const *subdirectory);

void pawkit_vfs_destroy(pawkit_vfs_t vfs);

pawkit_vfs_buffer_t pawkit_vfs_buffer_from_bytes(pawkit_u8 *ptr, pawkit_usize size);

void pawkit_vfs_buffer_destroy(pawkit_vfs_buffer_t buf);

pawkit_vfs_buffer_t pawkit_vfs_open(pawkit_vfs_t vfs, char const *path);

pawkit_usize pawkit_vfs_buffer_read(pawkit_vfs_buffer_t buf, pawkit_u8 *data, pawkit_usize size);

uint8_t const *pawkit_vfs_buffer_read_to_array(pawkit_vfs_buffer_t buf, pawkit_usize *size);

char const *pawkit_vfs_buffer_read_to_string(pawkit_vfs_buffer_t buf);

pawkit_vfs_list_t pawkit_vfs_list_subdirectories(pawkit_vfs_t vfs);

pawkit_vfs_list_t pawkit_vfs_list_files(pawkit_vfs_t vfs);

pawkit_vfs_list_t pawkit_vfs_list_files_recursive(pawkit_vfs_t vfs);

/// Takes ownership of the list. The pointer will no longer be valid after calling.
pawkit_vfs_list_t pawkit_vfs_list_with_extension(pawkit_vfs_list_t list, char const *extension);

char const *pawkit_vfs_list_next(pawkit_vfs_list_t list);

void pawkit_vfs_list_destroy(pawkit_vfs_list_t list);

#ifdef __cplusplus
}
#endif
