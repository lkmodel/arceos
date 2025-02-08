#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/uio.h>
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

// 测试 remove 函数
void test_remove()
{
    const char *file_to_remove = "testfile.txt";

    // 测试用例 1: 创建一个有效文件
    int fd = open(file_to_remove, O_CREAT | O_RDWR, 0644);
    TEST_RESULT("create test file", fd >= 0);
    if (fd >= 0)
        close(fd); // 清理

    // 测试用例 2: 测试成功移除文件
    for (int i = 0; i < 3; i++) {
        int result = remove(file_to_remove);
        TEST_RESULT("remove valid file", 0 == i ? result == 0 : result != 0);
    }

    // 测试用例 3: 测试移除不存在的文件（应失败）
    int result = remove("nonexistent.txt");
    TEST_RESULT("remove nonexistent file", result != 0);
}

// 测试 rename 函数
void test_rename()
{
    const char *old_file = "oldfile.txt";
    const char *new_file = "newfile.txt";

    // 测试用例 1: 创建一个有效文件
    int fd = open(old_file, O_CREAT | O_RDWR, 0644);
    TEST_RESULT("create old file", fd >= 0);
    if (fd >= 0)
        close(fd); // 清理

    // 测试用例 2: 测试重命名有效文件
    for (int i = 0; i < 3; i++) {
        int result = rename(old_file, new_file);
        TEST_RESULT("rename valid file", result == 0);

        // 清理
        remove(new_file); // 删除新文件
    }

    // 测试用例 3: 测试重命名不存在的文件（应失败）
    int result = rename("nonexistent.txt", new_file);
    TEST_RESULT("rename nonexistent file", result != 0);

    // 测试用例 4: 测试重命名到已存在的文件（应覆盖）
    fd = open(new_file, O_CREAT | O_RDWR, 0644);
    TEST_RESULT("create new file", fd >= 0);
    if (fd >= 0)
        close(fd); // 清理

    result = rename(old_file, new_file); // 应该成功并覆盖
    TEST_RESULT("rename to existing file", result == 0);

    // 清理
    remove(new_file); // 删除新文件

    // 测试用例 5: 测试重命名到相同文件名（应成功）
    fd = open(old_file, O_CREAT | O_RDWR, 0644);
    TEST_RESULT("create old file again", fd >= 0);
    if (fd >= 0)
        close(fd); // 清理

    result = rename(old_file, old_file); // 不应有错误
    TEST_RESULT("rename same file", result == 0);

    // 清理
    remove(old_file); // 删除旧文件
}

// 测试函数
void test_read_write()
{
    int fd;
    const char *test_str = "Hello, world!\n";
    char buffer[128];
    ssize_t result;

    // 测试用例 1: 测试打开一个有效文件进行写入
    fd = open("testfile.txt", O_CREAT | O_WRONLY | O_TRUNC, 0644);
    TEST_RESULT("open valid file for write", fd >= 0);

    if (fd >= 0) {
        // 测试用例 2: 向文件写入数据
        result = write(fd, test_str, strlen(test_str));
        TEST_RESULT("write valid data to file", result == strlen(test_str));
        close(fd); // 清理
    }

    // 测试用例 3: 测试打开文件进行读取
    fd = open("testfile.txt", O_RDONLY);
    TEST_RESULT("open valid file for read", fd >= 0);

    if (fd >= 0) {
        // 测试用例 4: 从文件读取数据
        result = read(fd, buffer, sizeof(buffer) - 1);
        buffer[result] = '\0'; // 确保字符串以 null 结尾
        TEST_RESULT("read data from file", (result > 0) && strcmp(test_str, buffer) == 0);
        close(fd); // 清理
    }

    // 测试用例 5: 测试读取不存在的文件（应失败）
    fd = open("nonexistent.txt", O_RDONLY);
    TEST_RESULT("open nonexistent file", fd < 0);

    // 测试用例 6: 测试无效文件描述符的读取（应失败）
    result = read(-1, buffer, sizeof(buffer));
    TEST_RESULT("read invalid fd", result < 0);

    // 测试用例 7: 测试写入无效文件描述符（应失败）
    result = write(-1, test_str, strlen(test_str));
    TEST_RESULT("write invalid fd", result < 0);
}

