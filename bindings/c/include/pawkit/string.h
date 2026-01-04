#pragma once

#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct pawkit_string *pawkit_string_t;

pawkit_string_t pawkit_string_from(char const *ptr, pawkit_usize len);

void pawkit_string_remref(pawkit_string_t string);

void pawkit_string_addref(pawkit_string_t string);

char const *pawkit_string_get(pawkit_string_t string, pawkit_usize *size);

#ifdef __cplusplus
}
#endif
