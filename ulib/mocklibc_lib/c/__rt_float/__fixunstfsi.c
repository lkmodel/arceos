#include <__rt_float.h>
#include <mocklibc.h>

unsigned int __fixunstfsi(long double a)
{
    typedef unsigned int (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSTFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
