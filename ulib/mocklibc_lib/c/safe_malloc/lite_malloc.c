#include "fork_impl.h"
#include "libc.h"
#include "lock.h"
#include "syscall.h"
#include <errno.h>
#include <limits.h>
#include <stdint.h>
#include <stdlib.h>
#include <sys/mman.h>

#define ALIGN 16

/* This function returns true if the interval [old,new]
 * intersects the 'len'-sized interval below &libc.auxv
 * (interpreted as the main-thread stack) or below &b
 * (the current stack). It is used to defend against
 * buggy brk implementations that can cross the stack. */

static int traverses_stack_p(uintptr_t old, uintptr_t new)
{
    const uintptr_t len = 8 << 20;
    uintptr_t a, b;

    b = (uintptr_t)libc.auxv;
    a = b > len ? b - len : 0;
    if (new > a && old < b)
        return 1;

    b = (uintptr_t)&b;
    a = b > len ? b - len : 0;
    if (new > a && old < b)
        return 1;

    return 0;
}

static volatile int lock[1];
volatile int *const __bump_lockptr = lock;

#include <mocklibc.h>
static void *__simple_malloc(size_t n)
{
    typedef void *(*FnABI)(size_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_MALLOC);
    FnABI func = (FnABI)(*abi_ptr);
    return func(n);
}

weak_alias(__simple_malloc, __libc_malloc_impl);

void *__libc_malloc(size_t n)
{
    return __libc_malloc_impl(n);
}

static void *default_malloc(size_t n)
{
    return __libc_malloc_impl(n);
}

weak_alias(default_malloc, malloc);
