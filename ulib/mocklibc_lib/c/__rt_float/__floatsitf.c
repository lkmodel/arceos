#include <__rt_float.h>
#include <mocklibc.h>

long double __floatsitf(int i)
{
    typedef long double (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATSITF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
