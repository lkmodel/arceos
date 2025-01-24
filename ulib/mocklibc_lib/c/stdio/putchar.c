#include <mocklibc.h>
#include <stdio.h>

void putchar(char c)
{
    typedef void (*FnABI)(char c);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_PUTCHAR);
    FnABI func = (FnABI)(*abi_ptr);
    return func(c);
}
