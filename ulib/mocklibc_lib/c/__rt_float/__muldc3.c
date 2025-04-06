#include <__rt_float.h>
#include <mocklibc.h>
#include <stdio.h>

_Complex double __muldc3(double, double, double, double)
{
    printf("in: __muldc3\n");
    NOIMPL
}
