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
void test_openat()
{
    int fd1, fd2;

    // 测试用例 1: 测试打开一个有效文件
    fd1 = openat(AT_FDCWD, "testfile.txt", O_CREAT | O_RDWR, 0644);
    TEST_RESULT("openat valid file", fd1 >= 0);
    if (fd1 >= 0)
        close(fd1); // 清理

    //    // 测试用例 2: 测试打开一个不存在的文件（应失败）
    //    fd2 = openat(AT_FDCWD, "nonexistent.txt", O_RDONLY);
    //    TEST_RESULT("openat nonexistent file", fd2 < 0);
    //
    //    // 测试用例 3: 测试使用无效的标志
    //    fd1 = openat(AT_FDCWD, "testfile.txt", 99999); // 使用无效标志
    //    TEST_RESULT("openat invalid flags", fd1 < 0);
    //    if (fd1 >= 0)
    //        close(fd1); // 清理
    //
    //    // 测试用例 4: 测试打开文件并写入数据
    //    fd1 = openat(AT_FDCWD, "testfile.txt", O_WRONLY);
    //    TEST_RESULT("openat write to file", fd1 >= 0);
    //    if (fd1 >= 0)
    //        close(fd1); // 清理
    //
    //    // 测试用例 5: 测试打开文件并读取数据
    //    fd1 = openat(AT_FDCWD, "testfile.txt", O_RDONLY);
    //    TEST_RESULT("openat read from file", fd1 >= 0);
    //    if (fd1 >= 0)
    //        close(fd1); // 清理
}

int main()
{
    // 执行测试
    test_openat();
    return 0;
}
