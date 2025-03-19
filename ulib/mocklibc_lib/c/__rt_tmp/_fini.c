#include <__rt_tmp.h>
#include <mocklibc.h>
#include <unistd.h>

void _fini(void)
{
    _exit(0);
    NOIMPL_STR("_fini\n")
}
