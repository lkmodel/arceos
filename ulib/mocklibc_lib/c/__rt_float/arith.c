#include <__rt_float.h>
#include <mocklibc.h>

// These functions return the sum of `a` and `b`.
float __addsf3(float a, float b)
{
    typedef float (*FnABI)(float a, float b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDSF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
double __adddf3(double a, double b)
{
    typedef double (*FnABI)(double a, double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDDF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __addtf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDTF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __addxf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDXF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the difference between `b` and `a`; that is, `a` - `b`.
float __subsf3(float a, float b)
{
    typedef float (*FnABI)(float a, float b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_SUBSF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
double __subdf3(double a, double b)
{
    typedef double (*FnABI)(double a, double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_SUBDF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __subtf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_SUBTF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __subxf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_SUBXF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the product of `a` and `b`.
float __mulsf3(float a, float b)
{
    typedef float (*FnABI)(float a, float b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULSF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
double __muldf3(double a, double b)
{
    typedef double (*FnABI)(double a, double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULDF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __multf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULTF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __mulxf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULXF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the quotient of `a` and `b`; that is, `a` / `b`.
float __divsf3(float a, float b)
{
    typedef float (*FnABI)(float a, float b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVSF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
double __divdf3(double a, double b)
{
    typedef double (*FnABI)(double a, double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVDF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __divtf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVTF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __divxf3(long double a, long double b)
{
    typedef long double (*FnABI)(long double a, long double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVXF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the negation of `a`. They simply flip the sign bit, so they can produce
// negative zero and negative `NaN`.
float __negsf2(float a)
{
    typedef float (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
double __negdf2(double a)
{
    typedef double (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __negtf2(long double a)
{
    typedef long double (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __negxf2(long double a)
{
    typedef long double (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_NEGXF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
