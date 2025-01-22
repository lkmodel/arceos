#ifndef _LIBC_H
#define _LIBC_H

#define SYS_NOIMPL                0
#define SYS_INIT_SCHEDULER        1
#define SYS_PUTCHAR               2
#define SYS_TERMINATE             3
#define SYS_TIMESPEC              4
#define SYS_VFPRINTF              5
#define SYS_VSNPRINTF             6
#define SYS_VSCANF                7
#define SYS_PTHREAD_CREATE        8
#define SYS_PTHREAD_JOIN          9
#define SYS_PTHREAD_EXIT          10
#define SYS_PTHREAD_SELF          11
#define SYS_SLEEP                 12
#define SYS_PTHREAD_MUTEX_INIT    13
#define SYS_PTHREAD_MUTEX_LOCK    14
#define SYS_PTHREAD_MUTEX_UNLOCK  15
#define SYS_PTHREAD_MUTEX_DESTORY 16
#define SYS_OUT                   17
#define SYS_OPEN                  18
#define SYS_LSEEK                 19
#define SYS_STAT                  20
#define SYS_FSTAT                 21
#define SYS_LSTAT                 22
#define SYS_GETCWD                23
#define SYS_RENAME                18

#define NOIMPL                                            \
    typedef int (*FnABI)();                               \
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_NOIMPL); \
    FnABI func = (FnABI)(*abi_ptr);                       \
    func();

extern unsigned long volatile abi_entry;

#include <stdarg.h>
extern int main(int, char **);

#endif
