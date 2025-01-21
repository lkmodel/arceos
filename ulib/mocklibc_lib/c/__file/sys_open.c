#include <fcntl.h>
#include <mocklibc.h>
#include <stdio.h>
#include <unistd.h>

int sys_open(const char *fimename, int flags, mode_t ctypes)
{
    typedef int (*FnABI)(const char *, int, mode_t);
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_OPEN);
    FnABI func = (FnABI)(*abi_ptr);
    func(fimename, flags, ctypes);
}
