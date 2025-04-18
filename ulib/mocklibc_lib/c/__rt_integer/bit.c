#include <__rt_integer.h>
#include <mocklibc.h>
#include <stdint.h>

// These functions return the number of leading 0-bits in `a`, starting at the most significant bit
// position. If `a` is zero, the result is undefined.
int __clzsi2(unsigned int a)
{
    typedef int (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __clzdi2(unsigned long a)
{
    typedef int (*FnABI)(unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __clzti2(unsigned long long a)
{
    typedef int (*FnABI)(unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CLZTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the number of trailing 0-bits in `a`, starting at the least significant
// bit position. If `a` is zero, the result is undefined.
int __ctzsi2(unsigned int a)
{
    typedef int (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CTZSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __ctzdi2(unsigned long a)
{
    typedef int (*FnABI)(unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CTZDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __ctzti2(unsigned long long a)
{
    typedef int (*FnABI)(unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CTZTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the index of the least significant 1-bit in `a`,
// or the value zero if `a` is zero. The least significant bit is index one.
int __ffsdi2(unsigned long a)
{
    typedef int (*FnABI)(unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FFSDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __ffsti2(unsigned long long a)
{
    typedef int (*FnABI)(unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FFSTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the value zero if the number of bits set in `a` is even,
// and the value one otherwise.
int __paritysi2(unsigned int a)
{
    typedef int (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_PARITYSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __paritydi2(unsigned long a)
{
    typedef int (*FnABI)(unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_PARITYDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __parityti2(unsigned long long a)
{
    typedef int (*FnABI)(unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_PARITYTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the number of bits set in `a`.
int __popcountsi2(unsigned int a)
{
    typedef int (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POPCOUNTSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __popcountdi2(unsigned long a)
{
    typedef int (*FnABI)(unsigned long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POPCOUNTDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __popcountti2(unsigned long long a)
{
    typedef int (*FnABI)(unsigned long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POPCOUNTTI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions return the a `byteswapped`.
int32_t __bswapsi2(int32_t a)
{
    typedef int32_t (*FnABI)(int32_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_BSWAPSI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int64_t __bswapdi2(int64_t a)
{
    typedef int64_t (*FnABI)(int64_t);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_BSWAPDI2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
