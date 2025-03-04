#include <mocklibc.h>
#include <stdlib.h>

void *malloc(size_t n)
{
    typedef void *(*FnABI)(size_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_MALLOC);
    FnABI func = (FnABI)(*abi_ptr);
    return func(n);
}
