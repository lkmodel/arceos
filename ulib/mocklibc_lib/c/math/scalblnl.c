// NOTE: `Std C impl based on musl 1.2.5`
#include <float.h>
#include <limits.h>
#include <math.h>

#if LDBL_MANT_DIG == 53 && LDBL_MAX_EXP == 1024
long double scalblnl(long double x, long n)
{
    return scalbln(x, n);
}
#else
long double scalblnl(long double x, long n)
{
    if (n > INT_MAX)
        n = INT_MAX;
    else if (n < INT_MIN)
        n = INT_MIN;
    return scalbnl(x, n);
}
#endif
