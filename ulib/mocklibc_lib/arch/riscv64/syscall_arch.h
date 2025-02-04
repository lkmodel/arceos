#include <mocklibc.h>

#define __SYSCALL_LL_E(x) (x)
#define __SYSCALL_LL_O(x) (x)

static inline long __syscall0(long n)
{
    typedef int (*__fn_abi_syscall0)(long n);
    long *__abi_syscall0_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL0);
    __fn_abi_syscall0 __abi_syscall0 = (__fn_abi_syscall0)(*__abi_syscall0_ptr);
    return __abi_syscall0(n);
}

static inline long __syscall1(long n, long a)
{
    typedef int (*__fn_abi_syscall1)(long n, long a);
    long *__abi_syscall1_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL1);
    __fn_abi_syscall1 __abi_syscall1 = (__fn_abi_syscall1)(*__abi_syscall1_ptr);
    return __abi_syscall1(n, a);
}

static inline long __syscall2(long n, long a, long b)
{
    typedef int (*__fn_abi_syscall2)(long n, long a, long b);
    long *__abi_syscall2_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL2);
    __fn_abi_syscall2 __abi_syscall2 = (__fn_abi_syscall2)(*__abi_syscall2_ptr);
    return __abi_syscall2(n, a, b);
}

static inline long __syscall3(long n, long a, long b, long c)
{
    typedef int (*__fn_abi_syscall3)(long n, long a, long b, long c);
    long *__abi_syscall3_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL3);
    __fn_abi_syscall3 __abi_syscall3 = (__fn_abi_syscall3)(*__abi_syscall3_ptr);
    return __abi_syscall3(n, a, b, c);
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
    typedef int (*__fn_abi_syscall4)(long n, long a, long b, long c, long d);
    long *__abi_syscall4_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL4);
    __fn_abi_syscall4 __abi_syscall4 = (__fn_abi_syscall4)(*__abi_syscall4_ptr);
    return __abi_syscall4(n, a, b, c, d);
}

static inline long __syscall5(long n, long a, long b, long c, long d, long e)
{
    typedef int (*__fn_abi_syscall5)(long n, long a, long b, long c, long d, long e);
    long *__abi_syscall5_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL5);
    __fn_abi_syscall5 __abi_syscall5 = (__fn_abi_syscall5)(*__abi_syscall5_ptr);
    return __abi_syscall5(n, a, b, c, d, e);
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
    typedef int (*__fn_abi_syscall6)(long n, long a, long b, long c, long d, long e, long f);
    long *__abi_syscall6_ptr = (long *)(abi_entry + 8 * ABI_SYSCALL6);
    __fn_abi_syscall6 __abi_syscall6 = (__fn_abi_syscall6)(*__abi_syscall6_ptr);
    return __abi_syscall6(n, a, b, c, d, e, f);
}

#define VDSO_USEFUL
/* We don't have a clock_gettime function.
#define VDSO_CGT_SYM "__vdso_clock_gettime"
#define VDSO_CGT_VER "LINUX_2.6" */

#define IPC_64 0
