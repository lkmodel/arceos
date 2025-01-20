#include "pthread_impl.h"
#include <mocklibc.h>
#include <threads.h>

static pthread_t __pthread_self_internal()
{
    typedef int (*FnABI)();
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_PTHREAD_SELF);
    FnABI func = (FnABI)(*abi_ptr);
    func();
}

weak_alias(__pthread_self_internal, pthread_self);
weak_alias(__pthread_self_internal, thrd_current);
