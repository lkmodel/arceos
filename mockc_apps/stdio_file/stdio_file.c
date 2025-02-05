#include <fcntl.h>
#include <stdio.h>
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

int main()
{
    // 执行测试
    test_remove();
    test_rename();
    return 0;
}
