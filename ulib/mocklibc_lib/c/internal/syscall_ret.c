#include "syscall.h"
#include <errno.h>

long __syscall_ret(unsigned long r)
{
    if (r > -4096UL) {
        // FIXME: 尝试使用errno
        //        errno = -r;
        return -1;
    }
    return r;
}
