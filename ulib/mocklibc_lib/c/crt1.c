#include "crt_arch.h"
#include <mocklibc.h>

unsigned long volatile abi_entry = 0;

__attribute__((visibility("hidden"))) void _start(long *p)
{
    __asm__ volatile("mv %0, a7" : "=r"(abi_entry));
    int argc = p[0];
    char **argv = (void *)(p + 1);

    main(argc, argv);

    terminate();
}

/// void mock_start_main(long *p)
void __libc_start_main(long *p)
{
    __asm__ volatile("mv %0, a7" : "=r"(abi_entry));
    int argc = p[0];
    char **argv = (void *)(p + 1);

    main(argc, argv);

    terminate();
}

void terminate()
{
    typedef void (*Fn)();
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_TERMINATE);
    Fn func = (Fn)(*abi_ptr);
    return func();
}
