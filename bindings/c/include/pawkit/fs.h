#pragma once

#include "assert.h"
#include "util.h"
#include <string_view>

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

pawkit_vfs_t pawkit_vfs_subdirectory(pawkit_vfs_t vfs, char const *subdirectory, pawkit_usize subdirectory_len, pawkit_vfs_error_t *error);

void pawkit_vfs_destroy(pawkit_vfs_t vfs);

/// The only error would be PAWKIT_VFS_ERROR_INVALID_PTR so it is omitted.
pawkit_vfs_buffer_t pawkit_vfs_buffer_from_bytes(pawkit_u8 const *ptr, pawkit_usize size);

void pawkit_vfs_buffer_destroy(pawkit_vfs_buffer_t buf);

pawkit_vfs_buffer_t pawkit_vfs_open(pawkit_vfs_t vfs, char const *path, pawkit_usize path_len, pawkit_vfs_error_t *error);

pawkit_usize pawkit_vfs_buffer_read(pawkit_vfs_buffer_t buf, pawkit_u8 *data, pawkit_usize size, pawkit_vfs_error_t *error);

uint8_t const *pawkit_vfs_buffer_read_to_array(pawkit_vfs_buffer_t buf, pawkit_usize *size, pawkit_vfs_error_t *error);

char const *pawkit_vfs_buffer_read_to_string(pawkit_vfs_buffer_t buf, pawkit_usize *size, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_subdirectories(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_files(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

pawkit_vfs_list_t pawkit_vfs_list_files_recursive(pawkit_vfs_t vfs, pawkit_vfs_error_t *error);

void pawkit_vfs_list_with_extension(pawkit_vfs_list_t list, char const *extension, pawkit_vfs_error_t *error);

char const *pawkit_vfs_list_next(pawkit_vfs_list_t list, pawkit_usize *size, pawkit_vfs_error_t *error);

void pawkit_vfs_list_destroy(pawkit_vfs_list_t list);

#ifdef __cplusplus
}

#include <optional>
#include <span>
#include <string>
#include <vector>
#include <expected>

namespace PawKit::Vfs {
    template <typename T>
    using Result = std::expected<T, pawkit_vfs_error_t>;
    
    /// Represents a virtual file buffer.
    /// It can be represented by an actual file buffer, a zip file buffer, or a byte array.
    struct Buffer final {
        ~Buffer() {
            pawkit_vfs_buffer_destroy(*this);
        };

        Buffer() = delete;
        Buffer(Buffer const &copy) = delete;
        Buffer(Buffer &&move) = delete;

        operator pawkit_vfs_buffer_t () {
            return static_cast<pawkit_vfs_buffer_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static Buffer *From(pawkit_vfs_buffer_t buf) {
            return static_cast<Buffer *>(buf);
        }

        /// Creates a buffer from a byte array
        static Buffer *FromBytes(std::span<pawkit_u8 const> bytes) {
            return From(pawkit_vfs_buffer_from_bytes(bytes.data(), bytes.size()));
        }

        /// Reads a number of bytes from the buffer
        Result<pawkit_usize> Read(std::span<pawkit_u8> buffer) {
            pawkit_vfs_error_t err = 0;

            pawkit_usize ok = pawkit_vfs_buffer_read(*this, buffer.data(), buffer.size(), &err);

            if (err) 
                return std::unexpected(err);

            return ok;
        }

        /// Reads the buffer to an array
        Result<std::vector<pawkit_usize>> ReadToArray() {
            pawkit_vfs_error_t err = 0;
            pawkit_usize len = 0;
            pawkit_u8 const *data = pawkit_vfs_buffer_read_to_array(*this, &len, &err);

            if (err) 
                return std::unexpected(err);

            std::vector<pawkit_usize> arr {data, data + len};

            pawkit_free_array(data, len);

            return arr;
        }

        /// Reads the buffer to a string
        Result<std::string> ReadToString() {
            pawkit_vfs_error_t err = 0;
            pawkit_usize len;
            char const *data = pawkit_vfs_buffer_read_to_string(*this, &len, &err);

            if (err) 
                return std::unexpected(err);

            std::string arr {data, data + len};

            pawkit_free_string(data, len);

            return arr;
        }
    };

    /// Represents a "list" operation for the VFS
    struct List final {
        ~List() {
            pawkit_vfs_list_destroy(*this);
        };

        List() = delete;
        List(List const &copy) = delete;
        List(List &&move) = delete;

        operator pawkit_vfs_list_t () {
            return static_cast<pawkit_vfs_list_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static List *From(pawkit_vfs_list_t list) {
            return static_cast<List *>(list);
        }

        /// Gets the next file in the list
        Result<std::optional<std::string>> Next() {
            pawkit_vfs_error_t err = 0;
            pawkit_usize len;
            char const *filename = pawkit_vfs_list_next(*this, &len, &err);

            if (err)
                return std::unexpected(err);

            std::string arr {filename, filename + len};

            pawkit_free_string(filename, len);

            return arr;
        }

        /// Only look for files with a given extension
        Result<List *> WithExtension(std::string const &ext) {
            pawkit_vfs_error_t err = 0;
            pawkit_vfs_list_with_extension(*this, ext.c_str(), &err);

            if (err)
                return std::unexpected(err);

            return this;
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
                
                if (!result) {
                    current = std::unexpected(result.error());
                    list = nullptr;
                } else {
                    std::optional<std::string> &opt = *result;
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
    struct Filesystem final {
        ~Filesystem() {
            pawkit_vfs_destroy(*this);
        };

        Filesystem() = delete;
        Filesystem(Filesystem const &copy) = delete;
        Filesystem(Filesystem &&move) = delete;

        operator pawkit_vfs_t () {
            return static_cast<pawkit_vfs_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static Filesystem *From(pawkit_vfs_t fs) {
            return static_cast<Filesystem *>(fs);
        }
        
        /// Gets the virtual filesystem associated with the current working directory
        static Result<Filesystem *> Working() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_t vfs = pawkit_vfs_working(&error);

            if (error)
                return std::unexpected(error);

            return From(vfs);
        }
        
        /// Takes ownership of the buffer, the pointer will no longer be valid.
        static Result<Filesystem *> Zip(Buffer *buf) {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_t vfs = pawkit_vfs_zip(*buf, &error);

            if (error)
                return std::unexpected(error);

            return From(vfs);
        }

        /// Gets a subdirectory of the filesystem as a new filesystem
        Result<Filesystem *> Subdirectory(std::string_view path) {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_t vfs = pawkit_vfs_subdirectory(*this, path.data(), path.size(), &error);

            if (error)
                return std::unexpected(error);

            return From(vfs);
        }

        /// Lists all the top-level subdirectories of the filesystem
        Result<List *> ListSubdirectories() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_subdirectories(*this,  &error);

            if (error)
                return std::unexpected(error);

            return List::From(list);
        }

        /// Lists all the top-level files of the filesystem
        Result<List *> ListFiles() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_files(*this,  &error);

            if (error)
                return std::unexpected(error);

            return List::From(list);
        }

        /// Lists all the files of the filesystem recursively
        Result<List *> ListFilesRecursive() {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_files_recursive(*this,  &error);

            if (error)
                return std::unexpected(error);

            return List::From(list);
        }

        /// Opens the file at the given path
        Result<Buffer *> Open(std::string_view path) {
            pawkit_vfs_error_t error = 0;

            pawkit_vfs_buffer_t buf = pawkit_vfs_open(*this, path.data(), path.length(), &error);

            if (error)
                return std::unexpected(error);

            return Buffer::From(buf);
        }
    };
}

#endif
