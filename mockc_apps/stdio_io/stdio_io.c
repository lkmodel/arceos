#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>

// 测试结果输出宏
#define TEST_RESULT(func, condition)                   \
    do {                                               \
        if (condition) {                               \
            printf("\033[32m%s: PASS\033[0m\n", func); \
        } else {                                       \
            printf("\033[31m%s: BAD\033[0m\n", func);  \
        }                                              \
    } while (0)

// 测试函数
void test_write_stdout()
{
    const char *test_str = "Hello, world!\n";
    ssize_t result;

    // 测试用例 1: 向标准输出写入有效数据
    result = write(STDOUT_FILENO, test_str, strlen(test_str));
    TEST_RESULT("write valid data to stdout", result == strlen(test_str));

    // 测试用例 2: 写入零长度数据
    result = write(STDOUT_FILENO, test_str, 0);
    TEST_RESULT("write zero length data to stdout", result == 0);

    // 测试用例 3: 使用无效的文件描述符（应失败）
    result = write(-1, test_str, strlen(test_str));
    TEST_RESULT("write invalid fd", result < 0);

    // 测试用例 4: 写入超过缓冲区大小的数据
    char large_data[1024];
    memset(large_data, 'A', sizeof(large_data)); // 填充数据
    result = write(STDOUT_FILENO, large_data, sizeof(large_data));
    TEST_RESULT("write large data to stdout", result == sizeof(large_data));

    // 测试用例 5: 写入部分数据
    result = write(STDOUT_FILENO, test_str, 5);
    TEST_RESULT("write partial data to stdout", result == 5);
}

int main()
{
    // 执行测试
    test_write_stdout();
    return 0;
}
