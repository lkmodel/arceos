#include <stdio.h>
#include <pthread.h>
#include <errno.h>

/* pthread_mutex_t mutex = PTHREAD_MUTEX_INITIALIZER; */
pthread_mutex_t mutex;

int count;

void * thread_run(void *arg)
{
    int i;
    pthread_mutex_lock(&mutex);
    for (i = 0; i < 3; i++) {
        ++count;
        printf("value of count: %d\n", count);
    }
    pthread_mutex_unlock(&mutex);
    return 0;
}

void test_mutex(pthread_mutex_t *mutex, const char *stage) {
    printf("\n=== %s ===\n", stage);
    
    // 尝试加锁
    int lock_result = pthread_mutex_lock(mutex);
    printf("Lock result: %d (EINVAL=%d)\n", lock_result, EINVAL);
    
    if (lock_result == 0) {
        printf("Successfully locked!\n");
        // 尝试解锁
        int unlock_result = pthread_mutex_unlock(mutex);
        printf("Unlock result: %d\n", unlock_result);
    }
}

int main(int argc, char *argv[])
{
    pthread_t thread1, thread2;
    pthread_mutex_init(&mutex, 0);
    pthread_create(&thread1, NULL, thread_run, 0);
    pthread_create(&thread2, NULL, thread_run, 0);
    pthread_join(thread1, 0);
    pthread_join(thread2, 0);
    pthread_mutex_destroy(&mutex);
    
    return 0;
}