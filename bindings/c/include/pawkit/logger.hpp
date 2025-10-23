#pragma once

#include "logger.h"

#include <format>

#define LOGFUNC(cppname, cname) \
    template <typename ...TArgs> \
    void cppname(std::format_string<TArgs &...> fmt, TArgs &&...args) { \
        std::string formatted = std::format(fmt, args...); \
        pawkit_logger_##cname(formatted.data(), formatted.size()); \
    }

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
