#include <stdio.h>
#include <unistd.h>
#include <stdint.h>
#include <float.h>
#include <limits.h>
#include <string.h>

int main() {
    printf("Hello, world!\n");

    int inum = 1;
    
    printf("the num is %d\n", inum);

    float fnum = 1111.1;

    printf("the num is %f\n", fnum);
    sleep(inum);  // 休眠1秒

    // 1. 基础整数类型打印
    // 处理整数打印时使用内部优化的整数转字符串实现
    int n = -12345;
    unsigned int u = 54321; 
    printf("Integers:\n");
    printf("  %%d:    |%d|\n", n);        // 标准decimal
    printf("  %%5d:   |%5d|\n", n);       // 宽度
    printf("  %%05d:  |%05d|\n", n);      // 零填充
    printf("  %%-5d:  |%-5d|\n", n);      // 左对齐
    printf("  %%+d:   |%+d|\n", n);       // 强制符号
    printf("  %% d:   |% d|\n", n);       // 空格占位符号
    
    // 2. 无符号和不同进制
    printf("\nUnsigned and different bases:\n");
    printf("  %%u:    |%u|\n", u);        // 无符号
    printf("  %%o:    |%o|\n", u);        // 八进制
    printf("  %%x:    |%x|\n", u);        // 小写十六进制
    printf("  %%X:    |%X|\n", u);        // 大写十六进制
    printf("  %%#x:   |%#x|\n", u);       // 带前缀十六进制
    printf("  %%#o:   |%#o|\n", u);       // 带前缀八进制

    // 3. 浮点数处理
    printf("Floating point:\n");
    float d = -123;
    // 在访问浮点数前打印地址
    printf("xxxxxxxxxx\n");
    printf("  %%f:    |%f|\n", d);        // 固定点
    printf("  %%.2f:  |%.2f|\n", d);      // 精度控制
    printf("  %%e:    |%e|\n", d);        // 科学计数(小写)
    printf("  %%E:    |%E|\n", d);        // 科学计数(大写)
    printf("  %%g:    |%g|\n", d);        // 自动选择
    printf("  %%G:    |%G|\n", d);        // 自动选择(大写)
    printf("  %%a:    |%a|\n", d);        // 十六进制浮点

    // 4. 特殊浮点值
    printf("\nSpecial floating point values:\n");
    printf("  INFINITY:  |%f|\n", 1.0/0.0);
    printf("  -INFINITY: |%f|\n", -1.0/0.0);
    printf("  NAN:      |%f|\n", 0.0/0.0);

    // 5. 字符和字符串
    char c = 'X';
    const char *s = "test string";
    printf("\nCharacters and strings:\n");
    printf("  %%c:    |%c|\n", c);        // 字符
    printf("  %%3c:   |%3c|\n", c);       // 宽度控制的字符
    printf("  %%-3c:  |%-3c|\n", c);      // 左对齐字符
    printf("  %%s:    |%s|\n", s);        // 字符串
    printf("  %%10s:  |%10s|\n", s);      // 宽度控制
    printf("  %%.5s:  |%.5s|\n", s);      // 精度控制(截断)
    printf("  %%-10s: |%-10s|\n", s);     // 左对齐

    // 6. 指针和长度修饰符
    void *ptr = (void*)&n;
    long l = LONG_MAX;
    long long ll = LLONG_MAX;
    size_t sz = SIZE_MAX;
    
    printf("\nPointers and length modifiers:\n");
    printf("  %%p:     |%p|\n", ptr);     // 指针
    printf("  %%ld:    |%ld|\n", l);      // long
    printf("  %%lld:   |%lld|\n", ll);    // long long
    
    // 7. 边界情况
    printf("\nEdge cases:\n");
    printf("  Empty string: |%s|\n", "");          // 空字符串
    printf("  Zero: |%d|\n", 0);                   // 零值
    printf("  MIN_INT: |%d|\n", INT_MIN);          // 最小整数
    printf("  MAX_INT: |%d|\n", INT_MAX);          // 最大整数

    // 8. 百分号和转义序列
    printf("\nPercent and escape sequences:\n");
    printf("  %%%%:     |%%|\n");         // 百分号
    printf("  Newline: |\n|");            // 换行
    printf("  Tab:     |\t|");            // 制表符
    printf("  Return:  |\r|\n");            // 回车

    return 0;
}