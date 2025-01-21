#ifndef _LIBC_H
#define _LIBC_H

#define SYS_NOIMPL         0
#define SYS_HELLO          1
#define SYS_PUTCHAR        2
#define SYS_TERMINATE      3
#define SYS_TIMESPEC       4
#define SYS_VFPRINTF       5
#define SYS_VSNPRINTF      6
#define SYS_VSCANF         7
#define SYS_PTHREAD_CREATE 8
#define SYS_PTHREAD_JOIN   9
#define SYS_PTHREAD_EXIT   10
#define SYS_PTHREAD_SELF   11
#define SYS_SLEEP          15
#define SYS_OUT            16
#define SYS_OPEN           17
#define SYS_LSEEK          18
#define SYS_STAT           19
#define SYS_FSTAT          20
#define SYS_LSTAT          21
#define SYS_GETCWD         22
#define SYS_RENAME         23

#define NOIMPL                                            \
    typedef int (*FnABI)();                               \
    long *abi_ptr = (long *)(abi_entry + 8 * SYS_NOIMPL); \
    FnABI func = (FnABI)(*abi_ptr);                       \
    func();

// unsigned long volatile abi_entry = 0;
extern unsigned long volatile abi_entry;

#include <stdarg.h>
extern int main(int, char **);

#endif
