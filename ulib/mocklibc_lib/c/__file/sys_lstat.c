#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

ssize_t sys_lstat(const char *path, stat *buf)
{
    typedef int (*FnABI)(const char *, stat *);
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_LSTAT);
    FnABI func = (FnABI)(*abi_ptr);
    func(path, buf);
}
