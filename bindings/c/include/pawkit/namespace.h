#pragma once

#define COMBINE_INNER(a, b) a##b
#define COMBINE(a, b) COMBINE_INNER(a, b)
#define N(value) COMBINE(NAMESPACE, _##value)
