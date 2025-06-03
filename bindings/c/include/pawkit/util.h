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
