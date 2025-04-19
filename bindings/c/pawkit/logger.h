#pragma once

#include "namespace.h"

#define NAMESPACE pawkit_logger

#ifdef __cplusplus
extern "C" {
#endif

    void N(info)(char const *message);
    void N(debug)(char const *message);
    void N(warn)(char const *message);
    void N(error)(char const *message);
    void N(fatal)(char const *message);

#ifdef __cplusplus
}

#include <string>

#if __cplusplus >= 202002L
#include <format>

#define LOGFUNC(cppname, cname) \
    template <typename ...TArgs> \
    void cppname(std::format_string<TArgs &...> fmt, TArgs &&...args) { \
        std::string formatted = std::format(fmt, args...); \
        N(cname)(formatted.c_str()); \
    }

#else

#define LOGFUNC(cppname, cname) \
    inline void cppname(std::string &message) { \
        N(cname)(message.c_str()); \
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

#undef NAMESPACE
