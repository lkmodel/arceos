#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <unistd.h>

int sys_rename(const char *old, const char *new)
{
    typedef int (*FnABI)(const char *, const char *);
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_RENAME);
    FnABI func = (FnABI)(*abi_ptr);
    func(old, new);
}
