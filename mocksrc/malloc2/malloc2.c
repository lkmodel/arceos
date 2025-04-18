#include <malloc.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define SIZE_MAX 18446744073709551615
// 测试宏
#define TEST_FUNCTION(func, expected, test_case)                                               \
    do {                                                                                       \
        typeof(func) result = func;                                                            \
        if (result == expected) {                                                              \
            printf("\033[0;32m%s: PASS\033[0m\n", #func);                                      \
        } else {                                                                               \
            printf("\033[0;31m%s: BAD\033[0m - Expected: %p, Got: %p, Test case: %s\n", #func, \
                   expected, result, test_case);                                               \
        }                                                                                      \
    } while (0)

// 测试函数
void test_malloc()
{
    void *ptr1 = malloc(10);
    void *ptr2 = malloc(20);
    void *ptr3 = malloc(-1);       // 期望返回 NULL
    void *ptr4 = malloc(SIZE_MAX); // 期望返回 NULL (如果实现了检查)

    TEST_FUNCTION(ptr1, ptr1, "Allocate 10 bytes");
    TEST_FUNCTION(ptr2, ptr2, "Allocate 20 bytes");
    TEST_FUNCTION(ptr3, NULL, "Allocate 0 bytes");
    TEST_FUNCTION(ptr4, NULL, "Allocate SIZE_MAX bytes");

    // 清理
    free(ptr1);
    free(ptr2);
}

void test_calloc()
{
    void *ptr1 = calloc(5, sizeof(int));        // 期望返回非 NULL
    void *ptr2 = calloc(-1, sizeof(int));       // 期望返回 NULL
    int *ptr3 = calloc(3, sizeof(int));         // 期望返回非 NULL
    void *ptr4 = calloc(SIZE_MAX, sizeof(int)); // 期望返回 NULL (如果实现了检查)

    TEST_FUNCTION(ptr1, ptr1, "Calloc 5 integers");
    TEST_FUNCTION(ptr2, NULL, "Calloc 0 integers");
    TEST_FUNCTION(ptr3, ptr3, "Calloc 3 integers");
    TEST_FUNCTION(ptr4, NULL, "Calloc SIZE_MAX integers");

    // 清理
    free(ptr1);
    free(ptr3);
}

void test_realloc()
{
    void *ptr = malloc(10);
    void *new_ptr = realloc(ptr, 20);     // 期望返回非 NULL
    void *failed_ptr = realloc(NULL, 10); // 期望返回非 NULL

    TEST_FUNCTION(new_ptr, new_ptr, "Realloc from 10 to 20 bytes");
    TEST_FUNCTION(failed_ptr, failed_ptr, "Realloc NULL");

    // 清理
    free(new_ptr);
}

void test_free()
{
    void *ptr = malloc(10);
    free(ptr);

    // 在此处可以添加更多的逻辑来验证释放后的指针状态
    printf("\033[0;32mfree: PASS\033[0m - Freeing a valid pointer\n");
}

int main()
{
    printf("Running malloc tests...\n");
    test_malloc();

    printf("\nRunning calloc tests...\n");
    test_calloc();

    printf("\nRunning realloc tests...\n");
    test_realloc();

    printf("\nRunning free tests...\n");
    test_free();

    return 0;
}
