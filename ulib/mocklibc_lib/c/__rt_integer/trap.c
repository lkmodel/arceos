#include <__rt_integer.h>
#include <mocklibc.h>

// These functions return the absolute value of `a`.
int __absvsi2(int a)
{
    typedef int (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ABSVSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __absvdi2(long a)
{
    typedef long (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ABSVSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the sum of `a` and `b`; that is `a` + `b`.
int __addvsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDVSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __addvdi3(long a, long b)
{
    typedef long (*FnABI)(long, long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDVDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// The functions return the product of `a` and `b`; that is `a` * `b`.
int __mulvsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULVSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __mulvdi3(long a, long b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULVDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the negation of `a`; that is `-a`.
int __negvsi2(int a)
{
    typedef int (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGVSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __negvdi2(long a)
{
    typedef long (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGVDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the difference between `b` and `a`; that is `a` - `b`
int __subvsi3(int a, int b)
{
    typedef int (*FnABI)(int, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_SUBVSI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long __subvdi3(long a, long b)
{
    typedef long (*FnABI)(long, long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_SUBVDI3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
