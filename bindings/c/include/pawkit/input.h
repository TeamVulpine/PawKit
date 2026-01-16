#pragma once

#include "string.h"
#include "util.h"
#include "assert.h"

#ifdef __cplusplus
extern "C" {
#endif

enum {
    PAWKIT_INPUT_MOUSEAXIS_DELTA_X,
    PAWKIT_INPUT_MOUSEAXIS_DELTA_Y,
    PAWKIT_INPUT_MOUSEAXIS_WHEEL_X,
    PAWKIT_INPUT_MOUSEAXIS_WHEEL_Y,
};
typedef pawkit_u8 pawkit_input_mouseaxis_t;

enum {
    PAWKIT_INPUT_JOYAXIS_LEFT_X,
    PAWKIT_INPUT_JOYAXIS_LEFT_Y,
    PAWKIT_INPUT_JOYAXIS_RIGHT_X,
    PAWKIT_INPUT_JOYAXIS_RIGHT_Y,
    PAWKIT_INPUT_JOYAXIS_LEFT_TRIGGER,
    PAWKIT_INPUT_JOYAXIS_RIGHT_TRIGGER,
};
typedef pawkit_u8 pawkit_input_joyaxis_t;

typedef pawkit_u8 pawkit_input_axis_t;

enum {
    PAWKIT_INPUT_KEYBUTTON_A, PAWKIT_INPUT_KEYBUTTON_B, PAWKIT_INPUT_KEYBUTTON_C, PAWKIT_INPUT_KEYBUTTON_D,
    PAWKIT_INPUT_KEYBUTTON_E, PAWKIT_INPUT_KEYBUTTON_F, PAWKIT_INPUT_KEYBUTTON_G, PAWKIT_INPUT_KEYBUTTON_H,
    PAWKIT_INPUT_KEYBUTTON_I, PAWKIT_INPUT_KEYBUTTON_J, PAWKIT_INPUT_KEYBUTTON_K, PAWKIT_INPUT_KEYBUTTON_L,
    PAWKIT_INPUT_KEYBUTTON_M, PAWKIT_INPUT_KEYBUTTON_N, PAWKIT_INPUT_KEYBUTTON_O, PAWKIT_INPUT_KEYBUTTON_P,
    PAWKIT_INPUT_KEYBUTTON_Q, PAWKIT_INPUT_KEYBUTTON_R, PAWKIT_INPUT_KEYBUTTON_S, PAWKIT_INPUT_KEYBUTTON_T,
    PAWKIT_INPUT_KEYBUTTON_U, PAWKIT_INPUT_KEYBUTTON_V, PAWKIT_INPUT_KEYBUTTON_W, PAWKIT_INPUT_KEYBUTTON_X,
    PAWKIT_INPUT_KEYBUTTON_Y, PAWKIT_INPUT_KEYBUTTON_Z,

    PAWKIT_INPUT_KEYBUTTON_NUMBER_0, PAWKIT_INPUT_KEYBUTTON_NUMBER_1, PAWKIT_INPUT_KEYBUTTON_NUMBER_2,
    PAWKIT_INPUT_KEYBUTTON_NUMBER_3, PAWKIT_INPUT_KEYBUTTON_NUMBER_4, PAWKIT_INPUT_KEYBUTTON_NUMBER_5,
    PAWKIT_INPUT_KEYBUTTON_NUMBER_6, PAWKIT_INPUT_KEYBUTTON_NUMBER_7, PAWKIT_INPUT_KEYBUTTON_NUMBER_8,
    PAWKIT_INPUT_KEYBUTTON_NUMBER_9,

    PAWKIT_INPUT_KEYBUTTON_UP, PAWKIT_INPUT_KEYBUTTON_DOWN, PAWKIT_INPUT_KEYBUTTON_LEFT, PAWKIT_INPUT_KEYBUTTON_RIGHT,

    PAWKIT_INPUT_KEYBUTTON_TILDE, PAWKIT_INPUT_KEYBUTTON_GRAVE, PAWKIT_INPUT_KEYBUTTON_MINUS, PAWKIT_INPUT_KEYBUTTON_PLUS,
    PAWKIT_INPUT_KEYBUTTON_LEFT_BRACKET, PAWKIT_INPUT_KEYBUTTON_RIGHT_BRACKET, PAWKIT_INPUT_KEYBUTTON_SEMICOLON,
    PAWKIT_INPUT_KEYBUTTON_QUOTE, PAWKIT_INPUT_KEYBUTTON_COMMA, PAWKIT_INPUT_KEYBUTTON_PERIOD,
    PAWKIT_INPUT_KEYBUTTON_SLASH, PAWKIT_INPUT_KEYBUTTON_BACKSLASH,

    PAWKIT_INPUT_KEYBUTTON_LEFT_SHIFT, PAWKIT_INPUT_KEYBUTTON_RIGHT_SHIFT,
    PAWKIT_INPUT_KEYBUTTON_LEFT_CONTROL, PAWKIT_INPUT_KEYBUTTON_RIGHT_CONTROL,
    PAWKIT_INPUT_KEYBUTTON_LEFT_ALT, PAWKIT_INPUT_KEYBUTTON_RIGHT_ALT,
    PAWKIT_INPUT_KEYBUTTON_LEFT_META, PAWKIT_INPUT_KEYBUTTON_RIGHT_META,

    PAWKIT_INPUT_KEYBUTTON_MENU, PAWKIT_INPUT_KEYBUTTON_ENTER, PAWKIT_INPUT_KEYBUTTON_ESCAPE, PAWKIT_INPUT_KEYBUTTON_SPACE,
    PAWKIT_INPUT_KEYBUTTON_TAB, PAWKIT_INPUT_KEYBUTTON_BACKSPACE, PAWKIT_INPUT_KEYBUTTON_INSERT,
    PAWKIT_INPUT_KEYBUTTON_DELETE, PAWKIT_INPUT_KEYBUTTON_PAGE_UP, PAWKIT_INPUT_KEYBUTTON_PAGE_DOWN,
    PAWKIT_INPUT_KEYBUTTON_HOME, PAWKIT_INPUT_KEYBUTTON_END, PAWKIT_INPUT_KEYBUTTON_CAPS_LOCK,
    PAWKIT_INPUT_KEYBUTTON_SCROLL_LOCK, PAWKIT_INPUT_KEYBUTTON_PRINT_SCREEN, PAWKIT_INPUT_KEYBUTTON_PAUSE,
    PAWKIT_INPUT_KEYBUTTON_NUM_LOCK, PAWKIT_INPUT_KEYBUTTON_CLEAR, PAWKIT_INPUT_KEYBUTTON_SLEEP,

    PAWKIT_INPUT_KEYBUTTON_NUMPAD_0, PAWKIT_INPUT_KEYBUTTON_NUMPAD_1, PAWKIT_INPUT_KEYBUTTON_NUMPAD_2,
    PAWKIT_INPUT_KEYBUTTON_NUMPAD_3, PAWKIT_INPUT_KEYBUTTON_NUMPAD_4, PAWKIT_INPUT_KEYBUTTON_NUMPAD_5,
    PAWKIT_INPUT_KEYBUTTON_NUMPAD_6, PAWKIT_INPUT_KEYBUTTON_NUMPAD_7, PAWKIT_INPUT_KEYBUTTON_NUMPAD_8,
    PAWKIT_INPUT_KEYBUTTON_NUMPAD_9, PAWKIT_INPUT_KEYBUTTON_NUMPAD_DIVIDE,
    PAWKIT_INPUT_KEYBUTTON_NUMPAD_MULTIPLY, PAWKIT_INPUT_KEYBUTTON_NUMPAD_MINUS,
    PAWKIT_INPUT_KEYBUTTON_NUMPAD_PLUS, PAWKIT_INPUT_KEYBUTTON_NUMPAD_DECIMAL,
    PAWKIT_INPUT_KEYBUTTON_NUMPAD_PERIOD, PAWKIT_INPUT_KEYBUTTON_NUMPAD_ENTER,

    PAWKIT_INPUT_KEYBUTTON_F1, PAWKIT_INPUT_KEYBUTTON_F2, PAWKIT_INPUT_KEYBUTTON_F3, PAWKIT_INPUT_KEYBUTTON_F4,
    PAWKIT_INPUT_KEYBUTTON_F5, PAWKIT_INPUT_KEYBUTTON_F6, PAWKIT_INPUT_KEYBUTTON_F7, PAWKIT_INPUT_KEYBUTTON_F8,
    PAWKIT_INPUT_KEYBUTTON_F9, PAWKIT_INPUT_KEYBUTTON_F10, PAWKIT_INPUT_KEYBUTTON_F11, PAWKIT_INPUT_KEYBUTTON_F12,
    PAWKIT_INPUT_KEYBUTTON_F13, PAWKIT_INPUT_KEYBUTTON_F14, PAWKIT_INPUT_KEYBUTTON_F15, PAWKIT_INPUT_KEYBUTTON_F16,
    PAWKIT_INPUT_KEYBUTTON_F17, PAWKIT_INPUT_KEYBUTTON_F18, PAWKIT_INPUT_KEYBUTTON_F19, PAWKIT_INPUT_KEYBUTTON_F20,
    PAWKIT_INPUT_KEYBUTTON_F21, PAWKIT_INPUT_KEYBUTTON_F22, PAWKIT_INPUT_KEYBUTTON_F23, PAWKIT_INPUT_KEYBUTTON_F24,
};
typedef pawkit_u8 pawkit_input_keybutton_t;

enum {
    PAWKIT_INPUT_MOUSEBUTTON_LEFT,
    PAWKIT_INPUT_MOUSEBUTTON_RIGHT,
    PAWKIT_INPUT_MOUSEBUTTON_MIDDLE,
    PAWKIT_INPUT_MOUSEBUTTON_SIDE1,
    PAWKIT_INPUT_MOUSEBUTTON_SIDE2,
};
typedef pawkit_u8 pawkit_input_mousebutton_t;

enum {
    PAWKIT_INPUT_JOYBUTTON_SOUTH,
    PAWKIT_INPUT_JOYBUTTON_EAST,
    PAWKIT_INPUT_JOYBUTTON_WEST,
    PAWKIT_INPUT_JOYBUTTON_NORTH,
    PAWKIT_INPUT_JOYBUTTON_BACK,
    PAWKIT_INPUT_JOYBUTTON_GUIDE,
    PAWKIT_INPUT_JOYBUTTON_START,
    PAWKIT_INPUT_JOYBUTTON_LEFT_STICK,
    PAWKIT_INPUT_JOYBUTTON_RIGHT_STICK,
    PAWKIT_INPUT_JOYBUTTON_LEFT_SHOULDER,
    PAWKIT_INPUT_JOYBUTTON_RIGHT_SHOULDER,
    PAWKIT_INPUT_JOYBUTTON_DPAD_UP,
    PAWKIT_INPUT_JOYBUTTON_DPAD_DOWN,
    PAWKIT_INPUT_JOYBUTTON_DPAD_LEFT,
    PAWKIT_INPUT_JOYBUTTON_DPAD_RIGHT,
    PAWKIT_INPUT_JOYBUTTON_MISC1,
    PAWKIT_INPUT_JOYBUTTON_RIGHT_PADDLE1,
    PAWKIT_INPUT_JOYBUTTON_LEFT_PADDLE1,
    PAWKIT_INPUT_JOYBUTTON_RIGHT_PADDLE2,
    PAWKIT_INPUT_JOYBUTTON_LEFT_PADDLE2,
    PAWKIT_INPUT_JOYBUTTON_TOUCHPAD,
    PAWKIT_INPUT_JOYBUTTON_MISC2,
    PAWKIT_INPUT_JOYBUTTON_MISC3,
    PAWKIT_INPUT_JOYBUTTON_MISC4,
    PAWKIT_INPUT_JOYBUTTON_MISC5,
    PAWKIT_INPUT_JOYBUTTON_MISC6,
};
typedef pawkit_u8 pawkit_input_joybutton_t;

typedef pawkit_u8 pawkit_input_button_t;

enum {
    PAWKIT_INPUT_FAMILY_KEY,
    PAWKIT_INPUT_FAMILY_MOUSE,
    PAWKIT_INPUT_FAMILY_JOY,
};
typedef pawkit_u8 pawkit_input_family_t;

enum {
    PAWKIT_INPUT_BOUND_BUTTON_TYPE_DIGITAL,
    PAWKIT_INPUT_BOUND_BUTTON_TYPE_ANALOG,
};
typedef pawkit_u8 pawkit_input_bound_button_type_t;

typedef struct pawkit_input_bound_button_t {
    pawkit_input_bound_button_type_t type;
    union {
        pawkit_input_button_t button;
        struct {
            pawkit_input_axis_t axis;
            pawkit_f32 threshold;
        };
    };
} pawkit_input_bound_button_t;

enum {
    PAWKIT_INPUT_BOUND_AXIS_TYPE_ANALOG,
    PAWKIT_INPUT_BOUND_AXIS_TYPE_DIGITAL,
    PAWKIT_INPUT_BOUND_AXIS_TYPE_MULTI_DIGITAL,
};
typedef pawkit_u8 pawkit_input_bound_axis_type_t;

typedef struct pawkit_input_bound_axis_t {
    pawkit_input_bound_axis_type_t type;
    union {
        pawkit_input_button_t button;
        pawkit_input_axis_t axis;
        struct {
            pawkit_input_button_t negative;
            pawkit_input_button_t positive;
        };
    };
} pawkit_input_bound_axis_t;

typedef struct pawkit_input_digital_binding_t {
    pawkit_input_family_t family;
    pawkit_input_bound_button_t binding;
} pawkit_input_digital_binding_t;

typedef struct pawkit_input_analog_binding_t {
    pawkit_input_family_t family;
    pawkit_input_bound_axis_t binding;
    pawkit_f32 deadzone;
    pawkit_f32 scale;
} pawkit_input_analog_binding_t;

typedef struct pawkit_input_vector_binding_t {
    pawkit_input_family_t family;
    pawkit_input_bound_axis_t x;
    pawkit_input_bound_axis_t y;
    pawkit_f32 deadzone;
    pawkit_f32 scale_x;
    pawkit_f32 scale_y;
} pawkit_input_vector_binding_t;

typedef struct pawkit_input_binding_map *pawkit_input_binding_map_t;
typedef struct pawkit_input_state *pawkit_input_state_t;
typedef struct pawkit_input_manager *pawkit_input_manager_t;

typedef union pawkit_input_frame_t {
    struct {
        bool value;
        bool just_pressed;
        bool just_released;
    } digital;
    
    struct {
        pawkit_f32 value;
        pawkit_f32 delta;
    } analog;
    
    struct {
        pawkit_f32 x;
        pawkit_f32 y;
        pawkit_f32 delta_x;
        pawkit_f32 delta_y;
    } vector;
} pawkit_input_frame_t;

enum {
    PAWKIT_INPUT_ERROR_OK,
    PAWKIT_INPUT_ERROR_INVALID_STRING,
    PAWKIT_INPUT_ERROR_INVALID_JSON,
};
typedef uint32_t pawkit_input_error_t;

typedef struct pawkit_device_id {
    pawkit_u8 state[16];
} pawkit_device_id_t;

pawkit_input_binding_map_t pawkit_input_binding_map_create();

void pawkit_input_binding_map_destroy(pawkit_input_binding_map_t map);

pawkit_input_binding_map_t pawkit_input_binding_map_load(
    char const *cstr,
    size_t len,
    pawkit_input_error_t *error
);

char const *pawkit_input_binding_map_save(
    pawkit_input_binding_map_t map,
    size_t *len
);

void pawkit_input_binding_map_register_digital_binding(
    pawkit_input_binding_map_t map,
    pawkit_string_t name,
    pawkit_input_digital_binding_t const *bindings,
    size_t len
);

void pawkit_input_binding_map_register_analog_binding(
    pawkit_input_binding_map_t map,
    pawkit_string_t name,
    pawkit_input_analog_binding_t const *bindings,
    size_t len
);

void pawkit_input_binding_map_register_vector_binding(
    pawkit_input_binding_map_t map,
    pawkit_string_t name,
    pawkit_input_vector_binding_t const *bindings,
    size_t len
);

void pawkit_input_binding_map_ensure_prototype(
    pawkit_input_binding_map_t map,
    pawkit_input_binding_map_t prototype
);

pawkit_input_state_t pawkit_input_state_create();

void pawkit_input_state_destroy(pawkit_input_state_t state);

pawkit_device_id_t pawkit_input_state_connect_keyboard(pawkit_input_state_t state);

pawkit_device_id_t pawkit_input_state_connect_mouse(pawkit_input_state_t state);

pawkit_device_id_t pawkit_input_state_connect_gamepad(pawkit_input_state_t state);

void pawkit_input_state_disconnect_device(
    pawkit_input_state_t state,
    pawkit_device_id_t device
);

void pawkit_input_state_set_button(
    pawkit_input_state_t state,
    pawkit_device_id_t device,
    pawkit_u8 button,
    bool value
);

void pawkit_input_state_set_axis(
    pawkit_input_state_t state,
    pawkit_device_id_t device,
    pawkit_u8 axis,
    pawkit_f32 value
);

pawkit_input_manager_t pawkit_input_manager_create(
    pawkit_input_binding_map_t map
);

void pawkit_input_manager_destroy(pawkit_input_manager_t manager);

void pawkit_input_manager_connect_device(
    pawkit_input_manager_t manager,
    pawkit_device_id_t device
);

void pawkit_input_manager_disconnect_device(
    pawkit_input_manager_t manager,
    pawkit_device_id_t device
);

void pawkit_input_manager_update(
    pawkit_input_manager_t manager,
    pawkit_input_state_t state
);

bool pawkit_input_manager_get(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_frame_t *frame
);

bool pawkit_input_manager_add_digital_binding(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_digital_binding_t binding
);

bool pawkit_input_manager_add_analog_binding(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_analog_binding_t binding
);

bool pawkit_input_manager_add_vector_binding(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_vector_binding_t binding
);

bool pawkit_input_manager_remove_digital_binding(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_digital_binding_t binding
);

bool pawkit_input_manager_remove_analog_binding(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_analog_binding_t binding
);

bool pawkit_input_manager_remove_vector_binding(
    pawkit_input_manager_t manager,
    pawkit_string_t name,
    pawkit_input_vector_binding_t binding
);

#ifdef __cplusplus
}
#endif
