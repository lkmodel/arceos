#include <setjmp.h>

_Noreturn void _longjmp(jmp_buf env, int val)
{
    __asm__ volatile("ld s0,    0(%0)  \n"
                     "ld s1,    8(%0)  \n"
                     "ld s2,    16(%0) \n"
                     "ld s3,    24(%0) \n"
                     "ld s4,    32(%0) \n"
                     "ld s5,    40(%0) \n"
                     "ld s6,    48(%0) \n"
                     "ld s7,    56(%0) \n"
                     "ld s8,    64(%0) \n"
                     "ld s9,    72(%0) \n"
                     "ld s10,   80(%0) \n"
                     "ld s11,   88(%0) \n"
                     "ld sp,    96(%0) \n"
                     "ld ra,    104(%0) \n"
#ifndef __riscv_float_abi_soft
                     "fld fs0,  112(%0) \n"
                     "fld fs1,  120(%0) \n"
                     "fld fs2,  128(%0) \n"
                     "fld fs3,  136(%0) \n"
                     "fld fs4,  144(%0) \n"
                     "fld fs5,  152(%0) \n"
                     "fld fs6,  160(%0) \n"
                     "fld fs7,  168(%0) \n"
                     "fld fs8,  176(%0) \n"
                     "fld fs9,  184(%0) \n"
                     "fld fs10, 192(%0) \n"
                     "fld fs11, 200(%0) \n"
#endif
                     "seqz %0, %1   \n"
                     "add %0, %0, %1 \n"
                     "ret"
                     : /* No output operands */
                     : "r"(env), "r"(val)
                     : "memory");
    while (1) {}
}

weak_alias(_longjmp, longjmp);
