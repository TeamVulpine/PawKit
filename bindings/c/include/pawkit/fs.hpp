#pragma once

#include "fs.h"
#include "util.h"

#include <optional>
#include <span>
#include <string>
#include <vector>
#include <expected>

namespace PawKit::Vfs {
    enum struct Error : pawkit_vfs_error_t {
        Ok,
        InvalidPtr,
        Io,
        Zip,
        NotFound,
        Other
    };

    template <typename T>
    using Result = std::expected<T, Error>;
    
    /// Represents a virtual file buffer.
    /// It can be represented by an actual file buffer, a zip file buffer, or a byte array.
    struct Buffer final {
        ~Buffer() {
            pawkit_vfs_buffer_free(*this);
        };

        Buffer() = delete;
        Buffer(Buffer const &copy) = delete;
        Buffer(Buffer &&move) = delete;

        operator pawkit_vfs_buffer_t () {
            return reinterpret_cast<pawkit_vfs_buffer_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static Buffer *From(pawkit_vfs_buffer_t buf) {
            return reinterpret_cast<Buffer *>(buf);
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
                return std::unexpected(Error(err));

            return ok;
        }

        /// Reads the buffer to an array
        Result<std::vector<pawkit_u8>> ReadToArray() {
            pawkit_vfs_error_t err = 0;
            pawkit_usize size = 0;
            pawkit_u8 const *data = pawkit_vfs_buffer_read_to_array(*this, &size, &err);

            if (err) 
                return std::unexpected(Error(err));

            std::vector<pawkit_u8> arr {data, data + size};

            pawkit_free_array(data, size);

            return arr;
        }

        /// Reads the buffer to a string
        Result<std::string> ReadToString() {
            pawkit_vfs_error_t err = 0;
            pawkit_usize size;
            char const *data = pawkit_vfs_buffer_read_to_string(*this, &size, &err);

            if (err) 
                return std::unexpected(Error(err));

            std::string arr {data, data + size};

            pawkit_free_string(data, size);

            return arr;
        }
    };

    /// Represents a "list" operation for the VFS
    struct List final {
        ~List() {
            pawkit_vfs_list_free(*this);
        };

        List() = delete;
        List(List const &copy) = delete;
        List(List &&move) = delete;

        operator pawkit_vfs_list_t () {
            return reinterpret_cast<pawkit_vfs_list_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static List *From(pawkit_vfs_list_t list) {
            return reinterpret_cast<List *>(list);
        }

        /// Gets the next file in the list
        Result<std::optional<std::string>> Next() {
            pawkit_vfs_error_t err = 0;
            pawkit_usize size;
            char const *fisizeame = pawkit_vfs_list_next(*this, &size, &err);

            if (err)
                return std::unexpected(Error(err));

            std::string arr {fisizeame, fisizeame + size};

            pawkit_free_string(fisizeame, size);

            return arr;
        }

        /// Only look for files with a given extension
        Result<List *> WithExtension(std::string const &ext) {
            pawkit_vfs_error_t err = 0;
            pawkit_vfs_list_with_extension(*this, ext.c_str(), &err);

            if (err)
                return std::unexpected(Error(err));

            return this;
        }

        struct Iterator final {
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
            pawkit_vfs_free(*this);
        };

        Filesystem() = delete;
        Filesystem(Filesystem const &copy) = delete;
        Filesystem(Filesystem &&move) = delete;

        operator pawkit_vfs_t () {
            return reinterpret_cast<pawkit_vfs_t>(this);
        }

        void operator delete (void *ptr) {
            // Empty to avoid double free.
        }

        static Filesystem *From(pawkit_vfs_t fs) {
            return reinterpret_cast<Filesystem *>(fs);
        }
        
        /// Gets the virtual filesystem associated with the current working directory
        static Result<Filesystem *> Working() {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_t vfs = pawkit_vfs_working(&err);

            if (err)
                return std::unexpected(Error(err));

            return From(vfs);
        }
        
        /// Takes ownership of the buffer, the pointer will no longer be valid.
        static Result<Filesystem *> Zip(Buffer *buf) {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_t vfs = pawkit_vfs_zip(*buf, &err);

            if (err)
                return std::unexpected(Error(err));

            return From(vfs);
        }

        /// Gets a subdirectory of the filesystem as a new filesystem
        Result<Filesystem *> Subdirectory(std::string_view path) {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_t vfs = pawkit_vfs_subdirectory(*this, path.data(), path.size(), &err);

            if (err)
                return std::unexpected(Error(err));

            return From(vfs);
        }

        /// Lists all the top-level subdirectories of the filesystem
        Result<List *> ListSubdirectories() {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_subdirectories(*this,  &err);

            if (err)
                return std::unexpected(Error(err));

            return List::From(list);
        }

        /// Lists all the top-level files of the filesystem
        Result<List *> ListFiles() {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_files(*this,  &err);

            if (err)
                return std::unexpected(Error(err));

            return List::From(list);
        }

        /// Lists all the files of the filesystem recursively
        Result<List *> ListFilesRecursive() {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_list_t list = pawkit_vfs_list_files_recursive(*this,  &err);

            if (err)
                return std::unexpected(Error(err));

            return List::From(list);
        }

        /// Opens the file at the given path
        Result<Buffer *> Open(std::string_view path) {
            pawkit_vfs_error_t err = 0;

            pawkit_vfs_buffer_t buf = pawkit_vfs_open(*this, path.data(), path.size(), &err);

            if (err)
                return std::unexpected(Error(err));

            return Buffer::From(buf);
        }
    };
}
