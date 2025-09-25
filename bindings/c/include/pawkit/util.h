#pragma once

#ifdef __cplusplus
extern "C" {
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
}

#include <memory>
#include <concepts>
#include <functional>
#include <string>
#include <vector>
#include <span>
#include <string_view>
#include <optional>
#include <variant>
#include <utility>

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

        private:
        std::shared_ptr<void> ptr;

        public:
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

    template <std::same_as<void *> T>
    struct OpaqueUnique {
        using Ptr = T;

        private:
        std::unique_ptr<void, void(*)(T)> ptr;

        public:
        inline OpaqueUnique(Ptr ptr, std::function<void (Ptr)> destruct) : ptr(ptr, destruct) {}
        virtual ~OpaqueUnique() = default;

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

        inline Ptr Release() {
            return ptr.release();
        }

        inline void Reset(Ptr p) {
            ptr.reset(p);
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

    template <bool TOwned>
    using BufReturnType = MaybeOwned<TOwned, std::vector<pawkit_u8>, std::span<pawkit_u8 const>>::Type;

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

    template <typename TError, bool TOwned = true, Callable<char const *, TError &> TFunc>
    std::variant<StrReturnType<TOwned>, TError> GetStringErr(TFunc func) {
        TError err = TError();

        char const* rawStr = func(err);

        if (err)
            return err;

        if (!rawStr)
            return "";

        StrReturnType<TOwned> str = rawStr;

        if constexpr (TOwned)
            pawkit_free_string(rawStr);

        return str;
    }

    template <typename TError, typename TResult, bool TOwned = true, Callable<char const *, TError &> TFunc>
    TResult GetStringErr(TFunc func) {
        TError err = TError();

        char const* rawStr = func(err);

        if (err)
            return TResult(err);

        if (!rawStr)
            return TResult("");

        StrReturnType<TOwned> str = rawStr;

        if constexpr (TOwned)
            pawkit_free_string(rawStr);

        return TResult(str);
    }

    template <bool TOwned = true, Callable<char const *> TFunc>
    std::optional<StrReturnType<TOwned>> GetStringOptional(TFunc func) {
        char const* rawStr = func();

        if (!rawStr)
            return std::nullopt;

        StrReturnType<TOwned> str = rawStr;

        if constexpr (TOwned)
            pawkit_free_string(rawStr);

        return str;
    }

    template <typename TError, bool TOwned = true, Callable<char const *, TError &> TFunc>
    std::variant<std::optional<StrReturnType<TOwned>>, TError> GetStringErrOptional(TFunc func) {
        using Output = std::variant<std::optional<StrReturnType<TOwned>>, TError>;

        TError err = TError();

        char const* rawStr = func(err);

        if (err)
            return Output(std::in_place_index<1>, err);

        if (!rawStr)
            return Output(std::in_place_index<0>, std::nullopt);

        StrReturnType<TOwned> str = rawStr;

        if constexpr (TOwned)
            pawkit_free_string(rawStr);

        return Output(std::in_place_index<0>, str);
    }

    template <typename TError, typename TResult, bool TOwned = true, Callable<char const *, TError &> TFunc>
    TResult GetStringErrOptional(TFunc func) {
        TError err = TError();

        char const* rawStr = func(err);

        if (err)
            return TResult(err);

        if (!rawStr)
            return TResult(std::nullopt);

        StrReturnType<TOwned> str = rawStr;

        if constexpr (TOwned)
            pawkit_free_string(rawStr);

        return TResult(std::optional(str));
    }

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

    template <typename TError, bool TOwned = true, Callable<pawkit_u8 const *, pawkit_usize &, TError &> TFunc>
    std::variant<BufReturnType<TOwned>, TError> GetBufErr(TFunc func) {
        pawkit_usize size = 0;
        TError error = TError();

        pawkit_u8 const *data = func(size, error);

        if (error)
            return error;

        if (!data || size == 0)
            return BufReturnType<true>{};

        BufReturnType<TOwned> buf {data, data + size};

        if constexpr (TOwned) 
            pawkit_free_array(data);
        
        return buf;
    }

    template <typename TError, typename TResult, bool TOwned = true, Callable<pawkit_u8 const *, pawkit_usize &, TError &> TFunc>
    TResult GetBufErr(TFunc func) {
        pawkit_usize size = 0;
        TError error = TError();

        pawkit_u8 const *data = func(size, error);

        if (error)
            return TResult(error);

        if (!data || size == 0)
            return BufReturnType<true>{};

        BufReturnType<TOwned> buf {data, data + size};

        if constexpr (TOwned) 
            pawkit_free_array(data);
        
        return TResult(buf);
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
