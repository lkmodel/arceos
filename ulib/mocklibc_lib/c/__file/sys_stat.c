#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

int sys_stat(const char *path, stat *buf)
{
    typedef int (*FnABI)(const char *, stat *);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_STAT);
    FnABI func = (FnABI)(*abi_ptr);
    return func(path, buf);
}
