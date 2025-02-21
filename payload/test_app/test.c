#include "unity.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stddef.h>

typedef struct Test {
    int id;
    float num;
} Test;

void setUp(void) {
    // set stuff up here
}

void tearDown(void) {
    // clean stuff up here
}

void test_basic(void){
    TEST_MESSAGE("Begin test");

    TEST_ASSERT_TRUE(1);
    TEST_ASSERT_FALSE(0);
    TEST_ASSERT(1);

    TEST_ASSERT_EQUAL_INT(2,1+1);

    TEST_ASSERT_EQUAL_FLOAT(1.1, 1.1);

    int arr[]={0,10,20,30,40};
    int arr2[]={0,10,20,30,40};
    TEST_ASSERT_EQUAL_INT_ARRAY(arr,arr2,5);

    TEST_ASSERT_EQUAL_STRING("Jude","Jude");

    TEST_ASSERT_NULL(NULL);
    TEST_ASSERT_NOT_NULL(1);
}
void test_advance(void){
    Test* test = (Test*)malloc(sizeof(Test));
    TEST_ASSERT_NOT_NULL_MESSAGE(test,"Test memory allocation failed\n");

    test->id = 1;
    test->num = 1.1;

    Test* test2 = (Test*)malloc(sizeof(Test));
    TEST_ASSERT_NOT_NULL_MESSAGE(test2,"Test memory allocation failed\n");

    test2->id = 1;
    test2->num = 1.1;

    TEST_ASSERT_EQUAL_MEMORY(test,test2,sizeof(Test));
}

// 测试基础内存分配
void test_simple_allocation(void) {
    int* ptr = malloc(sizeof(int));
    TEST_ASSERT_NOT_NULL(ptr);  // 验证分配成功

    *ptr = 0xDEADBEEF;
    TEST_ASSERT_EQUAL_HEX32(0xDEADBEEF, *ptr);  // 验证内存可写

    free(ptr);
}

// 测试数组内存操作
void test_array_allocation(void) {
    const int size = 5;
    int* arr = malloc(size * sizeof(int));
    TEST_ASSERT_NOT_NULL(arr);

    // 初始化并验证数组内容
    for(int i=0; i<size; i++) {
        arr[i] = i*10;
        TEST_ASSERT_EQUAL(i*10, arr[i]);
    }

    free(arr);
}

// 测试结构体内存分配
typedef struct {
    int id;
    char name[20];
    float score;
} Student;

void test_struct_allocation(void) {
    Student* s = malloc(sizeof(Student));
    TEST_ASSERT_NOT_NULL(s);

    s->id = 1001;
    strcpy(s->name, "Alice");
    s->score = 95.5f;

    TEST_ASSERT_EQUAL(1001, s->id);
    TEST_ASSERT_EQUAL_STRING("Alice", s->name);
    TEST_ASSERT_EQUAL_FLOAT(95.5f, s->score);

    free(s);
}

//// 测试内存边界写入
//void test_memory_boundary(void) {
//    char* buf = malloc(10);
//    TEST_ASSERT_NOT_NULL(buf);
//
//    memset(buf, 'A', 10);  // 正确填充
//    TEST_ASSERT_EACH_EQUAL_CHAR('A', buf, 10);
//
//    free(buf);
//}

// 测试realloc功能
void test_realloc_behavior(void) {
    int* arr = malloc(3*sizeof(int));
    TEST_ASSERT_NOT_NULL(arr);

    arr[0] = 1;
    arr = realloc(arr, 5*sizeof(int));  // 扩展内存
    TEST_ASSERT_NOT_NULL(arr);

    TEST_ASSERT_EQUAL(1, arr[0]);  // 验证原数据保留
    arr[3] = 4;                    // 测试新内存区域
    TEST_ASSERT_EQUAL(4, arr[3]);

    free(arr);
}

// 测试calloc初始化清零
void test_calloc_initialization(void) {
    int* arr = calloc(5, sizeof(int));
    TEST_ASSERT_NOT_NULL(arr);

    for(int i=0; i<5; i++) {
        TEST_ASSERT_EQUAL(0, arr[i]);  // 验证内存已清零
    }

    free(arr);
}


int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_basic);
    RUN_TEST(test_advance);

    RUN_TEST(test_simple_allocation);
    RUN_TEST(test_array_allocation);
    RUN_TEST(test_struct_allocation);
//    RUN_TEST(test_memory_boundary);
    RUN_TEST(test_realloc_behavior);
    RUN_TEST(test_calloc_initialization);
    return UNITY_END();
}