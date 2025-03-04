#include <__rt_float.h>
#include <mocklibc.h>

long double __extendsftf2(float a)
{
    typedef long double (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
