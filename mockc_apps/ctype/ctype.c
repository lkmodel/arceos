#include <ctype.h>
#include <stdio.h>

// 颜色宏
#define RED   "\033[31m"
#define GREEN "\033[32m"
#define RESET "\033[0m"

// 测试宏
#define TEST(func, input, expected)                                                              \
    do {                                                                                         \
        int result = func(input);                                                                \
        if (result == expected) {                                                                \
            printf(GREEN "%s: PASS\n" RESET, #func);                                             \
        } else {                                                                                 \
            printf(RED "%s: BAD - Expected %d but got %d for input %d\n" RESET, #func, expected, \
                   result, input);                                                               \
        }                                                                                        \
    } while (0)

// 测试函数
void run_tests()
{
    // 测试 isalnum
    TEST(isalnum, 'A', 1);
    TEST(isalnum, '1', 1);
    TEST(isalnum, '@', 0);

    // 测试 isalpha
    TEST(isalpha, 'A', 1);
    TEST(isalpha, '1', 0);
    TEST(isalpha, '@', 0);

    // 测试 isblank
    TEST(isblank, ' ', 1);
    TEST(isblank, '\t', 1);
    TEST(isblank, 'A', 0);

    // 测试 iscntrl
    TEST(iscntrl, '\n', 1);
    TEST(iscntrl, 'A', 0);
    TEST(iscntrl, 0x1F, 1);

    // 测试 isdigit
    TEST(isdigit, '0', 1);
    TEST(isdigit, '9', 1);
    TEST(isdigit, 'A', 0);

    // 测试 isgraph
    TEST(isgraph, 'A', 1);
    TEST(isgraph, ' ', 0);
    TEST(isgraph, '@', 1);

    // 测试 islower
    TEST(islower, 'a', 1);
    TEST(islower, 'A', 0);
    TEST(islower, '1', 0);

    // 测试 isprint
    TEST(isprint, 'A', 1);
    TEST(isprint, '\n', 0);
    TEST(isprint, ' ', 1);

    // 测试 ispunct
    TEST(ispunct, '!', 1);
    TEST(ispunct, 'A', 0);
    TEST(ispunct, ' ', 0);

    // 测试 isspace
    TEST(isspace, ' ', 1);
    TEST(isspace, '\n', 1);
    TEST(isspace, 'A', 0);

    // 测试 isupper
    TEST(isupper, 'A', 1);
    TEST(isupper, 'a', 0);
    TEST(isupper, '1', 0);

    // 测试 isxdigit
    TEST(isxdigit, 'A', 1);
    TEST(isxdigit, '1', 1);
    TEST(isxdigit, 'G', 0);
}

// 主函数
int main()
{
    run_tests();
    return 0;
}
