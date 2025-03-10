// NOTE: `Std C impl based on musl 1.2.5`
#include "libm.h"

/* k is such that k*ln2 has minimal relative error and x - kln2 > log(DBL_MIN) */
static const int k = 2043;
static const double kln2 = 0x1.62066151add8bp+10;

/* exp(x)/2 for x >= log(DBL_MAX), slightly better than 0.5*exp(x/2)*exp(x/2) */
double __expo2(double x, double sign)
{
    double scale;

    /* note that k is odd and scale*scale overflows */
    INSERT_WORDS(scale, (uint32_t)(0x3ff + k / 2) << 20, 0);
    /* exp(x - k ln2) * 2**(k-1) */
    /* in directed rounding correct sign before rounding or overflow is important */
    return exp(x - kln2) * (sign * scale) * scale;
}
