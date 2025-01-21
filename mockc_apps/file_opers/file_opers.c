#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>

int main()
{
    int fd;
    struct stat statbuf;
    char cwd[1024];
    const char *filename = "testfile.txt";
    const char *new_filename = "renamed_file.txt";

    // 1. 使用 sys_open 打开文件
    fd = open(filename, O_CREAT | O_RDWR, S_IRUSR | S_IWUSR);
    if (fd < 0) {
        printf("Error: sys_open failed\n");
        return 1;
    }
    printf("File opened successfully: %s\n", filename);

    // 2. 使用 sys_lseek 移动文件指针
    if (lseek(fd, 0, SEEK_SET) == (off_t)-1) {
        printf("Error: sys_lseek failed\n");
        close(fd);
        return 1;
    }
    printf("File pointer moved successfully.\n");

    // 3. 使用 sys_fstat 获取文件状态
    if (fstat(fd, &statbuf) == -1) {
        printf("Error: sys_fstat failed\n");
        close(fd);
        return 1;
    }
    printf("File size: %lld bytes\n", (long long)statbuf.st_size);

    // 4. 使用 sys_getcwd 获取当前工作目录
    if (getcwd(cwd, sizeof(cwd)) != NULL) {
        printf("Current working directory: %s\n", cwd);
    } else {
        printf("Error: sys_getcwd failed\n");
        close(fd);
        return 1;
    }

    // 5. 使用 sys_rename 重命名文件
    if (rename(filename, new_filename) == -1) {
        printf("Error: sys_rename failed\n");
        close(fd);
        return 1;
    }
    printf("File renamed successfully to: %s\n", new_filename);

    // 6. 使用 sys_stat 获取文件状态
    if (stat(new_filename, &statbuf) == -1) {
        printf("Error: sys_stat failed\n");
        close(fd);
        return 1;
    }
    printf("Renamed file size: %lld bytes\n", (long long)statbuf.st_size);

    // 7. 使用 sys_lstat 获取文件状态（用于符号链接）
    if (lstat(new_filename, &statbuf) == -1) {
        printf("Error: sys_lstat failed\n");
        close(fd);
        return 1;
    }
    printf("lstat on renamed file successful.\n");

    // 关闭文件描述符
    close(fd);
    return 0;
}
