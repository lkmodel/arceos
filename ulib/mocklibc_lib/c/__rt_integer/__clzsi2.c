#include <__rt_integer.h>
#include <mocklibc.h>

int __clzsi2(unsigned int a)
{
    typedef int (*FnABI)(unsigned int a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
