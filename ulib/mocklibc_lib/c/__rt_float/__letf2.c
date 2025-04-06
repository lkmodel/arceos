#include <__rt_float.h>
#include <mocklibc.h>

int __letf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LETF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
