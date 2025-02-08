#include "pthread_impl.h"
#include <errno.h>
// REMOVE
#include <stdio.h>

int *__errno_location(void)
{
    return &__pthread_self()->errno_val; // 取得`errno_val`的地址
}

weak_alias(__errno_location, ___errno_location);
