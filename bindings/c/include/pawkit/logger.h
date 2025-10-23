#pragma once

#include "util.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef PAWKIT_CDECL void (*pawkit_logger_callback_fn_t)(const char* message, pawkit_usize message_size);

typedef struct pawkit_logger_callbacks_t {
    pawkit_logger_callback_fn_t print_to_console;
    pawkit_logger_callback_fn_t print_to_logfile;
} pawkit_logger_callbacks;

void pawkit_logger_set_logger_callbacks(pawkit_logger_callbacks callback);
void pawkit_logger_reset_logger_callbacks();

void pawkit_logger_info(char const *message, pawkit_usize message_size);
void pawkit_logger_debug(char const *message, pawkit_usize message_size);
void pawkit_logger_warn(char const *message, pawkit_usize message_size);
void pawkit_logger_error(char const *message, pawkit_usize message_size);
void pawkit_logger_fatal(char const *message, pawkit_usize message_size);

#ifdef __cplusplus
}
#endif
