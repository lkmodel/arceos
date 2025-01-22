#include "crt_arch.h"
#include <mocklibc.h>

void terminate();

unsigned long volatile abi_entry = 0;
__attribute__((visibility("hidden"))) void _start_c(long *p)
{
    __asm__ volatile("mv %0, a7" : "=r"(abi_entry));
    int argc = p[0];
    char **argv = (void *)(p + 1);

    init_scheduler();

    main(argc, argv);

    terminate();
}

void init_scheduler()
{
    typedef void (*Fn)();
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_INIT_SCHEDULER);
    Fn func = (Fn)(*abi_ptr);
    func();
}

void terminate()
{
    typedef void (*Fn)();
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_TERMINATE);
    Fn func = (Fn)(*abi_ptr);
    func();
}
