#include <__rt_float.h>
#include <mocklibc.h>

// These functions extend a to the wider mode of their return type.
double __extendsfdf2(float a)
{
    typedef double (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extendsftf2(float a)
{
    typedef long double (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extendsfxf2(float a)
{
    typedef long double (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFXF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extenddftf2(double a)
{
    typedef long double (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDDFTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extenddfxf2(double a)
{
    typedef long double (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDDFXF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions truncate a to the narrower mode of their return type, rounding toward zero.
double __truncxfdf2(long double a)
{
    typedef double (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCXFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
double __trunctfdf2(long double a)
{
    typedef double (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCTFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
float __truncxfsf2(long double a)
{
    typedef float (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCXFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
float __trunctfsf2(long double a)
{
    typedef float (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCTFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
float __truncdfsf2(double a)
{
    typedef float (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCDFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to a signed integer, rounding toward zero.
int __fixsfsi(float a)
{
    typedef int (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXSFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __fixdfsi(double a)
{
    typedef int (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXDFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __fixtfsi(long double a)
{
    typedef int (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __fixxfsi(long double a)
{
    typedef int (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXXFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to a signed long, rounding toward zero.
long __fixsfdi(float a)
{
    typedef long (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXSFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __fixdfdi(double a)
{
    typedef long (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXDFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __fixtfdi(long double a)
{
    typedef long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __fixxfdi(long double a)
{
    typedef long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXXFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to a signed long long, rounding toward zero.
long long __fixsfti(float a)
{
    typedef long long (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXSFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __fixdfti(double a)
{
    typedef long long (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXDFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __fixtfti(long double a)
{
    typedef long long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __fixxfti(long double a)
{
    typedef long long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXXFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to an unsigned integer, rounding toward zero. Negative values all
// become zero.
unsigned int __fixunssfsi(float a)
{
    typedef unsigned int (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSSFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned int __fixunsdfsi(double a)
{
    typedef unsigned int (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSDFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned int __fixunstfsi(long double a)
{
    typedef unsigned int (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSTFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned int __fixunsxfsi(long double a)
{
    typedef unsigned int (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSXFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to an unsigned long, rounding toward zero.
// Negative values all become zero.
unsigned long __fixunssfdi(float a)
{
    typedef unsigned long (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSSFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long __fixunsdfdi(double a)
{
    typedef unsigned long (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSDFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long __fixunstfdi(long double a)
{
    typedef unsigned long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSTFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long __fixunsxfdi(long double a)
{
    typedef unsigned long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSXFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to an unsigned long long, rounding toward zero.
// Negative values all become zero.
unsigned long long __fixunssfti(float a)
{
    typedef unsigned long long (*FnABI)(float);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSSFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long long __fixunsdfti(double a)
{
    typedef unsigned long long (*FnABI)(double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSDFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long long __fixunstfti(long double a)
{
    typedef unsigned long long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSTFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long long __fixunsxfti(long double a)
{
    typedef unsigned long long (*FnABI)(long double);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSXFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert `i`, a signed integer, to floating point.
float __floatsisf(int i)
{
    typedef float (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATSISF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
double __floatsidf(int i)
{
    typedef double (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATSIDF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floatsitf(int i)
{
    typedef long double (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATSITF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floatsixf(int i)
{
    typedef long double (*FnABI)(int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATSIXF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}

// These functions convert `i`, a signed long, to floating point.
float __floatdisf(long i)
{
    typedef float (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATDISF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
double __floatdidf(long i)
{
    typedef double (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATDIDF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floatditf(long i)
{
    typedef long double (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATDITF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floatdixf(long i)
{
    typedef long double (*FnABI)(long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATDIXF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}

// These functions convert `i`, a signed long long, to floating point.
float __floattisf(long long i)
{
    typedef float (*FnABI)(long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATTISF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
double __floattidf(long long i)
{
    typedef double (*FnABI)(long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATTIDF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floattitf(long long i)
{
    typedef long double (*FnABI)(long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATTITF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floattixf(long long i)
{
    typedef long double (*FnABI)(long long);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATTIXF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}

// These functions convert `i`, an unsigned integer, to floating point.
float __floatunsisf(unsigned int i)
{
    typedef float (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATUNSISF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
double __floatunsidf(unsigned int i)
{
    typedef double (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATUNSIDF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floatunsitf(unsigned int i)
{
    typedef long double (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATUNSITF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
long double __floatunsixf(unsigned int i)
{
    typedef long double (*FnABI)(unsigned int);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FLOATUNSIXF);
    FnABI func = (FnABI)(*abi_ptr);
    return func(i);
}
