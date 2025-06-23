#pragma once

#include "assert.h"
#include "util.h"

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
#include <utility>
#include <variant>
#include <vector>

namespace PawKit::Vfs {
    template <typename T>
    using Result = std::variant<T, pawkit_vfs_error_t>;

    struct Buffer : OpaqueUnique<pawkit_vfs_buffer_t> {
        friend struct Filesystem;
        friend Result<Buffer>;

        Buffer(pawkit_vfs_buffer_t buf) : OpaqueUnique(buf, pawkit_vfs_buffer_destroy) {}

        static std::optional<Buffer> FromBytes(std::span<pawkit_u8 const> bytes) {
            pawkit_vfs_buffer_t buf = pawkit_vfs_buffer_from_bytes(bytes.data(), bytes.size());

            if (!buf)
                return std::nullopt;

            return buf;
        }

        Result<pawkit_usize> Read(std::span<pawkit_u8> buffer) {
            pawkit_vfs_error_t err = 0;

            pawkit_usize ok = pawkit_vfs_buffer_read(Get(), buffer.data(), buffer.size(), &err);

            if (err) 
                return err;

            return ok;
        }

        Result<std::vector<pawkit_u8>> ReadToArray() {
            return GetBufErr<pawkit_vfs_error_t>([&](pawkit_usize &data, pawkit_vfs_error_t &err) {
                return pawkit_vfs_buffer_read_to_array(Get(), &data, &err);
            });
        }

        Result<std::string> ReadToString() {
            return GetStringErr<pawkit_vfs_error_t>([&](pawkit_vfs_error_t &err) {
                return pawkit_vfs_buffer_read_to_string(Get(), &err);
            });
        }
    };

    struct List : OpaqueUnique<pawkit_vfs_list_t> {
        friend struct Filesystem;

        private:
        List(pawkit_vfs_list_t list) : OpaqueUnique(list, pawkit_vfs_buffer_destroy) {}

        public:
        Result<std::optional<std::string>> Next() {
            return GetStringErrOptional<pawkit_vfs_error_t>([&](pawkit_vfs_error_t &err) {
                return pawkit_vfs_list_next(Get(), &err);
            });
        }

        List *WithExtension(std::string const &ext) {
            Reset(pawkit_vfs_list_with_extension(Release(), ext.c_str()));

            return this;
        }

        struct Iterator {
            List *list = nullptr;
            std::optional<Result<std::string>> current;

            Iterator(List* l) : list(l) {
                ++(*this);
            }

            Iterator() = default;

            std::optional<Result<std::string>> const& operator*() const {
                return current;
            }

            Iterator& operator++() {
                if (!list) {
                    current = std::nullopt;
                    return *this;
                }

                Result<std::optional<std::string>> result = list->Next();
                if (std::holds_alternative<pawkit_vfs_error_t>(result)) {
                    current = std::get<1>(result);
                    list = nullptr;
                } else {
                    std::optional<std::string> &opt = std::get<0>(result);
                    if (!opt.has_value()) {
                        current = std::nullopt;
                        list = nullptr;
                    } else {
                        current = *opt;
                    }
                }

                return *this;
            }

            bool operator!=(Iterator const& other) const {
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

    struct Filesystem : OpaqueShared<pawkit_vfs_t> {
        private:
        Filesystem(pawkit_vfs_t ptr) : OpaqueShared(ptr, pawkit_vfs_destroy) {}

        public:
        static Result<Filesystem> Working() {
            pawkit_vfs_error_t error;

            pawkit_vfs_t vfs = pawkit_vfs_working(&error);

            if (error)
                return error;

            return Filesystem(vfs);
        }

        static Result<Filesystem> Zip(Buffer buffer) {
            pawkit_vfs_error_t error;

            pawkit_vfs_t vfs = pawkit_vfs_zip(buffer.Release(), &error);

            if (error)
                return error;

            return Filesystem(vfs);
        }

        Result<Filesystem> Subdirectory(std::string const &path) {
            pawkit_vfs_error_t error;

            pawkit_vfs_t vfs = pawkit_vfs_subdirectory(Get(), path.c_str(), &error);

            if (error)
                return error;

            return Filesystem(vfs);
        }

        Result<Buffer> Open(std::string const &path) {
            pawkit_vfs_error_t error;

            pawkit_vfs_buffer_t buf = pawkit_vfs_open(Get(), path.c_str(), &error);
            
            if (error)
                return error;

            return Result<Buffer>(std::in_place_index<0>, buf);
        }
    };
}

#endif
