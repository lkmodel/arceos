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

    TEST_ASSERT_TRUE(0);
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
int main(void) {
    UNITY_BEGIN();
    RUN_TEST(test_basic);
    RUN_TEST(test_advance);
    return UNITY_END();
}