#ifndef MALLOC_GLUE_H
#define MALLOC_GLUE_H

// #include "atomic.h"
// #include "dynlink.h"
// #include "libc.h"
// #include "lock.h"
// #include "syscall.h"
// #include <elf.h>
#include <pthread.h>
#include <stdint.h>
#include <string.h>
#include <sys/mman.h>
#include <unistd.h>

// use macros to appropriately namespace these.
#define size_classes __malloc_size_classes

#endif
