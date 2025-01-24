#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

ssize_t sys_lstat(const char *path, stat *buf)
{
    typedef size_t (*FnABI)(const char *, stat *);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_LSTAT);
    FnABI func = (FnABI)(*abi_ptr);
    return func(path, buf);
}
