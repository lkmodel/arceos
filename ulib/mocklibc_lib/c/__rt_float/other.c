#include <__rt_float.h>
#include <mocklibc.h>

// These functions convert raise `a` to the power `b`.
float __powisf2(float a, int b)
{
    typedef float (*FnABI)(float, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POWISF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
double __powidf2(double a, int b)
{
    typedef double (*FnABI)(double, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POWIDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __powitf2(long double a, int b)
{
    typedef long double (*FnABI)(long double, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POWITF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
long double __powixf2(long double a, int b)
{
    typedef long double (*FnABI)(long double, int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_POWIXF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}

// These functions return the product of `a` + `ib` and `c` + `id`,
// following the rules of `C99` Annex `G`.
_Complex float __mulsc3(float a, float b, float c, float d)
{
    typedef _Complex float (*FnABI)(float, float, float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULSC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
_Complex double __muldc3(double a, double b, double c, double d)
{
    typedef _Complex double (*FnABI)(double, double, double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULDC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
_Complex long double __multc3(long double a, long double b, long double c, long double d)
{
    typedef _Complex long double (*FnABI)(long double, long double, long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULTC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
_Complex long double __mulxc3(long double a, long double b, long double c, long double d)
{
    typedef _Complex long double (*FnABI)(long double, long double, long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULXC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}

// These functions return the quotient of `a` + `ib` and `c` + `id`
// (i.e., (`a` + `ib`) / (`c` + `id`)), following the rules of `C99` Annex `G`.
_Complex float __divsc3(float a, float b, float c, float d)
{
    typedef _Complex float (*FnABI)(float, float, float, float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVSC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
_Complex double __divdc3(double a, double b, double c, double d)
{
    typedef _Complex double (*FnABI)(double, double, double, double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVDC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
_Complex long double __divtc3(long double a, long double b, long double c, long double d)
{
    typedef _Complex long double (*FnABI)(long double, long double, long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVTC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
_Complex long double __divxc3(long double a, long double b, long double c, long double d)
{
    typedef _Complex long double (*FnABI)(long double, long double, long double, long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_DIVXC3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b, c, d);
}
