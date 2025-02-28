#include <__rt_float.h>
#include <mocklibc.h>

float __trunctfsf2(long double a)
{
    typedef float (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCTFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
