#include <__rt_integer.h>
#include <mocklibc.h>

int __clzdi2(unsigned long a)
{
    typedef int (*FnABI)(unsigned long a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
