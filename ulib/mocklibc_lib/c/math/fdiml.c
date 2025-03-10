// NOTE: `Std C impl based on musl 1.2.5`
#include <float.h>
#include <math.h>

#if LDBL_MANT_DIG == 53 && LDBL_MAX_EXP == 1024
long double fdiml(long double x, long double y)
{
    return fdim(x, y);
}
#else
long double fdiml(long double x, long double y)
{
    if (isnan(x))
        return x;
    if (isnan(y))
        return y;
    return x > y ? x - y : 0;
}
#endif
