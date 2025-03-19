#include <features.h>
#include <setjmp.h>

int _setjmp(jmp_buf env)
{
    __asm__ volatile("sd s0,    0(%0)  \n"
                     "sd s1,    8(%0)  \n"
                     "sd s2,    16(%0) \n"
                     "sd s3,    24(%0) \n"
                     "sd s4,    32(%0) \n"
                     "sd s5,    40(%0) \n"
                     "sd s6,    48(%0) \n"
                     "sd s7,    56(%0) \n"
                     "sd s8,    64(%0) \n"
                     "sd s9,    72(%0) \n"
                     "sd s10,   80(%0) \n"
                     "sd s11,   88(%0) \n"
                     "sd sp,    96(%0) \n"
                     "sd ra,    104(%0) \n"
#ifndef __riscv_float_abi_soft
                     "fsd fs0,  112(%0) \n"
                     "fsd fs1,  120(%0) \n"
                     "fsd fs2,  128(%0) \n"
                     "fsd fs3,  136(%0) \n"
                     "fsd fs4,  144(%0) \n"
                     "fsd fs5,  152(%0) \n"
                     "fsd fs6,  160(%0) \n"
                     "fsd fs7,  168(%0) \n"
                     "fsd fs8,  176(%0) \n"
                     "fsd fs9,  184(%0) \n"
                     "fsd fs10, 192(%0) \n"
                     "fsd fs11, 200(%0) \n"
#endif
                     "li %0, 0      \n" // 设置返回值为 0
                     :
                     : "r"(env)
                     : "memory");

    return 0; // 返回值为 0
}

weak_alias(_setjmp, setjmp);
