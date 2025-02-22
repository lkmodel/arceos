#include "stdio_impl.h"
// #include <unistd.h>

extern off_t __lseek(int, off_t, int);

off_t __stdio_seek(FILE *f, off_t off, int whence)
{
    return __lseek(f->fd, off, whence);
}
