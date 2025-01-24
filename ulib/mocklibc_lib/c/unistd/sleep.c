#include <mocklibc.h>
#include <time.h>
#include <unistd.h>

unsigned sleep(unsigned seconds)
{
    typedef int (*FnABI)(unsigned);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_SLEEP);
    FnABI func = (FnABI)(*abi_ptr);
    return func(seconds);

    //    struct timespec tv = {.tv_sec = seconds, .tv_nsec = 0};
    //    if (nanosleep(&tv, &tv))
    //        return tv.tv_sec;
    //    return 0;
}
