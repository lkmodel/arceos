#include <__rt_integer.h>
#include <mocklibc.h>

int __clzti2(unsigned long long a)
{
    typedef int (*FnABI)(unsigned long long a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
