#pragma once

#include "assert.h"
#include "util.h"
#include <type_traits>
#include <utility>

#ifdef __cplusplus
extern "C" {
#endif

typedef void *pawkit_vfs_t;
typedef void *pawkit_vfs_buffer_t;
typedef void *pawkit_vfs_list_t;

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

pawkit_vfs_t pawkit_vfs_subdirectory(pawkit_vfs_t vfs, char const *subdirectory, pawkit_vfs_error_t *error);

void pawkit_vfs_destroy(pawkit_vfs_t vfs);

/// The only error would be PAWKIT_VFS_ERROR_INVALID_PTR so it is omitted.
pawkit_vfs_buffer_t pawkit_vfs_buffer_from_bytes(pawkit_u8 const *ptr, pawkit_usize size);

void pawkit_vfs_buffer_destroy(pawkit_vfs_buffer_t buf);

pawkit_vfs_buffer_t pawkit_vfs_open(pawkit_vfs_t vfs, char const *path, pawkit_vfs_error_t *error);

pawkit_usize pawkit_vfs_buffer_read(pawkit_vfs_buffer_t buf, pawkit_u8 *data, pawkit_usize size, pawkit_vfs_error_t *error);

uint8_t const *pawkit_vfs_buffer_read_to_array(pawkit_vfs_buffer_t buf, pawkit_usize *size, pawkit_vfs_error_t *error);

char const *pawkit_vfs_buffer_read_to_string(pawkit_vfs_buffer_t buf, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_subdirectories(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_files(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_files_recursive(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

/// Takes ownership of the list. The pointer will no longer be valid after calling.
/// The only error would be PAWKIT_VFS_ERROR_INVALID_PTR so it is omitted.
pawkit_vfs_list_t pawkit_vfs_list_with_extension(pawkit_vfs_list_t list, char const *extension);

char const *pawkit_vfs_list_next(pawkit_vfs_list_t list, pawkit_vfs_error_t *error);

void pawkit_vfs_list_destroy(pawkit_vfs_list_t list);

#ifdef __cplusplus
}

#include <optional>
#include <span>
#include <string>
#include <vector>

namespace PawKit::Vfs {
    template <typename T>
    struct Result final {
        pawkit_vfs_error_t err {0};
        std::optional<T> ok {std::nullopt};

        template <typename ...TParams>
            requires std::is_constructible_v<T, TParams...>
        Result(TParams &&...params) : ok(std::forward<TParams>(params)...) {}
        Result(pawkit_vfs_error_t error) : err(error) {}

        bool IsOk() {
            return ok.has_value();
        }

        bool IsErr() {
            return !IsOk();
        }

        T &Unwrap() {
            return *ok;
        }

        pawkit_vfs_error_t UnwrapErr() {
            return err;
        }
    };

    // template <typename T>
    // using Result = std::variant<T, pawkit_vfs_error_t>;

    /// Represents a virtual file buffer.
    /// It can be represented by an actual file buffer, a zip file buffer, or a byte array.
    struct Buffer : OpaqueShared<pawkit_vfs_buffer_t> {
        Buffer(pawkit_vfs_buffer_t buf) : OpaqueShared(buf, pawkit_vfs_buffer_destroy) {}

        /// Creates a buffer from a byte array
        static std::optional<Buffer> FromBytes(std::span<pawkit_u8 const> bytes) {
            pawkit_vfs_buffer_t buf = pawkit_vfs_buffer_from_bytes(bytes.data(), bytes.size());

            if (!buf)
                return std::nullopt;

            return buf;
        }

        // Reads up to the size of the input buffer in bytes, returns the number of bytes consumed
        Result<pawkit_usize> Read(std::span<pawkit_u8> buffer) {
            pawkit_vfs_error_t err = 0;

            pawkit_usize ok = pawkit_vfs_buffer_read(Get(), buffer.data(), buffer.size(), &err);

            if (err) 
                return err;

            return ok;
        }

        /// Reads the buffer to a vector
        Result<std::vector<pawkit_u8>> ReadToArray() {
            return GetBufErr<pawkit_vfs_error_t, Result<std::vector<pawkit_u8>>>([&](pawkit_usize &data, pawkit_vfs_error_t &err) {
                return pawkit_vfs_buffer_read_to_array(Get(), &data, &err);
            });
        }

        /// Reads the buffer to a string
        Result<std::string> ReadToString() {
            return GetStringErr<pawkit_vfs_error_t, Result<std::string>>([&](pawkit_vfs_error_t &err) {
                return pawkit_vfs_buffer_read_to_string(Get(), &err);
            });
        }
    };

    /// Represents a "list" operation for the VFS
    struct List : OpaqueUnique<pawkit_vfs_list_t> {
        List(pawkit_vfs_list_t list) : OpaqueUnique(list, pawkit_vfs_list_destroy) {}

        /// Gets the next file in the list
        Result<std::optional<std::string>> Next() {
            return GetStringErrOptional<pawkit_vfs_error_t, Result<std::optional<std::string>>>([&](pawkit_vfs_error_t &err) {
                return pawkit_vfs_list_next(Get(), &err);
            });
        }

        /// Only look for files with a given extension
        List &WithExtension(std::string const &ext) {
            Reset(pawkit_vfs_list_with_extension(Release(), ext.c_str()));
            
            return *this;
        }

        struct Iterator {
            List *list = nullptr;
            std::optional<Result<std::string>> current;

            Iterator(List *l) : list(l) {
                ++(*this);
            }

            Iterator() = default;

            Result<std::string> const &operator*() const {
                return *current;
            }

            Iterator &operator++() {
                if (!list) {
                    current = std::nullopt;
                    return *this;
                }

                Result<std::optional<std::string>> result = list->Next();
                if (result.IsErr()) {
                    current = result.UnwrapErr();
                    list = nullptr;
                } else {
                    std::optional<std::string> &opt = result.Unwrap();
                    if (!opt.has_value()) {
                        current = std::nullopt;
                        list = nullptr;
                    } else {
                        current = Result<std::string>(std::string(*opt));
                    }
                }

                return *this;
            }

            bool operator!=(Iterator const &other) const {
                return current.has_value() != other.current.has_value();
            }
        };

        Iterator begin() {
            return Iterator(this);
        }

        Iterator end() {
            return Iterator();
        }
    };

    /// Represents a virtual filesystem.
    /// A virtual file system represent the real filesystem, a zip file, or a subdirectory of them.
    struct Filesystem : OpaqueShared<pawkit_vfs_t> {
        Filesystem(pawkit_vfs_t ptr) : OpaqueShared(ptr, pawkit_vfs_destroy) {}

        /// Gets the virtual filesystem associated with the current working directory
        static Result<Filesystem> Working() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_t vfs = pawkit_vfs_working(&error);

            if (error)
                return Result<Filesystem>(error);

            return Result<Filesystem>(Filesystem(vfs));
        }

        /// Creates a virtual filesystem from the contents of a zip file
        static Result<Filesystem> Zip(Buffer buffer) {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_t vfs = pawkit_vfs_zip(buffer.Get(), &error);

            if (error)
                return Result<Filesystem>(error);

            return Result<Filesystem>(Filesystem(vfs));
        }

        /// Gets a subdirectory of the filesystem as a new filesystem
        Result<Filesystem> Subdirectory(std::string const &path) {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_t vfs = pawkit_vfs_subdirectory(Get(), path.c_str(), &error);

            if (error)
                return Result<Filesystem>(error);

            return Result<Filesystem>(Filesystem(vfs));
        }

        /// Lists all the top-level subdirectories of the filesystem
        Result<List> ListSubdirectories() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_subdirectories(Get(),  &error);

            if (error)
                return Result<List>(error);

            return Result<List>(list);
        }

        /// Lists all the top-level files of the filesystem
        Result<List> ListFiles() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_files(Get(),  &error);

            if (error)
                return Result<List>(error);

            return Result<List>(list);
        }

        /// Lists all the files of the filesystem recursively
        Result<List> ListFilesRecursive() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_files_recursive(Get(),  &error);

            if (error)
                return Result<List>(error);

            return Result<List>(list);
        }

        /// Opens the file at the given path
        Result<Buffer> Open(std::string const &path) {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_buffer_t buf = pawkit_vfs_open(Get(), path.c_str(), &error);
            
            if (error)
                return Result<Buffer>(error);

            return Result<Buffer>(buf);
        }
    };
}

#endif
