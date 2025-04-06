#include <setjmp.h>

int sigsetjmp(sigjmp_buf env, int savemask)
{
    __asm__ volatile("bnez %1, 1f           \n" // 如果 savemask 非零，跳转到标签 1
                     "tail setjmp           \n" // 调用 setjmp
                     "1:                    \n" // 标签 1

                     "sd ra, 208(%0)       \n" // 保存 ra 寄存器
                     "sd s0, 224(%0)       \n" // 保存 s0 寄存器
                     "mv s0, %0            \n" // 将 env 赋值给 s0

                     "call setjmp          \n" // 调用 setjmp

                     "mv %1, a0            \n" // 将返回值保存到 a1（即 env）
                     "mv %0, s0            \n" // 恢复 env
                     "ld s0, 224(%0)       \n" // 恢复 s0
                     "ld ra, 208(%0)       \n" // 恢复 ra

                     ".hidden __sigsetjmp_tail \n" // 隐藏 __sigsetjmp_tail
                     "tail __sigsetjmp_tail \n"    // 调用 __sigsetjmp_tail
                     : "=r"(env), "=r"(savemask)   // 输出操作数
                     : "r"(env), "r"(savemask)     // 输入操作数
                     : "memory"                    // 告诉编译器可能修改内存
    );

    return 0; // 返回值
}
