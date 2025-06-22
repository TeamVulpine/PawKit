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

void pawkit_free_string(char const *str);
void pawkit_free_array(pawkit_u8 const *buf);

#ifdef __cplusplus

#include <memory>
#include <concepts>
#include <functional>
#include <string>
#include <vector>
#include <span>
#include <string_view>
#include <optional>

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

        inline Ptr Get() {
            return ptr;
        }

        inline Ptr Get() const {
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

        inline Ptr Get() {
            return ptr.get();
        }

        inline Ptr Get() const {
            return ptr.get();
        }
    };

    template <typename T, typename TReturn, typename ...TArgs>
    concept Callable = requires (T func, TArgs ...args) {
        {func(args...)} -> std::same_as<TReturn>;
    };

    template <bool TOwned, typename TOwning, typename TNonOwning>
    struct MaybeOwned;

    template <typename TOwning, typename TNonOwning>
    struct MaybeOwned<true, TOwning, TNonOwning> {
        using Type = TOwning;
    };

    template <typename TOwning, typename TNonOwning>
    struct MaybeOwned<false, TOwning, TNonOwning> {
        using Type = TNonOwning;
    };

    template <bool TOwned>
    using StrReturnType = MaybeOwned<TOwned, std::string, std::string_view>::Type;

    template <bool TOwned = true, Callable<char const *> TFunc>
    StrReturnType<TOwned> GetString(TFunc func) {
        char const* rawStr = func();

        if (!rawStr)
            return "";

        StrReturnType<TOwned> str = rawStr;

        if constexpr (TOwned)
            pawkit_free_string(rawStr);

        return str;
    }

    template <bool TOwned>
    using BufReturnType = MaybeOwned<TOwned, std::vector<pawkit_u8>, std::span<pawkit_u8 const>>::Type;

    template <bool TOwned = true, Callable<pawkit_u8 const *, pawkit_usize &> TFunc>
    BufReturnType<TOwned> GetBuf(TFunc func) {
        pawkit_usize size = 0;
        pawkit_u8 const *data = func(size);

        if (!data || size == 0)
            return {};

        BufReturnType<TOwned> buf {data, data + size};

        if constexpr (TOwned) 
            pawkit_free_array(data);
        
        return buf;
    }

    template <typename T, Callable<bool, T &> TFunc>
    std::optional<T> GetOptional(TFunc func) {
        T value;

        if (!func(value))
            return std::nullopt;

        return value;
    }
}

#endif
