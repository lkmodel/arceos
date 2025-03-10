// NOTE: `Std C impl based on musl 1.2.5`
#include <ctype.h>
#include <stdlib.h>

long atol(const char *s)
{
    long n = 0;
    int neg = 0;
    while (isspace(*s)) s++;
    switch (*s) {
    case '-':
        neg = 1;
    case '+':
        s++;
    }
    /* Compute n as a negative number to avoid overflow on LONG_MIN */
    while (isdigit(*s)) n = 10 * n - (*s++ - '0');
    return neg ? n : -n;
}
