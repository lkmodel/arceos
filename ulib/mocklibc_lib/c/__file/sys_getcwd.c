#include <fcntl.h>
#include <mocklibc.h>
#include <stddef.h>
#include <stdio.h>
#include <unistd.h>

char sys_getcwd(char *buf, size_t size)
{
    typedef char (*FnABI)(char *, size_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_GETCWD);
    FnABI func = (FnABI)(*abi_ptr);
    return func(buf, size);
}
