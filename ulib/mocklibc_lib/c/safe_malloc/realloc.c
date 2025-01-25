#include <mocklibc.h>
#include <stdlib.h>

void *realloc(void *p, size_t n)
{
    typedef void *(*FnABI)(void *, size_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_REALLOC);
    FnABI func = (FnABI)(*abi_ptr);
    return func(p, n);
}
