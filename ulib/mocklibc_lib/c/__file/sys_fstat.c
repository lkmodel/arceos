#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <sys/stat.h>
#include <unistd.h>

int sys_fstat(int fd, stat *buf)
{
    typedef int (*FnABI)(int, stat *);
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_FSTAT);
    FnABI func = (FnABI)(*abi_ptr);
    func(fd, buf);
}
