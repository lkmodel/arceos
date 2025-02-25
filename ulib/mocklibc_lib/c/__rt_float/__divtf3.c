#include <__rt_float.h>
#include <mocklibc.h>

long double __divtf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVTF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
