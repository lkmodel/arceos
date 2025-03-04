#include <__rt_tmp.h>
#include <mocklibc.h>

void __cxa_finalize(void *d)
{
    NOIMPL
    //    typedef int (*FnABI)(unsigned long long a);
    //    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZTI2);
    //    FnABI func = (FnABI)(*abi_ptr);
    //    return func(a);
}
