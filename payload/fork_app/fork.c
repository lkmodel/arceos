#include <stdio.h>
#include <unistd.h>
#include <sys/types.h>

int main() {
    unsigned long sp, s0, ra;
    
    // 在程序开始时获取sp和s0的值
    asm volatile("mv %0, sp" : "=r"(sp));
    asm volatile("mv %0, s0" : "=r"(s0));
    asm volatile("mv %0, ra" : "=r"(ra));

    printf("Start - SP: 0x%lx, S0: 0x%lx, RA: 0x%lx\n", sp, s0, ra);

    pid_t pid;

    printf("Before fork %s\n", "fork_app");

    pid = fork();

    printf("PID: %d\n", pid);

    if (pid < 0) {
        printf("Fork failed %d \n", pid);
    }
    else if (pid == 0) {
        printf("I am the child process, my process id is %d\n", pid);
    }
    else {
        printf("I am the parent process, my process id is %d\n", pid);
    }

    printf("After fork %s\n", "fork_app");

    // 在程序结束时获取sp和s0的值
    asm volatile("mv %0, sp" : "=r"(sp));
    asm volatile("mv %0, s0" : "=r"(s0));
    asm volatile("mv %0, ra" : "=r"(ra));
    printf("Start - SP: 0x%lx, S0: 0x%lx, RA: 0x%lx\n", sp, s0, ra);

    return 0;
}