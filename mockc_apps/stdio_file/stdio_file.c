#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <string.h>
#include <sys/sendfile.h>
#include <sys/stat.h>
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

// 测试结果输出宏
#define TEST_RESULT_ERRNO(func, condition, expected_errno)                           \
    do {                                                                             \
        if (condition && errno == expected_errno) {                                  \
            printf("\033[32m%s: PASS\033[0m\n", func);                               \
        } else {                                                                     \
            printf("\033[31m%s: BAD\033[0m\n", func);                                \
            printf("Expected errno: %d, Actual errno: %d\n", expected_errno, errno); \
        }                                                                            \
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

// 测试函数
void test_dup()
{
    int fd1, fd2;

    // 测试用例 1: 正常情况下复制一个有效的文件描述符
    errno = 0; // 重置 errno
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    TEST_RESULT_ERRNO("dup valid fd", (fd2 = dup(fd1)) >= 0, 0);
    if (fd1 >= 0)
        close(fd1);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 2: 测试重复调用 dup
    errno = 0; // 重置 errno
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    dup(fd1); // 第一次成功
    TEST_RESULT_ERRNO("dup valid fd twice", (fd2 = dup(fd1)) >= 0, 0);
    if (fd1 >= 0)
        close(fd1);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 3: 测试复制一个无效的文件描述符（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup invalid fd", (fd2 = dup(-1)) < 0, EBADF);

    // 测试用例 4: 测试复制一个已关闭的文件描述符（应失败）
    errno = 0; // 重置 errno
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    close(fd1); // 关闭 fd1
    TEST_RESULT_ERRNO("dup closed fd", (fd2 = dup(fd1)) < 0, EBADF);

    // 测试用例 5: 边界测试：复制标准输入（fd = 0）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup stdin", (fd2 = dup(0)) >= 0, 0);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 6: 边界测试：复制标准输出（fd = 1）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup stdout", (fd2 = dup(1)) >= 0, 0);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 7: 边界测试：复制标准错误（fd = 2）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup stderr", (fd2 = dup(2)) >= 0, 0);
    if (fd2 >= 0)
        close(fd2); // 清理

    remove("testfile.txt");
}

// 测试函数
void test_dup3()
{
    int fd1, fd2;

    // 测试用例 1: 正常情况下复制一个有效的文件描述符
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 valid fd", (fd2 = dup3(fd1, 100, 0)) >= 0, 0);
    if (fd1 >= 0)
        close(fd1);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 2: 测试重复调用 dup3
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 valid fd twice", (fd2 = dup3(fd1, 101, 0)) >= 0, 0);
    if (fd1 >= 0)
        close(fd1);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 3: 测试复制一个无效的文件描述符（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 invalid fd", (fd2 = dup3(-1, 102, 0)) < 0, EBADF);

    // 测试用例 4: 测试复制一个已关闭的文件描述符（应失败）
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    close(fd1); // 关闭 fd1
    errno = 0;  // 重置 errno
    TEST_RESULT_ERRNO("dup3 closed fd", (fd2 = dup3(fd1, 103, 0)) < 0, EBADF);

    // 测试用例 5: 测试使用无效的目标文件描述符（应失败）
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 invalid target fd", (fd2 = dup3(fd1, -1, 0)) < 0, EBADF);
    close(fd1); // 清理

    // 测试用例 6: 边界测试：复制标准输入（fd = 0）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 stdin", (fd2 = dup3(0, 104, 0)) >= 0, 0);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 7: 边界测试：复制标准输出（fd = 1）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 stdout", (fd2 = dup3(1, 105, 0)) >= 0, 0);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 8: 边界测试：复制标准错误（fd = 2）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 stderr", (fd2 = dup3(2, 106, 0)) >= 0, 0);
    if (fd2 >= 0)
        close(fd2); // 清理

    // 测试用例 9: 测试 oldfd 与 newfd 相同（应失败）
    fd1 = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("dup3 same fd", (fd2 = dup3(fd1, fd1, 0)) < 0, EINVAL);
    close(fd1); // 清理

    remove("testfile.txt");
}

// 测试函数
void test_lseek()
{
    int fd;
    off_t result;

    // 测试用例 1: 正常情况下从文件开头向后移动偏移量
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    result = lseek(fd, 10, SEEK_SET);
    TEST_RESULT_ERRNO("lseek valid fd SEEK_SET", result == 10, 0);
    close(fd);

    // 测试用例 2: 正常情况下从当前位置向后移动偏移量
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0;              // 重置 errno
    lseek(fd, 5, SEEK_SET); // 移动到5
    result = lseek(fd, 5, SEEK_CUR);
    TEST_RESULT_ERRNO("lseek valid fd SEEK_CUR", result == 10, 0);
    close(fd);

    // 测试用例 3: 正常情况下从文件末尾向前移动偏移量
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    result = lseek(fd, 0, SEEK_END);
    TEST_RESULT_ERRNO("lseek valid fd SEEK_END", result == 0, 0);
    close(fd);

    // 测试用例 4: 测试无效的文件描述符（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("lseek invalid fd", lseek(-1, 0, SEEK_SET) == (off_t)-1, EBADF);

    // 测试用例 5: 测试无效的 whence 值（应失败）
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("lseek invalid whence", lseek(fd, 0, 999) == (off_t)-1, EINVAL);
    close(fd);

    // 测试用例 6: 测试负偏移量（应失败）
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("lseek negative offset", lseek(fd, -5, SEEK_SET) == (off_t)-1, EINVAL);
    close(fd);

    // 测试用例 7: 测试超出文件末尾（应返回负偏移量）
    fd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    lseek(fd, 100, SEEK_SET); // 移动到100
    errno = 0;                // 重置 errno
    result = lseek(fd, 0, SEEK_CUR);
    TEST_RESULT_ERRNO("lseek out of bounds", result == 100, 0);
    close(fd);

    remove("testfile.txt");
}

// 测试函数
void test_sendfile()
{
    int fd_source, fd_dest;
    off_t offset = 0;
    ssize_t bytes_sent;

    // 创建一个测试文件
    fd_source = open("source.txt", O_CREAT | O_RDWR | O_TRUNC, 0644);
    write(fd_source, "Hello, World!", 13); // 写入一些数据
    fd_dest = open("dest.txt", O_CREAT | O_RDWR | O_TRUNC, 0644);

    // 测试用例 1: 正常情况下从文件发送数据到套接字
    errno = 0; // 重置 errno
    bytes_sent = sendfile(fd_dest, fd_source, &offset, 13);
    TEST_RESULT_ERRNO("sendfile valid fd", bytes_sent == 13, 0);
    close(fd_source);
    close(fd_dest);

    // 重新打开文件以测试
    fd_source = open("source.txt", O_RDONLY);
    fd_dest = open("dest.txt", O_RDWR);

    // 测试用例 2: 测试无效的源文件描述符（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("sendfile invalid source fd", sendfile(fd_dest, -1, &offset, 13) == -1,
                      EBADF);

    // 测试用例 3: 测试无效的目标文件描述符（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("sendfile invalid dest fd", sendfile(-1, fd_source, &offset, 13) == -1,
                      EBADF);

    // 测试用例 4: 测试超出文件大小（应成功）
    fd_source = open("source.txt", O_RDONLY);
    offset = 100; // 超出文件大小
    errno = 0;    // 重置 errno
    TEST_RESULT_ERRNO("sendfile out of bounds", sendfile(fd_dest, fd_source, &offset, 13) == 0, 0);

    // 清理
    close(fd_source);
    close(fd_dest);
    remove("source.txt");
    remove("dest.txt");
}

