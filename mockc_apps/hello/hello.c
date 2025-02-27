#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <sys/time.h>
#include <time.h>

int main()
{
    puts("[test puts] Hello world\n");
    int a = 1;
    int b = 2;
    int c = a + b;
    for (int i = 0; i < 3; i++) {
        clock_t time = clock();
        printf("current time:%ld\n", time);
        c += i;
    }
    printf("[Test printf] %s\n", 0 == printf("HEX: %0x\n", 0x55) ? "BAD!" : "PASS!");

    puts("[Test sprintf]\n");
    putchar('f');
    char str1[20];
    int i;
    sprintf(str1, "%p\n", &i);
    puts(str1);

    puts("[Test ctype.h]\n");
    printf("isalnum %s\n", isalnum('1') ? "PASS!" : "BAD!");
    printf("isalpha %s\n", isalpha('a') ? "PASS!" : "BAD!");
    printf("isblank %s\n", isblank(' ') ? "PASS!" : "BAD!");
    printf("fff %f\n", 3.14f);

    puts("[Test time.h]\n");
    struct timeval tv;
    struct timezone tz;

    // 获取当前时间
    if (gettimeofday(&tv, &tz) == 0) {
        printf("Seconds since Epoch: %ld\n", tv.tv_sec);
        printf("Microseconds: %ld\n", tv.tv_usec);
    } else {
        perror("gettimeofday failed");
        return 1;
    }

    char str[30];
    scanf("%s", str);
    printf("get input: %s\n", str);

    return 0;
}
