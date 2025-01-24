#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <unistd.h>

off_t sys_lseek(int fd, off_t offset, int whence)
{
    typedef off_t (*FnABI)(int, off_t, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_LSEEK);
    FnABI func = (FnABI)(*abi_ptr);
    return func(fd, offset, whence);
}
