// NOTE: `Std C impl based on musl 1.2.5`
#include <math.h>

float fdimf(float x, float y)
{
    if (isnan(x))
        return x;
    if (isnan(y))
        return y;
    return x > y ? x - y : 0;
}
