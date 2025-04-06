#include <__rt_float.h>
#include <mocklibc.h>

long double __floatunsitf(unsigned int i)
{
    typedef long double (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATUNSITF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
