#include <ctype.h>
#include <errno.h>
#include <stdio.h>
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
    char str1[20];
    int i;
    sprintf(str1, "%p\n", &i);
    puts(str1);

    puts("[Test ctype.h]\n");
    printf("isalnum %s\n", isalnum('1') ? "PASS!" : "BAD!");
    printf("isalpha %s\n", isalpha('a') ? "PASS!" : "BAD!");
    printf("isblank %s\n", isblank(' ') ? "PASS!" : "BAD!");
    printf("fff %f\n", 3.14f);

    return 0;
}
