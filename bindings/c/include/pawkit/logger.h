#pragma once

#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef PAWKIT_CDECL void (*pawkit_logger_callback_fn)(const char* message);

typedef struct pawkit_logger_callbacks_t {
    pawkit_logger_callback_fn print_to_console;
    pawkit_logger_callback_fn print_to_logfile;
} pawkit_logger_callbacks;

void pawkit_logger_set_logger_callbacks(pawkit_logger_callbacks callback);
void pawkit_logger_reset_logger_callbacks();

void pawkit_logger_info(char const *message);
void pawkit_logger_debug(char const *message);
void pawkit_logger_warn(char const *message);
void pawkit_logger_error(char const *message);
void pawkit_logger_fatal(char const *message);

#ifdef __cplusplus
}

#include <string>

#if __cplusplus >= 202002L
#include <format>

#define LOGFUNC(cppname, cname) \
    template <typename ...TArgs> \
    void cppname(std::format_string<TArgs &...> fmt, TArgs &&...args) { \
        std::string formatted = std::format(fmt, args...); \
        pawkit_logger_##cname(formatted.c_str()); \
    }

#else

#define LOGFUNC(cppname, cname) \
    inline void cppname(std::string &message) { \
        pawkit_logger_##cname(message.c_str()); \
    }

#endif

namespace PawKit::Logger {
    using LoggerCallbacks = pawkit_logger_callbacks;

    inline void SetLoggerCallbacks(LoggerCallbacks callback) {
        pawkit_logger_set_logger_callbacks(callback);
    }

    inline void ResetLoggerCallbacks() {
        pawkit_logger_reset_logger_callbacks();
    }

    LOGFUNC(Info, info)
    LOGFUNC(Debug, debug)
    LOGFUNC(Warn, warn)
    LOGFUNC(Error, error)
    LOGFUNC(Fatal, fatal)
}

#undef LOGFUNC

#endif
