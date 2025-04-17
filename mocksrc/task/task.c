#include <errno.h>
#include <linux/futex.h>
#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/syscall.h>
#include <unistd.h>

// `futex` syscall wrapper
static inline int futex_wait(volatile int *uaddr, int expected)
{
    return syscall(SYS_futex, uaddr, FUTEX_WAIT, expected, NULL, NULL, 0);
}

static inline int futex_wake(volatile int *uaddr)
{
    return syscall(SYS_futex, uaddr, FUTEX_WAKE, 1, NULL, NULL, 0);
}

// Shared variable
volatile int waiter = 0;

void *thread_func(void *arg)
{
    printf("Thread 1: Waiting...\n");
    futex_wait(&waiter, 0); // Wait until `waiter` is not 0
    printf("Thread 1: Woken up!\n");
    return NULL;
}

int main()
{
    pthread_t thread;

    // Create a new thread
    pthread_create(&thread, NULL, thread_func, NULL);

    // Sleep to ensure the thread is waiting
    sleep(2);

    printf("Main thread: Waking up Thread 1...\n");
    waiter = 1;          // Change the state to wake the waiting thread
    futex_wake(&waiter); // Wake up the waiting thread

    // Wait for the thread to finish
    pthread_join(thread, NULL);
    return 0;
}
