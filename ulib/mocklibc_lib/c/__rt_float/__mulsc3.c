#include <__rt_float.h>
#include <mocklibc.h>
#include <stdio.h>

_Complex float __mulsc3(float, float, float, float)
{
    printf("in: mulsc3\n");
    NOIMPL
}
