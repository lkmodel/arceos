// NOTE: `Std C impl based on musl 1.2.5`
#include <stdarg.h>
#include <stdio.h>

int snprintf(char *restrict s, size_t n, const char *restrict fmt, ...)
{
    int ret;
    va_list ap;
    va_start(ap, fmt);
    ret = vsnprintf(s, n, fmt, ap);
    va_end(ap);
    return ret;
}
