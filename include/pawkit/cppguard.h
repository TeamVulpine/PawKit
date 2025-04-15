#pragma once

#ifdef __cplusplus
#define PAWKIT_CSEL(cpp, c) cpp
#else
#define PAWKIT_CSEL(cpp, c) c
#endif

#define PAWKIT_CPPGUARD_S PAWKIT_CSEL(extern "C" {, )
#define PAWKIT_CPPGUARD_E PAWKIT_CSEL(}, )
