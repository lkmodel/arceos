#include <mocklibc.h>
#include <stdint.h>

void __cp_end(void)
{
    NOIMPL_STR("__cp_end\n");
}

void __cp_cancel(void)
{
    NOIMPL_STR("__cp_cancel\n");
}

void __cp_begin(void)
{
    NOIMPL_STR("__cp_begin\n");
}

void __syscall_cp_asm(uintptr_t *arg0, uintptr_t arg1, uintptr_t arg2, uintptr_t arg3,
                      uintptr_t arg4, uintptr_t arg5)
{

    NOIMPL_STR("__syscall_cp_asm\n");
    //    uintptr_t t0;
    //
    //    // 嵌入汇编代码
    //    __asm__ volatile("lw %0, 0(%1)        \n" // 加载 arg0 的值到 t0
    //                     "bnez %0, __cp_cancel\n" // 如果 t0 不为零，跳转到 __cp_cancel
    //
    //                     "mv %0, %2           \n" // 移动 arg1 到 t0
    //                     "mv %1, %3           \n" // 移动 arg2 到 a0
    //                     "mv %2, %4           \n" // 移动 arg3 到 a1
    //                     "mv %3, %5           \n" // 移动 arg4 到 a2
    //                     "mv %4, %6           \n" // 移动 arg5 到 a3
    //                     "mv %5, a4           \n" // 移动 a4
    //                     "mv %6, a5           \n" // 移动 a5
    //                     "ld a6, 0(sp)        \n" // 从栈中加载 a6
    //                     "mv a7, %0          \n"  // 将 t0 移动到 a7
    //                     "ecall               \n" // 发起系统调用
    //                     "__cp_end:          \n"  // 标签 __cp_end
    //                     "ret                 \n" // 返回
    //                     "__cp_cancel:       \n"  // 标签 __cp_cancel
    //                     "tail __cancel      \n"  // 调用 __cancel
    //                     : "=r"(t0)               // 输出操作数
    //                     : "r"(arg0), "r"(arg1), "r"(arg2), "r"(arg3), "r"(arg4),
    //                       "r"(arg5) // 输入操作数
    //                     : "memory"  // 告诉编译器可能修改内存
    //    );
}
