#include <__rt_float.h>
#include <mocklibc.h>

long double __extenddftf2(double a)
{
    typedef long double (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDDFTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
