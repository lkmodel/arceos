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

// 测试函数
void test_read_stdin()
{
    char buffer[128];
    ssize_t result;

    // 测试用例 1: 从标准输入读取有效数据
    printf("Please enter some text (will read up to 128 characters):\n");
    result = read(STDIN_FILENO, buffer, sizeof(buffer) - 1);
    buffer[result] = '\0'; // 确保字符串以 null 结尾
    TEST_RESULT("read valid data from stdin", result >= 0);

    // 测试用例 2: 从标准输入读取零长度数据
    result = read(STDIN_FILENO, buffer, 0);
    TEST_RESULT("read zero length data from stdin", result == 0);

    // 测试用例 3: 使用无效的文件描述符（应失败）
    result = read(-1, buffer, sizeof(buffer));
    TEST_RESULT("read invalid fd", result < 0);

    // 测试用例 4: 从标准输入读取超过缓冲区大小的数据
    printf("Please enter more than 128 characters:\n");
    result = read(STDIN_FILENO, buffer, sizeof(buffer) * 2); // 读取更多数据
    TEST_RESULT("read large data from stdin", result > sizeof(buffer));

    // 测试用例 5: 读取部分数据
    printf("Please enter some text (will read 5 characters):\n");
    result = read(STDIN_FILENO, buffer, 5);
    buffer[result] = '\0'; // 确保字符串以 null 结尾
    TEST_RESULT("read partial data from stdin", result == 5);
}

int main()
{
    // 执行测试
    test_write_stdout();
    test_read_stdin();
    return 0;
}