// 测试函数
void test_linkat()
{
    int olddirfd, newdirfd;
    const char *oldpath = "source.txt";
    const char *newpath = "newlink.txt";
    const char *newpath2 = "newlink2.txt";

    // 创建一个测试文件
    olddirfd = open(".", O_RDONLY);
    int fd = open(oldpath, O_CREAT | O_RDWR, 0644);
    write(fd, "Hello, World!", 13);
    close(fd);

    // 测试用例 1: 正常情况下创建一个硬链接
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("linkat valid paths", linkat(olddirfd, oldpath, AT_FDCWD, newpath, 0) == 0,
                      0);

    // 测试用例 2: 测试新路径已存在（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("linkat new path exists",
                      linkat(olddirfd, oldpath, AT_FDCWD, newpath, 0) == -1, EEXIST);

    // 测试用例 3: 测试无效的旧路径（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("linkat invalid old path",
                      linkat(olddirfd, "invalid.txt", AT_FDCWD, newpath, 0) == -1, ENOENT);

    // 测试用例 4: 测试无效的新路径（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("linkat invalid new path", linkat(olddirfd, oldpath, AT_FDCWD, NULL, 0) == -1,
                      EFAULT);

    // 测试用例 5: 测试相对路径和绝对路径链接（应成功）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("linkat relative and absolute paths",
                      linkat(olddirfd, oldpath, AT_FDCWD, newpath2, 0) == 0, 0);

    // FIX: NOT SUPPORT YET
    // 测试用例 6: 测试在不同文件系统（应失败）
    // 创建一个新的目录来测试 EXDEV 错误
    // int dir_fd = open("testdir", O_CREAT | O_RDONLY, 0755);
    // newdirfd = dir_fd; // 新目录文件描述符
    // errno = 0;         // 重置 errno
    // TEST_RESULT_ERRNO("linkat across filesystems",
    //                  linkat(olddirfd, oldpath, newdirfd, newpath, 0) == -1, EXDEV);

    // 清理
    close(newdirfd);
    remove(oldpath);
    remove(newpath);
    remove(newpath2);
    remove("testdir");
}

