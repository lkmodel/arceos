#include <stdbool.h>
#include <stdio.h>
#include <string.h>

// 宏定义以简化测试输出
#define TEST_FUNCTION(func, ...)                          \
    do {                                                  \
        if (func(__VA_ARGS__)) {                          \
            printf("\033[0;32m%s: PASS\033[0m\n", #func); \
        } else {                                          \
            printf("\033[0;31m%s: BAD\033[0m\n", #func);  \
        }                                                 \
    } while (0)

// 测试函数
bool test_memcpy()
{
    char dest[50];
    char src[] = "Hello, World!";
    memcpy(dest, src, strlen(src) + 1);
    return strcmp(dest, src) == 0;
}

bool test_memmove()
{
    char str[] = "Hello, World!";
    memmove(str + 7, str, 5); // Move "Hello" to "World!"
    return strcmp(str, "Hello, Hello!") == 0;
}

bool test_memset()
{
    char buffer[10];
    memset(buffer, 'A', 5);
    buffer[5] = '\0'; // null-terminate
    return strcmp(buffer, "AAAAA") == 0;
}

bool test_memcmp()
{
    return memcmp("abc", "abc", 3) == 0 && memcmp("abc", "abd", 3) < 0 &&
           memcmp("abc", "abb", 3) > 0;
}

bool test_strcpy()
{
    char dest[50];
    strcpy(dest, "Hello, World!");
    return strcmp(dest, "Hello, World!") == 0;
}

bool test_strncpy()
{
    char dest[50];
    strncpy(dest, "Hello, World!", 5);
    dest[5] = '\0'; // null-terminate
    return strcmp(dest, "Hello") == 0;
}

bool test_strcat()
{
    char dest[50] = "Hello";
    strcat(dest, ", World!");
    return strcmp(dest, "Hello, World!") == 0;
}

bool test_strncat()
{
    char dest[50] = "Hello";
    strncat(dest, ", World!", 7);
    return strcmp(dest, "Hello, World") == 0;
}

bool test_strcmp()
{
    return strcmp("abc", "abc") == 0 && strcmp("abc", "abd") < 0 && strcmp("abc", "abb") > 0;
}

bool test_strncmp()
{
    return strncmp("abc", "abc", 3) == 0 && strncmp("abc", "abd", 3) < 0 &&
           strncmp("abc", "abb", 3) > 0;
}

bool test_strchr()
{
    return strchr("Hello, World!", 'W') == "Hello, World!" + 7;
}

bool test_strrchr()
{
    return strrchr("Hello, World!", 'o') == "Hello, World!" + 8;
}

bool test_strcspn()
{
    return strcspn("Hello, World!", "oW") == 4;
}

bool test_strspn()
{
    return strspn("Hello, World!", "Hello") == 5;
}

bool test_strpbrk()
{
    return strpbrk("Hello, World!", "oW") == "Hello, World!" + 4;
}

bool test_strstr()
{
    return strstr("Hello, World!", "World") == "Hello, World!" + 7;
}

bool test_strtok()
{
    char str[] = "Hello, World!";
    char *token = strtok(str, ", ");
    return strcmp(token, "Hello") == 0;
}

bool test_strlen()
{
    return strlen("Hello, World!") == 13;
}

int main()
{
    // 运行测试
    TEST_FUNCTION(test_memcpy);
    TEST_FUNCTION(test_memmove);
    TEST_FUNCTION(test_memset);
    TEST_FUNCTION(test_memcmp);
    TEST_FUNCTION(test_strcpy);
    TEST_FUNCTION(test_strncpy);
    TEST_FUNCTION(test_strcat);
    TEST_FUNCTION(test_strncat);
    TEST_FUNCTION(test_strcmp);
    TEST_FUNCTION(test_strncmp);
    TEST_FUNCTION(test_strchr);
    TEST_FUNCTION(test_strrchr);
    TEST_FUNCTION(test_strcspn);
    TEST_FUNCTION(test_strspn);
    TEST_FUNCTION(test_strpbrk);
    TEST_FUNCTION(test_strstr);
    TEST_FUNCTION(test_strtok);
    TEST_FUNCTION(test_strlen);

    return 0;
}
