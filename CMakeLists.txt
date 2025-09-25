cmake_minimum_required(VERSION 3.15)

set(PAWKIT_RUST_OUTDIR "${CMAKE_BINARY_DIR}/pawkit")

set(_PAWKIT_STATIC_LIBNAME "libpawkit.a")
set(_PAWKIT_DYNAMIC_LIBNAME "libpawkit.so")

if(WIN32)
    set(_PAWKIT_STATIC_LIBNAME "pawkit.lib")
    set(_PAWKIT_DYNAMIC_LIBNAME "pawkit.dll")
endif()

set(PAWKIT_INCLUDE "${CMAKE_CURRENT_SOURCE_DIR}/bindings/c/include")

set(PAWKIT_STATIC_LIB "${PAWKIT_RUST_OUTDIR}/release/${_PAWKIT_STATIC_LIBNAME}")
set(PAWKIT_DYNAMIC_LIB "${PAWKIT_RUST_OUTDIR}/release/${_PAWKIT_DYNAMIC_LIBNAME}")

file(GLOB_RECURSE PAWKIT_RUST_SOURCES
    "${CMAKE_CURRENT_SOURCE_DIR}/bindings/c/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/bindings/c/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/bitarray/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/bitarray/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/crockford/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/crockford/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/fs/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/fs/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/futures/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/futures/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/holy-array/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/holy-array/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/input/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/input/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/logger/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/logger/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/http/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/http/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/runtime/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/runtime/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/signaling/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/signaling/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/websocket/src/*.rs"
    "${CMAKE_CURRENT_SOURCE_DIR}/net/websocket/src/Cargo.toml"
    "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
)

add_custom_command(
    USES_TERMINAL
    OUTPUT ${PAWKIT_STATIC_LIB} ${PAWKIT_DYNAMIC_LIB}
    DEPENDS ${PAWKIT_RUST_SOURCES}
    COMMAND ${CMAKE_COMMAND} -E env CC= CXX= cargo build
        --release
        --target-dir ${PAWKIT_RUST_OUTDIR}
        --manifest-path "${CMAKE_CURRENT_SOURCE_DIR}/Cargo.toml"
        --package "pawkit-c"
    WORKING_DIRECTORY ${CMAKE_CURRENT_SOURCE_DIR}
    COMMENT "Building PawKit"
    VERBATIM
)

add_custom_target(pawkit_build ALL
    DEPENDS ${PAWKIT_STATIC_LIB} ${PAWKIT_DYNAMIC_LIB}
)

add_library(pawkit::static UNKNOWN IMPORTED GLOBAL)
add_library(pawkit::dynamic UNKNOWN IMPORTED GLOBAL)

set_target_properties(pawkit::static PROPERTIES
    IMPORTED_LOCATION "${PAWKIT_STATIC_LIB}"
    INTERFACE_INCLUDE_DIRECTORIES ${PAWKIT_INCLUDE}
)

set_target_properties(pawkit::dynamic PROPERTIES
    IMPORTED_LOCATION "${PAWKIT_DYNAMIC_LIB}"
    INTERFACE_INCLUDE_DIRECTORIES ${PAWKIT_INCLUDE}
)

if (CMAKE_SYSTEM_NAME STREQUAL "Windows")
    set_target_properties(pawkit::dynamic PROPERTIES
        IMPORTED_IMPLIB "${PAWKIT_DYNAMIC_LIB}.lib"
    )
endif()