// 测试函数
void test_mkdirat()
{
    int dirfd;
    const char *dirname = "testdir";
    const char *existing_dir = "existing_dir";

    // 创建一个有效的目录用于测试
    dirfd = open(".", O_RDONLY);
    mkdirat(dirfd, existing_dir, 0755); // 创建一个已存在目录

    // 正向测试用例 1: 正常情况下创建目录
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("mkdirat valid dir", mkdirat(dirfd, dirname, 0755) == 0, 0);

    // 正向测试用例 2: 再次创建同一目录（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("mkdirat existing dir", mkdirat(dirfd, existing_dir, 0755) < 0, EEXIST);

    // 正向测试用例 3: 创建目录，使用合法模式
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("mkdirat valid dir with mode", mkdirat(dirfd, "new_dir", 0755) == 0, 0);

    // 异常测试用例 1: 无效的 dirfd（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("mkdirat invalid dirfd", mkdirat(-1, dirname, 0755) < 0, EBADF);

    // 异常测试用例 2: 无效的路径（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("mkdirat invalid path", mkdirat(dirfd, "invalid/dir", 0755) < 0, ENOENT);

    // 异常测试用例 3: dirfd 不是一个目录（应失败）
    int filefd = open("testfile.txt", O_CREAT | O_RDWR, 0644);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("mkdirat non-dir fd", mkdirat(filefd, dirname, 0755) < 0, ENOTDIR);
    close(filefd);

    // 边界测试用例 1: 测试创建带有长路径的目录
    errno = 0; // 重置 errno
    char long_dir[257];
    memset(long_dir, 'a', sizeof(long_dir) - 1);
    long_dir[sizeof(long_dir) - 1] = '\0';
    TEST_RESULT_ERRNO("mkdirat long path", mkdirat(dirfd, long_dir, 0755) < 0, ENAMETOOLONG);

    // FIX: NOT SUPPORT YET
    //    // 边界测试用例 2: 测试创建目录时没有权限（在没有写权限的目录中）
    //    // 需要确保 dirfd 指向一个没有写权限的目录
    //    // 这里假设 dirfd 是一个无写权限的目录
    //    TEST_RESULT_ERRNO("mkdirat no permission", mkdirat(dirfd, "no_permission_dir", 0755) < 0,
    //                      EACCES);

    // 清理
    unlinkat(dirfd, existing_dir, AT_REMOVEDIR); // 删除已存在的目录
    unlinkat(dirfd, dirname, AT_REMOVEDIR);      // 删除新创建的目录
    unlinkat(dirfd, "new_dir", AT_REMOVEDIR);    // 删除另外创建的目录
    close(dirfd);
}

// 测试函数
void test_unlinkat()
{
    int dirfd, fd;
    const char *filename = "testfile3.txt";
    const char *dirname = "testdir";

    // 创建测试文件和目录
    dirfd = open(".", O_RDONLY);
    fd = open(filename, O_CREAT | O_RDWR, 0644);
    close(fd);
    mkdirat(dirfd, dirname, 0755); // 使用 mkdirat 创建目录

    // 正向测试用例 1: 正常情况下删除文件
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat valid file", unlinkat(dirfd, filename, 0) == 0, 0);

    // 正向测试用例 2: 再次删除文件（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat deleted file", unlinkat(dirfd, filename, 0) < 0, ENOENT);

    // 正向测试用例 3: 删除目录（应失败，AT_REMOVEDIR未指定）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat directory without AT_REMOVEDIR", unlinkat(dirfd, dirname, 0) < 0,
                      EISDIR);

    // 正向测试用例 4: 正常情况下删除空目录
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat valid empty dir", unlinkat(dirfd, dirname, AT_REMOVEDIR) == 0, 0);

    // 异常测试用例 1: 无效的 dirfd（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat invalid dirfd", unlinkat(-1, filename, 0) < 0, EBADF);

    // 异常测试用例 2: 无效的路径（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat invalid path", unlinkat(dirfd, "invalid.txt", 0) < 0, ENOENT);

    // 异常测试用例 3: dirfd 不是一个目录（应失败）
    int filefd = open(filename, O_CREAT | O_RDWR);
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat non-dir fd", unlinkat(filefd, filename, 0) < 0, ENOTDIR);
    close(filefd);

    // 边界测试用例 1: 测试删除当前工作目录（应失败）
    errno = 0; // 重置 errno
    TEST_RESULT_ERRNO("unlinkat current dir", unlinkat(dirfd, ".", 0) < 0, EPERM);

    // 边界测试用例 2: 测试删除根目录（通常应失败）
    // 这里假设你的测试环境允许，否则可以注释掉
    // TEST_RESULT("unlinkat root dir", unlinkat(dirfd, "/", 0) < 0, EPERM);

    // 清理
    unlinkat(dirfd, filename, 0);           // 删除文件
    unlinkat(dirfd, dirname, AT_REMOVEDIR); // 删除目录
    close(dirfd);
}