// 测试函数
void test_readv_writev()
{
    int fd;
    const char *test_str = "Hello, world!\n";
    char buffer[128];
    struct iovec read_iov[1], write_iov[1];
    ssize_t result;

    // 测试用例 1: 测试打开一个有效文件进行写入
    fd = open("testfile.txt", O_CREAT | O_WRONLY | O_TRUNC, 0644);
    TEST_RESULT("open valid file for write", fd >= 0);

    if (fd >= 0) {
        write_iov[0].iov_base = (void *)test_str;
        write_iov[0].iov_len = strlen(test_str);

        // 测试用例 2: 使用 writev 向文件写入数据
        result = writev(fd, write_iov, 1);
        TEST_RESULT("writev valid data to file", result == strlen(test_str));
        close(fd); // 清理
    }

    // 测试用例 3: 测试打开文件进行读取
    fd = open("testfile.txt", O_RDONLY);
    TEST_RESULT("open valid file for read", fd >= 0);

    if (fd >= 0) {
        read_iov[0].iov_base = buffer;
        read_iov[0].iov_len = sizeof(buffer) - 1;

        // 测试用例 4: 使用 readv 从文件读取数据
        result = readv(fd, read_iov, 1);
        buffer[result] = '\0'; // 确保字符串以 null 结尾
        TEST_RESULT("readv data from file", result > 0);
        close(fd); // 清理
    }

    // 测试用例 5: 测试读取不存在的文件（应失败）
    fd = open("nonexistent.txt", O_RDONLY);
    TEST_RESULT("open nonexistent file", fd < 0);

    // 测试用例 6: 测试无效文件描述符的读取（应失败）
    result = readv(-1, read_iov, 1);
    TEST_RESULT("readv invalid fd", result < 0);

    // 测试用例 7: 测试无效文件描述符的写入（应失败）
    result = writev(-1, write_iov, 1);
    TEST_RESULT("writev invalid fd", result < 0);
}

// 测试函数
void test_pread_pwrite()
{
    int fd;
    const char *test_str = "Hello, world!\n";
    char buffer[128];
    ssize_t result;

    // 测试用例 1: 测试打开一个有效文件进行写入
    fd = open("testfile.txt", O_CREAT | O_WRONLY | O_TRUNC, 0644);
    TEST_RESULT("open valid file for write", fd >= 0);

    if (fd >= 0) {
        // 测试用例 2: 使用 pwrite 向文件写入数据
        result = pwrite(fd, test_str, strlen(test_str), 0);
        TEST_RESULT("pwrite valid data to file", result == strlen(test_str));
        close(fd); // 清理
    }

    // 测试用例 3: 测试打开文件进行读取
    fd = open("testfile.txt", O_RDONLY);
    TEST_RESULT("open valid file for read", fd >= 0);

    if (fd >= 0) {
        // 测试用例 4: 使用 pread 从文件读取数据
        result = pread(fd, buffer, sizeof(buffer) - 1, 0);
        buffer[result] = '\0'; // 确保字符串以 null 结尾
        TEST_RESULT("pread data from file", result > 0);
        close(fd); // 清理
    }

    // 测试用例 5: 测试无效的文件描述符进行读取（应失败）
    result = pread(-1, buffer, sizeof(buffer), 0);
    TEST_RESULT("pread invalid fd", result < 0);

    // 测试用例 6: 测试无效的文件描述符进行写入（应失败）
    result = pwrite(-1, test_str, strlen(test_str), 0);
    TEST_RESULT("pwrite invalid fd", result < 0);

    // 测试用例 7: 测试在偏移量0写入数据
    fd = open("testfile.txt", O_WRONLY);
    result = pwrite(fd, test_str, strlen(test_str), 0);
    TEST_RESULT("pwrite data at offset 0", result == strlen(test_str));
    close(fd); // 清理

    // 测试用例 8: 测试在偏移量10写入数据
    fd = open("testfile.txt", O_WRONLY);
    result = pwrite(fd, "Test", 4, 10);
    TEST_RESULT("pwrite data at offset 10", result == 4);
    close(fd); // 清理
}

int main()
{
    // 执行测试
    printf("test_remove\n");
    test_remove();
    printf("test_rename\n");
    test_rename();
    printf("test_read_write\n");
    test_read_write();
    printf("test_readv_writev\n");
    test_readv_writev();
    printf("test_pread_pwrite\n");
    test_pread_pwrite();
    return 0;
}
