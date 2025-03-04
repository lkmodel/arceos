#include <__rt_float.h>
#include <mocklibc.h>

double __trunctfdf2(long double a)
{
    typedef double (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCTFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
