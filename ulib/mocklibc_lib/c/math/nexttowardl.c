// NOTE: `Std C impl based on musl 1.2.5`
#include <math.h>

long double nexttowardl(long double x, long double y)
{
    return nextafterl(x, y);
}
