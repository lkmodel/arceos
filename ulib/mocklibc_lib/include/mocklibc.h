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
// `abi_syscall`
#define ABI_SYSCALL0 60
#define ABI_SYSCALL1 61
#define ABI_SYSCALL2 62
#define ABI_SYSCALL3 63
#define ABI_SYSCALL4 64
#define ABI_SYSCALL5 65
#define ABI_SYSCALL6 66

extern unsigned long volatile abi_entry;

#define NOIMPL                                            \
    typedef int (*FnABI)();                               \
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_NOIMPL); \
    FnABI func = (FnABI)(*abi_ptr);                       \
    func();

typedef int (*__fn_abi_syscall0)(long n);
typedef int (*__fn_abi_syscall1)(long n, long a);
typedef int (*__fn_abi_syscall2)(long n, long a, long b);
typedef int (*__fn_abi_syscall3)(long n, long a, long b, long c);
typedef int (*__fn_abi_syscall4)(long n, long a, long b, long c, long d);
typedef int (*__fn_abi_syscall5)(long n, long a, long b, long c, long d, long e);
typedef int (*__fn_abi_syscall6)(long n, long a, long b, long c, long d, long e, long f);

long *__abi_syscall0_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL0);
long *__abi_syscall1_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL1);
long *__abi_syscall2_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL2);
long *__abi_syscall3_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL3);
long *__abi_syscall4_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL4);
long *__abi_syscall5_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL5);
long *__abi_syscall6_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL6);

__fn_abi_syscall0 __abi_syscall0 = (__fn_abi_syscall0)(*__abi_syscall0_ptr);
__fn_abi_syscall1 __abi_syscall1 = (__fn_abi_syscall1)(*__abi_syscall1_ptr);
__fn_abi_syscall2 __abi_syscall2 = (__fn_abi_syscall2)(*__abi_syscall2_ptr);
__fn_abi_syscall3 __abi_syscall3 = (__fn_abi_syscall3)(*__abi_syscall3_ptr);
__fn_abi_syscall4 __abi_syscall4 = (__fn_abi_syscall4)(*__abi_syscall4_ptr);
__fn_abi_syscall5 __abi_syscall5 = (__fn_abi_syscall5)(*__abi_syscall5_ptr);
__fn_abi_syscall6 __abi_syscall6 = (__fn_abi_syscall6)(*__abi_syscall6_ptr);

#include <stdarg.h>
extern int main(int, char **);

void __libc_start_main(long *p);
// void mock_start_main(long *p);
void terminate();

#endif
