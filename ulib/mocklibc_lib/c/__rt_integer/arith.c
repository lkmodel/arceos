#include <__rt_integer.h>
#include <mocklibc.h>

// These functions return the result of shifting a left by b bits.
int __ashlsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ASHLSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __ashldi3(long a, int b)
{
    typedef long (*FnABI)(long, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ASHLDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long long __ashlti3(long long a, int b)
{
    typedef long (*FnABI)(long long, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ASHLTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the result of arithmetically shifting a right by b bits.
int __ashrsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ASHRSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __ashrdi3(long a, int b)
{
    typedef long (*FnABI)(long, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ASHRDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long long __ashrti3(long long a, int b)
{
    typedef long long (*FnABI)(long long, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ASHRTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the quotient of the signed division of `a` and `b`.
int __divsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __divdi3(long a, long b)
{
    typedef long (*FnABI)(long, long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long long __divti3(long long a, long long b)
{
    typedef long long (*FnABI)(long long, long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the result of logically shifting a right by b bits.
int __lshrsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LSHRSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __lshrdi3(long a, int b)
{
    typedef long (*FnABI)(long, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LSHRDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long long __lshrti3(long long a, int b)
{
    typedef long long (*FnABI)(long long, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LSHRTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the remainder of the signed division of `a` and `b`.
int __modsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MODSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __moddi3(long a, long b)
{
    typedef long (*FnABI)(long, long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MODDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long long __modti3(long long a, long long b)
{
    typedef long long (*FnABI)(long long, long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MODTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the product of `a` and `b`.
int __mulsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __muldi3(long a, long b)
{
    typedef long (*FnABI)(long, long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long long __multi3(long long a, long long b)
{
    typedef long long (*FnABI)(long long, long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the negation of `a`.
long __negdi2(long a)
{
    typedef long (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __negti2(long long a)
{
    typedef long long (*FnABI)(long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the quotient of the unsigned division of `a` and `b`.
unsigned int __udivsi3(unsigned int a, unsigned int b)
{
    typedef unsigned int (*FnABI)(unsigned int, unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UDIVSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
unsigned long __udivdi3(unsigned long a, unsigned long b)
{
    typedef unsigned long (*FnABI)(unsigned long, unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UDIVDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
unsigned long long __udivti3(unsigned long long a, unsigned long long b)
{
    typedef unsigned long long (*FnABI)(unsigned long long, unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UDIVTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions calculate both the quotient and remainder of the unsigned division of `a` and
// `b`. The return value is the quotient, and the remainder is placed in variable pointed to by `c`.
unsigned long __udivmoddi4(unsigned long a, unsigned long b, unsigned long *c)
{
    typedef unsigned long (*FnABI)(unsigned long, unsigned long, unsigned long *);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UDIVMODDI4);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c);
}
unsigned long long __udivmodti4(unsigned long long a, unsigned long long b, unsigned long long *c)
{
    typedef unsigned long long (*FnABI)(unsigned long long, unsigned long long,
                                        unsigned long long *);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UDIVMODTI4);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c);
}

// These functions return the remainder of the unsigned division of `a` and `b`.
unsigned int __umodsi3(unsigned int a, unsigned int b)
{
    typedef unsigned int (*FnABI)(unsigned int, unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UMODSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
unsigned long __umoddi3(unsigned long a, unsigned long b)
{
    typedef unsigned long (*FnABI)(unsigned long, unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UMODDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
unsigned long long __umodti3(unsigned long long a, unsigned long long b)
{
    typedef unsigned long long (*FnABI)(unsigned long long, unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UMODTI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
