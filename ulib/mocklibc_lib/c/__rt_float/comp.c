#include <__rt_float.h>
#include <mocklibc.h>

// These functions calculate `a` <=> `b`.
// That is, if `a` is less than `b`, they return −1;
// if `a` is greater than `b`, they return 1;
// and if `a` and `b` are equal they return 0.
// If either argument is `NaN` they return 1, but you should not rely on this;
// if `NaN` is a possibility, use one of the higher-level comparison functions.
int __cmpsf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CMPSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __cmpdf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CMPDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __cmptf2(long double a, long double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_CMPTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return a nonzero value if either argument is `NaN`, otherwise 0.
int __unordsf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UNORDSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __unorddf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UNORDDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __unordtf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_UNORDTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return zero if neither argument is `NaN`, and `a` and `b` are equal.
int __eqsf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EQSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __eqdf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EQDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __eqtf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EQTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return a nonzero value if either argument is `NaN`,
// or if `a` and `b` are unequal.
int __nesf2(float a, float b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NESF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __nedf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __netf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NETF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return a value greater than or equal to zero if neither argument is `NaN`,
// and `a` is greater than or equal to `b`.
int __gesf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_GESF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __gedf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_GEDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __getf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_GETF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return a value less than zero if neither argument is `NaN`,
// and `a` is strictly less than `b`.
int __ltsf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LTSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __ltdf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LTDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __lttf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LTTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return a value less than or equal to zero if neither argument is `NaN`,
// and `a` is less than or equal to `b`.
int __lesf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LESF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __ledf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LEDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __letf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_LETF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return a value greater than zero if neither argument is `NaN`,
// and `a` is strictly greater than `b`.
int __gtsf2(float a, float b)
{
    typedef int (*FnABI)(float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_GTSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __gtdf2(double a, double b)
{
    typedef int (*FnABI)(double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_GTDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
int __gttf2(long double a, long double b)
{
    typedef int (*FnABI)(long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_GTTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
