#include <__rt_float.h>
#include <mocklibc.h>

long __fixtfdi(long double a)
{
    typedef long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
