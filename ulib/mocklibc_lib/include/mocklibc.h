#ifndef _LIBC_H
#define _LIBC_H

// `0-10提供ArceOS相关ABI调用`
#define ABI_NOIMPL         0
#define ABI_INIT_SCHEDULER 1
#define ABI_TERMINATE      2
#define ABI_CHECKPOINT     3
// `stdio`
#define ABI_TIMESPEC 11
// `pthread`
#define ABI_PTHREAD_CREATE        20
#define ABI_PTHREAD_JOIN          21
#define ABI_PTHREAD_EXIT          22
#define ABI_PTHREAD_SELF          23
#define ABI_PTHREAD_MUTEX_INIT    24
#define ABI_PTHREAD_MUTEX_LOCK    25
#define ABI_PTHREAD_MUTEX_UNLOCK  26
#define ABI_PTHREAD_MUTEX_DESTORY 27
// `malloc`
#define ABI_MALLOC  40
#define ABI_CALLOC  41
#define ABI_REALLOC 42
#define ABI_FREE    43
// `unistd`
#define ABI_SLEEP 50
// `abi_syscall`
#define ABI_SYSCALL0 60
#define ABI_SYSCALL1 61
#define ABI_SYSCALL2 62
#define ABI_SYSCALL3 63
#define ABI_SYSCALL4 64
#define ABI_SYSCALL5 65
#define ABI_SYSCALL6 66
// `rt_float`的实现
// Arithmetic functions[100, 119]
#define ABI_RT_ADDSF3 100
#define ABI_RT_ADDDF3 101
#define ABI_RT_ADDTF3 102
#define ABI_RT_ADDXF3 103

#define ABI_RT_SUBSF3 104
#define ABI_RT_SUBDF3 105
#define ABI_RT_SUBTF3 106
#define ABI_RT_SUBXF3 107

#define ABI_RT_MULSF3 108
#define ABI_RT_MULDF3 109
#define ABI_RT_MULTF3 110
#define ABI_RT_MULXF3 111

#define ABI_RT_DIVSF3 112
#define ABI_RT_DIVDF3 113
#define ABI_RT_DIVTF3 114
#define ABI_RT_DIVXF3 115

#define ABI_RT_NEGSF2 116
#define ABI_RT_NEGDF2 117
#define ABI_RT_NEGTF2 118
#define ABI_RT_NEGXF2 119
// Conversion functions[120, 187]
#define ABI_RT_EXTENDSFDF2 120
#define ABI_RT_EXTENDSFTF2 121
#define ABI_RT_EXTENDSFXF2 122
#define ABI_RT_EXTENDDFTF2 123
#define ABI_RT_EXTENDDFXF2 124

#define ABI_RT_TRUNCXFDF2 125
#define ABI_RT_TRUNCTFDF2 126
#define ABI_RT_TRUNCXFSF2 127
#define ABI_RT_TRUNCTFSF2 128
#define ABI_RT_TRUNCDFSF2 129

#define ABI_RT_FIXSFSI 130
#define ABI_RT_FIXDFSI 131
#define ABI_RT_FIXTFSI 132
#define ABI_RT_FIXXFSI 133

#define ABI_RT_FIXSFDI 134
#define ABI_RT_FIXDFDI 135
#define ABI_RT_FIXTFDI 136
#define ABI_RT_FIXXFDI 137

#define ABI_RT_FIXSFTI 138
#define ABI_RT_FIXDFTI 139
#define ABI_RT_FIXTFTI 140
#define ABI_RT_FIXXFTI 141

#define ABI_RT_FIXUNSSFSI 142
#define ABI_RT_FIXUNSDFSI 143
#define ABI_RT_FIXUNSTFSI 144
#define ABI_RT_FIXUNSXFSI 145

#define ABI_RT_FLOATSITF   156
#define ABI_RT_FLOATDITF   160
#define ABI_RT_FLOATUNSITF 168
#define ABI_RT_EQTF2       196
#define ABI_RT_NETF2       199
#define ABI_RT_GETF2       202
#define ABI_RT_LTTF2       205
#define ABI_RT_LETF2       208
#define ABI_RT_GTTF2       211

#define ABI_RT_CLZSI2 271
#define ABI_RT_CLZDI2 272
#define ABI_RT_CLZTI2 273

#define ABI_RT_BSWAPSI2 285
#define ABI_RT_BSWAPDI2 286

extern unsigned long volatile abi_entry;

#define __DEBUG__

// 调试程序的可变参数宏定义
#ifdef __DEBUG__
#include <stdio.h>
#define DEBUG_PRINT(...) printf(__VA_ARGS__);
#else
#define DEBUG_PRINT(...)
#endif

#define NOIMPL_STR(str)                                   \
    DEBUG_PRINT(str)                                      \
    typedef int (*FnABI)();                               \
    long *abi_ptr = (long *)(abi_entry + 8 * ABI_NOIMPL); \
    FnABI func = (FnABI)(*abi_ptr);                       \
    func();

#define ASM_CHECKPOINT(val)                \
    __asm__ volatile("nop\n"               \
                     "nop\n"               \
                     "addi zero,zero,%0\n" \
                     "nop\n"               \
                     "nop\n"               \
                     :                     \
                     : "i"(val));

#define CHECKPOINT                                                                \
    typedef int (*FnABI_CheckPoint)();                                            \
    long *abi_ptr_check_point = (long *)(abi_entry + 8 * ABI_CHECKPOINT);         \
    FnABI_CheckPoint func_check_point = (FnABI_CheckPoint)(*abi_ptr_check_point); \
    func_check_point();

#include <stdarg.h>
extern int main(int, char **);

void __libc_start_main(long *p);
// void mock_start_main(long *p);
void terminate();

#endif
