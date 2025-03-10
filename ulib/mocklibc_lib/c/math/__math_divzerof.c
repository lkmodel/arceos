// NOTE: `Std C impl based on musl 1.2.5`
#include "libm.h"

float __math_divzerof(uint32_t sign)
{
    return fp_barrierf(sign ? -1.0f : 1.0f) / 0.0f;
}
