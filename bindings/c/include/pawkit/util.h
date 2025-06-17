#pragma once

#if defined(_MSC_VER)
#define PAWKIT_CDECL __cdecl
#elif defined(__GNUC__) || defined(__clang__)
#define PAWKIT_CDECL __attribute__((__cdecl__))
#else
#define PAWKIT_CDECL
#endif

#include <stdint.h>
#include <stddef.h>

typedef uint8_t pawkit_u8;
typedef int8_t pawkit_i8;

typedef uint16_t pawkit_u16;
typedef int16_t pawkit_i16;

typedef uint32_t pawkit_u32;
typedef int32_t pawkit_i32;

typedef uint64_t pawkit_u64;
typedef int64_t pawkit_i64;

typedef size_t pawkit_usize;
typedef ptrdiff_t pawkit_isize;

typedef float pawkit_f32;
typedef double pawkit_f64;

#ifdef __cplusplus

#include <memory>
#include <concepts>
#include <functional>

namespace PawKit {
    constexpr inline void NullDeleter(void *ptr) {}

    template <std::same_as<void *> T>
    struct OpaqueHolder {
        using Ptr = T;

        private:
        Ptr ptr;

        public:
        inline OpaqueHolder(Ptr ptr) : ptr(ptr) {}
        virtual ~OpaqueHolder() = default;

        inline operator Ptr () {
            return ptr;
        }

        inline operator Ptr () const {
            return ptr;
        }
    };

    template <std::same_as<void *> T>
    struct OpaqueShared {
        using Ptr = T;

        std::shared_ptr<void> ptr;

        inline OpaqueShared(Ptr ptr, std::function<void (Ptr)> destruct) : ptr(ptr, destruct) {}
        virtual ~OpaqueShared() = default;

        inline operator Ptr () {
            return ptr.get();
        }

        inline operator Ptr () const {
            return ptr.get();
        }
    };
}

#endif
