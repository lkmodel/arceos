// NOTE: `Std C impl based on musl 1.2.5`
#include <ctype.h>

int isalnum(int c)
{
    return isalpha(c) || isdigit(c);
}

int __isalnum_l(int c, locale_t l)
{
    return isalnum(c);
}

weak_alias(__isalnum_l, isalnum_l);
