#include <__rt_float.h>
#include <mocklibc.h>

double __muldf3(double a, double b)
{
    typedef double (*FnABI)(double a, double b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_MULDF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
