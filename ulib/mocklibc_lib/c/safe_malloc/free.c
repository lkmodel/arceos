#include <mocklibc.h>
#include <stdlib.h>

void free(void *p)
{
    typedef void (*FnABI)(void *);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_FREE);
    FnABI func = (FnABI)(*abi_ptr);
    return func(p);
}
