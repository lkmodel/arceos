#include <__rt_integer.h>
#include <mocklibc.h>

// These functions perform a signed comparison of `a` and `b`.
// If `a` is less than `b`, they return 0;
// if `a` is greater than `b`, they return 2;
// and if `a` and `b` are equal they return 1.
int __cmpdi2(long a, long b)
{
    typedef int (*FnABI)(long, long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CMPDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __cmpti2(long long a, long long b)
{
    typedef int (*FnABI)(long long, long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CMPTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions perform an unsigned comparison of `a` and `b`.
// If `a` is less than `b`, they return 0;
// if `a` is greater than `b`, they return 2;
// and if `a` and `b` are equal they return 1.
int __ucmpdi2(unsigned long a, unsigned long b)
{
    typedef int (*FnABI)(unsigned long, unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UCMPDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __ucmpti2(unsigned long long a, unsigned long long b)
{
    typedef int (*FnABI)(unsigned long long, unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UCMPTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
