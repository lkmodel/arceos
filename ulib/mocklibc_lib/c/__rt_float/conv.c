#include <__rt_float.h>
#include <mocklibc.h>

// These functions extend a to the wider mode of their return type.
double __extendsfdf2(float a)
{
    typedef double (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extendsftf2(float a)
{
    typedef long double (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extendsfxf2(float a)
{
    typedef long double (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDSFXF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extenddftf2(double a)
{
    typedef long double (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDDFTF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long double __extenddfxf2(double a)
{
    typedef long double (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_EXTENDDFXF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions truncate a to the narrower mode of their return type, rounding toward zero.
double __truncxfdf2(long double a)
{
    typedef double (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCXFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
double __trunctfdf2(long double a)
{
    typedef double (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCTFDF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
float __truncxfsf2(long double a)
{
    typedef float (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCXFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
float __trunctfsf2(long double a)
{
    typedef float (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCTFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
float __truncdfsf2(double a)
{
    typedef float (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_TRUNCDFSF2);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to a signed integer, rounding toward zero.
int __fixsfsi(float a)
{
    typedef int (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXSFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __fixdfsi(double a)
{
    typedef int (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXDFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __fixtfsi(long double a)
{
    typedef int (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
int __fixxfsi(long double a)
{
    typedef int (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXXFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to a signed long, rounding toward zero.
long __fixsfdi(float a)
{
    typedef long (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXSFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __fixdfdi(double a)
{
    typedef long (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXDFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __fixtfdi(long double a)
{
    typedef long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long __fixxfdi(long double a)
{
    typedef long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXXFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to a signed long long, rounding toward zero.
long long __fixsfti(float a)
{
    typedef long long (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXSFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __fixdfti(double a)
{
    typedef long long (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXDFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __fixtfti(long double a)
{
    typedef long long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXTFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
long long __fixxfti(long double a)
{
    typedef long long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXXFTI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to an unsigned integer, rounding toward zero. Negative values all
// become zero.
unsigned int __fixunssfsi(float a)
{
    typedef unsigned int (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSSFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned int __fixunsdfsi(double a)
{
    typedef unsigned int (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSDFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned int __fixunstfsi(long double a)
{
    typedef unsigned int (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSTFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned int __fixunsxfsi(long double a)
{
    typedef unsigned int (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSXFSI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}

// These functions convert a to an unsigned long, rounding toward zero.
// Negative values all become zero.
unsigned long __fixunssfdi(float a)
{
    typedef unsigned long (*FnABI)(float a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSSFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long __fixunsdfdi(double a)
{
    typedef unsigned long (*FnABI)(double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSDFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long __fixunstfdi(long double a)
{
    typedef unsigned long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSTFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
unsigned long __fixunsxfdi(long double a)
{
    typedef unsigned long (*FnABI)(long double a);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_FIXUNSXFDI);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a);
}
