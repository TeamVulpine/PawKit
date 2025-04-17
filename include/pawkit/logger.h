#pragma once

#ifdef __cplusplus
extern "C" {
#endif

    void pawkit_logger_info(char const *message);
    void pawkit_logger_debug(char const *message);
    void pawkit_logger_warn(char const *message);
    void pawkit_logger_error(char const *message);
    void pawkit_logger_fatal(char const *message);

#ifdef __cplusplus
}
#endif

#ifdef __cplusplus

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
    inline void cppname(std::string &fmt) { \
        pawkit_logger_##cname(fmt.c_str()); \
    }

#endif

namespace PawKit::Logger {
    LOGFUNC(Info, info)
    LOGFUNC(Debug, debug)
    LOGFUNC(Warn, warn)
    LOGFUNC(Error, error)
    LOGFUNC(Fatal, fatal)
}

#undef LOGFUNC

#endif