// 测试函数
void test_openat()
{
    int fd;

    // 测试用例 1: 正常情况下打开一个有效的文件
    errno = 0; // 重置 errno
    fd = openat(AT_FDCWD, "testfile.txt", O_CREAT | O_RDWR, 0644);
    TEST_RESULT_ERRNO("openat valid file", fd >= 0, 0);
    if (fd >= 0)
        close(fd); // 清理

    // 测试用例 2: 测试重复打开同一文件
    errno = 0; // 重置 errno
    fd = openat(AT_FDCWD, "testfile.txt", O_RDWR, 0);
    TEST_RESULT_ERRNO("openat valid file twice", fd >= 0, 0);
    if (fd >= 0)
        close(fd); // 清理

    // 测试用例 3: 测试使用 O_EXCL 打开已存在的文件（应失败）
    errno = 0; // 重置 errno
    fd = openat(AT_FDCWD, "testfile.txt", O_CREAT | O_EXCL, 0644);
    TEST_RESULT_ERRNO("openat O_EXCL on existing file", fd < 0, EEXIST);

    // 测试用例 4: 测试打开不存在的文件（应失败）
    errno = 0; // 重置 errno
    fd = openat(AT_FDCWD, "nonexistent.txt", O_RDONLY, 0);
    TEST_RESULT_ERRNO("openat nonexistent file", fd < 0, ENOENT);

    // 测试用例 5: 测试使用无效的 dirfd（应失败）
    errno = 0; // 重置 errno
    fd = openat(-1, "testfile.txt", O_RDONLY, 0);
    TEST_RESULT_ERRNO("openat invalid dirfd", fd < 0, EBADF);

    // 测试用例 6: 测试打开目录（应成功）
    errno = 0; // 重置 errno
    fd = openat(AT_FDCWD, ".", O_RDONLY, 0);
    TEST_RESULT_ERRNO("openat current directory", fd >= 0, 0);
    if (fd >= 0)
        close(fd); // 清理

    // 测试用例 7: 测试打开一个目录的有效路径（应成功）
    errno = 0; // 重置 errno
    fd = openat(AT_FDCWD, "testfile.txt", O_RDONLY, 0);
    TEST_RESULT_ERRNO("openat valid dir", fd >= 0, 0);
    if (fd >= 0)
        close(fd); // 清理

    // 测试用例 8: 测试打开文件时的权限问题（需要设置文件权限）
    errno = 0;                                                             // 重置 errno
    fd = openat(AT_FDCWD, "protected_file.txt", O_WRONLY | O_CREAT, 0000); // 创建无权限文件
    TEST_RESULT_ERRNO("openat permission denied", fd < 0, EACCES);
}

int main()
{
    // 执行测试
    printf("\ntest_openat\n");
    test_openat();
    printf("\ntest_linkat\n");
    test_linkat();
    printf("\ntest_mkdirat\n");
    test_mkdirat();
    printf("\ntest_unlinkat\n");
    test_unlinkat();

    printf("\ntest_remove\n");
    test_remove();
    printf("test_rename\n");
    test_rename();
    printf("test_read_write\n");
    test_read_write();
    printf("test_readv_writev\n");
    test_readv_writev();
    printf("test_pread_pwrite\n");
    test_pread_pwrite();
    printf("test_dup\n");
    test_dup();
    printf("test_dup3\n");
    test_dup3();
    printf("test_lseek\n");
    test_lseek();
    printf("test_sendfile\n");
    test_sendfile();
    return 0;
}
