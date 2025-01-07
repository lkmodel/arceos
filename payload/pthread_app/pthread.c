#include <stdio.h>
#include <pthread.h>
#include <unistd.h>

#define NUM_THREADS 2

// 线程函数
void* thread_function(void *arg);
int addtwo(int a);

int main() {
    pthread_t threads[NUM_THREADS];
    int thread_ids[NUM_THREADS];

    printf("start threads test %d !\n", NUM_THREADS);

    // 打印内部函数的地址
    printf("Address of addtwo: %p\n", (void*)addtwo);
    printf("Address of thread_function: %p\n", (void*)thread_function);
    printf("Address of main: %p\n", (void*)main);

    int a = addtwo(1);
    printf("addtwo %d\n", a);
    
    // 创建多个线程
    for(int i = 0; i < NUM_THREADS; i++) {
        thread_ids[i] = i;
        int ret = pthread_create(&threads[i], NULL, thread_function, &thread_ids[i]);

        printf("ret %d\n", ret);

        if(ret != 0) {
            printf("Error: pthread_create failed\n");
            return 1;
        }
        printf("Created thread %d\n", i);
    }

    printf("checkpoint 1\n");

    // 等待所有线程完成
    for(int i = 0; i < NUM_THREADS; i++) {
        pthread_join(threads[i], NULL);
    }

    printf("All threads completed\n");
    return 0;
}

int addtwo(int a) {
    return a + 2;
}

// 线程函数
void* thread_function(void *arg) {
    int thread_id = *((int*)arg);

    // 在线程中也打印函数地址
    printf("In thread %d - Address of thread_function: %p\n", thread_id, (void*)thread_function);

    for(int i = 0; i < 2; i++) {
        printf("Thread %d is running, count: %d\n", thread_id, i);
        sleep(1);  // 休眠1秒
    }

    printf("Thread %d finished\n", thread_id);
    pthread_exit(NULL);
}