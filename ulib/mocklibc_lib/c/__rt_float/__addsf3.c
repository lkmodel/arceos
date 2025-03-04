#include <__rt_float.h>
#include <mocklibc.h>

float __addsf3(float a, float b)
{
    typedef float (*FnABI)(float a, float b);
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_RT_ADDSF3);
    FnABI func = (FnABI)(*abi_ptr);
    return func(a, b);
}
