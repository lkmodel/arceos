#include <mocklibc.h>
#include <stdlib.h>

void *calloc(size_t m, size_t n)
{
    typedef void *(*FnABI)(size_t, size_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_CALLOC);
    FnABI func = (FnABI)(*abi_ptr);
    return func(m, n);
}
