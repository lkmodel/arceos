#include <ctype.h>
#include <errno.h>
#include <stdio.h>
#include <time.h>

int test()
{
    printf("WTF\n");
}

int main()
{
    test();
    test();
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
    char str1[20];
    int i;
    sprintf(str1, "%p\n", &i);
    puts(str1);

    // FIXME: Not `musl 1.2.5.`
    //   puts("[Test scanf]\n");
    //  char str2[20];
    //    scanf(str2, "%s");
    // printf("read: %s\n", str2);

    puts("[Test ctype.h]\n");
    printf("isalnum %s\n", isalnum('1') ? "PASS!" : "BAD!");
    printf("isalpha %s\n", isalpha('a') ? "PASS!" : "BAD!");
    printf("isblank %s\n", isblank(' ') ? "PASS!" : "BAD!");
    printf("fff %f\n", 3.14f);

    // FIXME: 需要注意！这里是有BUG的
    //    printf("%d\n", errno);

    return 0;
}
