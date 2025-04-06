#include <__rt_integer.h>
#include <mocklibc.h>

int64_t __bswapdi2(int64_t a)
{
    typedef int64_t (*FnABI)(int64_t a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_BSWAPDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
