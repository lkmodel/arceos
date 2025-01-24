#ifndef _LIBC_H
#define _LIBC_H

// `0-10提供ArceOS相关ABI调用`
#define ABI_NOIMPL         0
#define ABI_INIT_SCHEDULER 1
#define ABI_TERMINATE      2
// `stdio`
#define ABI_PUTCHAR   10
#define ABI_TIMESPEC  11
#define ABI_VFPRINTF  12
#define ABI_VSNPRINTF 13
#define ABI_VSCANF    14
#define ABI_OUT       15
// `pthread`
#define ABI_PTHREAD_CREATE        20
#define ABI_PTHREAD_JOIN          21
#define ABI_PTHREAD_EXIT          22
#define ABI_PTHREAD_SELF          23
#define ABI_PTHREAD_MUTEX_INIT    24
#define ABI_PTHREAD_MUTEX_LOCK    25
#define ABI_PTHREAD_MUTEX_UNLOCK  26
#define ABI_PTHREAD_MUTEX_DESTORY 27
// `file`
#define ABI_OPEN   30
#define ABI_LSEEK  31
#define ABI_STAT   32
#define ABI_FSTAT  33
#define ABI_LSTAT  34
#define ABI_GETCWD 35
#define ABI_RENAME 36
// `malloc`
#define ABI_MALLOC  40
#define ABI_CALLOC  41
#define ABI_REALLOC 42
#define ABI_FREE    43
// `unistd`
#define ABI_SLEEP 50

#define NOIMPL                                            \
    typedef int (*FnABI)();                               \
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_NOIMPL); \
    FnABI func = (FnABI)(*abi_ptr);                       \
    func();

extern unsigned long volatile abi_entry;

#include <stdarg.h>
extern int main(int, char **);

void __libc_start_main(long *p);
// void mock_start_main(long *p);
void terminate();

#endif
