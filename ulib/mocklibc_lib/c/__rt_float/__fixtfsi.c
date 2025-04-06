#include <__rt_float.h>
#include <mocklibc.h>

int __fixtfsi(long double a)
{
    typedef int (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
