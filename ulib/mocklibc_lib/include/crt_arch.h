// #define START "_start"
// asm(".section .sdata\n"
//     ".text\n"
//     ".global " START "\n"
//     ".type " START ",%function\n" START ":\n"
//     ".weak __global_pointer$\n"
//     ".hidden __global_pointer$\n"
//     ".option push\n"
//     ".option norelax\n\t"
//     "lla gp, __global_pointer$\n"
//     ".option pop\n\t"
//     "mv a0, sp\n"
//     "andi sp, sp, -16\n\t"
//     "tail " START "_c");

// asm(".section .sdata\n"            // 数据段
//     ".text\n"                      // 代码段
//     ".global " START "\n"          // 定义全局符号 _start
//     ".type " START ", %function\n" // 指定 _start 为函数类型
//     START ":\n"                    // _start 函数定义
//     ".weak __global_pointer$\n"    // 声明全局指针为弱符号
//     ".hidden __global_pointer$\n"  // 将全局指针隐藏
//     ".option push\n"               // 保存当前选项
//     ".option norelax\n\t"          // 禁用重定位
//     "lla gp, __global_pointer$\n"  // 加载全局指针到 gp
//     ".option pop\n\t"              // 恢复选项
//     "mv a0, sp\n"                  // 将堆栈指针赋值给 a0
//     "andi sp, sp, -16\n\t"         // 堆栈对齐
//     "tail " START "_c"             // 调用 _start_c 函数
// );
