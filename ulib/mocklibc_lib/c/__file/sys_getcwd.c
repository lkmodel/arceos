#include <fcntl.h>
#include <mocklibc.h>
#include <stddef.h>
#include <stdio.h>
#include <unistd.h>

char sys_getcwd(char *buf, size_t size)
{
    typedef int (*FnABI)(char *, size_t);
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_GETCWD);
    FnABI func = (FnABI)(*abi_ptr);
    func(buf, size);
}
