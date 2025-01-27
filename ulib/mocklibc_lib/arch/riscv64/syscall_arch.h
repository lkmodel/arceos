#include <mocklibc.h>

#define __SYSCALL_LL_E(x) (x)
#define __SYSCALL_LL_O(x) (x)

static inline long __syscall0(long n) {
  return __abi_syscall0(n);
}

static inline long __syscall1(long n, long a)
{
  return __abi_syscall1(n, a);
}

static inline long __syscall2(long n, long a, long b)
{
  return __abi_syscall2(n, a, b);
}

static inline long __syscall3(long n, long a, long b, long c)
{
  return __abi_syscall3(n, a, b, c);
}

static inline long __syscall4(long n, long a, long b, long c, long d)
{
  return __abi_syscall4(n, a, b, c, d);
}

static inline long __syscall5(long n, long a, long b, long c, long d, long e)
{
  return __abi_syscall5(n, a, b, c, d, e);
}

static inline long __syscall6(long n, long a, long b, long c, long d, long e, long f)
{
  return __abi_syscall6(n, a, b, c, d, e, f);
}

#define VDSO_USEFUL
/* We don't have a clock_gettime function.
#define VDSO_CGT_SYM "__vdso_clock_gettime"
#define VDSO_CGT_VER "LINUX_2.6" */

#define IPC_64 0
