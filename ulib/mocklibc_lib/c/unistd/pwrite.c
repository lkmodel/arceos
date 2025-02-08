#include "syscall.h"
#include <unistd.h>

ssize_t pwrite(int fd, const void *buf, size_t size, off_t ofs)
{
    return syscall_cp(SYS_pwrite, fd, buf, size, __SYSCALL_LL_PRW(ofs));
}
