#include <__rt_integer.h>
#include <mocklibc.h>

int32_t __bswapsi2(int32_t a)
{
    typedef int32_t (*FnABI)(int32_t a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_BSWAPSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
