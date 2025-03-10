// NOTE: `Std C impl based on musl 1.2.5`
#include "libm.h"

// FIXME: macro in math.h
int __signbitf(float x)
{
    union {
        float f;
        uint32_t i;
    } y = {x};
    return y.i >> 31;
}
