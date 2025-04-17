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

    // 测试用例 2: 测试打开一个不存在的文件（应失败）
    fd2 = openat(AT_FDCWD, "nonexistent.txt", O_RDONLY);
    TEST_RESULT("openat nonexistent file", fd2 < 0);

    // 测试用例 3: 测试使用无效的标志
    fd1 = openat(AT_FDCWD, "testfile.txt", 99999); // 使用无效标志
    TEST_RESULT("openat invalid flags", fd1 < 0);
    if (fd1 >= 0)
        close(fd1); // 清理

    // 测试用例 4: 测试打开文件并写入数据
    fd1 = openat(AT_FDCWD, "testfile.txt", O_WRONLY);
    TEST_RESULT("openat write to file", fd1 >= 0);
    if (fd1 >= 0)
        close(fd1); // 清理

    // 测试用例 5: 测试打开文件并读取数据
    fd1 = openat(AT_FDCWD, "testfile.txt", O_RDONLY);
    TEST_RESULT("openat read from file", fd1 >= 0);
    if (fd1 >= 0)
        close(fd1); // 清理
}

// 测试函数
void test_fcntl()
{
    int fd;

    // 测试用例 1: 测试打开一个有效文件
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    TEST_RESULT("open valid file", fd >= 0);

    if (fd >= 0) {
        // 测试用例 2: 测试 fcntl 获取文件描述符状态标志
        int flags = fcntl(fd, F_GETFL);
        TEST_RESULT("fcntl get file status flags", flags != -1);

        // 测试用例 3: 测试 fcntl 设置文件描述符状态标志
        int set_result = fcntl(fd, F_SETFL, O_NONBLOCK);
        TEST_RESULT("fcntl set file status flags", set_result != -1);

        // 清理
        close(fd);
        remove("testfile.txt");
    } else {
        printf("\033[31mFailed to open test file.\033[0m\n");
    }

    // 测试用例 4: 测试 fcntl 使用无效的文件描述符
    int invalid_result = fcntl(-1, F_GETFL);
    TEST_RESULT("fcntl invalid fd", invalid_result == -1);
}

// 测试函数
void test_creat()
{
    // 测试用例 1: 测试创建一个有效文件
    int fd1 = creat("testfile2.txt", 0644);
    TEST_RESULT("creat valid file", fd1 >= 0);
    if (fd1 >= 0)
        close(fd1); // 清理

    // 测试用例 2: 再次创建同名文件（应失败）
    int fd2 = creat("testfile.txt", 0644);
    TEST_RESULT("creat existing file", fd2 >= 0);
    if (fd2 >= 0)
        close(fd2); // 清理（如果创建成功）

    // 测试用例 3: 测试使用无效的模式
    int fd3 = creat("testfile.txt", 99999); // 使用无效模式
    TEST_RESULT("creat invalid mode", fd3 < 0);
    if (fd3 >= 0)
        close(fd3); // 清理

    // 清理：删除测试文件
    if (remove("testfile.txt") != 0) {
        printf("Failed to remove test file\n");
    }

    // 测试用例 4: 测试创建文件多次
    for (int i = 0; i < 3; i++) {
        int fd = creat("testfile.txt", 0644);
        TEST_RESULT("creat valid file multiple times", fd >= 0);
        if (fd >= 0)
            close(fd); // 清理
        // 清理：删除测试文件
        if (remove("testfile.txt") != 0) {
            printf("Failed to remove test file\n");
        }
    }
}

int main()
{
    // 执行测试
    test_openat();
    test_fcntl();
    test_creat();
    return 0;
}
