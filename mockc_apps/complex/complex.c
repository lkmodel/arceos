#include <complex.h>
#include <math.h> // 假设使用自己的 mocklibc
#include <stdbool.h>
#include <stdio.h>

#define EPSILON 1e-6
#define GREEN   "\033[0;32m"
#define RED     "\033[0;31m"
#define RESET   "\033[0m"

// 比较复数的辅助函数
bool compare_complex(double complex a, double complex b)
{
    return fabs(creal(a) - creal(b)) < EPSILON && fabs(cimag(a) - cimag(b)) < EPSILON;
}

// 测试宏定义
#define TEST_FUNC(func, input, expected)                                            \
    do {                                                                            \
        double complex result = func(input);                                        \
        if (compare_complex(result, expected)) {                                    \
            printf(GREEN "%s: PASS\n" RESET, #func);                                \
        } else {                                                                    \
            printf(RED "%s: BAD - expected (%f, %f), got (%f, %f)\n" RESET, #func,  \
                   creal(expected), cimag(expected), creal(result), cimag(result)); \
        }                                                                           \
    } while (0)

#define TEST_FUNC_F(func, input, expected)                                          \
    do {                                                                            \
        float complex result = func(input);                                         \
        if (compare_complex(result, expected)) {                                    \
            printf(GREEN "%s: PASS\n" RESET, #func);                                \
        } else {                                                                    \
            printf(RED "%s: BAD - expected (%f, %f), got (%f, %f)\n" RESET, #func,  \
                   creal(expected), cimag(expected), creal(result), cimag(result)); \
        }                                                                           \
    } while (0)

// 测试宏定义
#define TEST_FUNC_2(func, input1, input2, expected)                                 \
    do {                                                                            \
        double complex result = func(input1, input2);                               \
        if (compare_complex(result, expected)) {                                    \
            printf(GREEN "%s: PASS\n" RESET, #func);                                \
        } else {                                                                    \
            printf(RED "%s: BAD - expected (%f, %f), got (%f, %f)\n" RESET, #func,  \
                   creal(expected), cimag(expected), creal(result), cimag(result)); \
        }                                                                           \
    } while (0)

#define TEST_FUNC_F_2(func, input1, input2, expected)                               \
    do {                                                                            \
        float complex result = func(input1, input2);                                \
        if (compare_complex(result, expected)) {                                    \
            printf(GREEN "%s: PASS\n" RESET, #func);                                \
        } else {                                                                    \
            printf(RED "%s: BAD - expected (%f, %f), got (%f, %f)\n" RESET, #func,  \
                   creal(expected), cimag(expected), creal(result), cimag(result)); \
        }                                                                           \
    } while (0)

// 测试函数
void test_cacos()
{
    TEST_FUNC(cacos, 1.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(cacos, 0.0 + 1.0 * I, 1.5707963267948966 - 0.881373587019543 * I);
    TEST_FUNC(cacos, -1.0 + 0.0 * I, M_PI + 0.0 * I);
}

void test_casin()
{
    TEST_FUNC(casin, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(casin, 1.0 + 0.0 * I, M_PI_2 + 0.0 * I);
    TEST_FUNC(casin, -1.0 + 0.0 * I, -M_PI_2 + 0.0 * I);
}

void test_catan()
{
    TEST_FUNC(catan, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(catan, 1.0 + 0.0 * I, M_PI_4 + 0.0 * I);
    TEST_FUNC(catan, -1.0 + 0.0 * I, -M_PI_4 + 0.0 * I);
}

void test_ccos()
{
    TEST_FUNC(ccos, 0.0 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(ccos, M_PI + 0.0 * I, -1.0 + 0.0 * I);
    TEST_FUNC(ccos, M_PI_2 + 0.0 * I, 0.0 + 0.0 * I);
}

void test_csin()
{
    TEST_FUNC(csin, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(csin, M_PI_2 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(csin, M_PI + 0.0 * I, 0.0 + 0.0 * I);
}

void test_ctan()
{
    TEST_FUNC(ctan, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(ctan, M_PI_4 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(ctan, -M_PI_4 + 0.0 * I, -1.0 + 0.0 * I);
}

void test_cacosh()
{
    TEST_FUNC(cacosh, 1.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(cacosh, 0.0 + 1.0 * I, 0.881373587019543 + 1.5707963267948966 * I);
    TEST_FUNC(cacosh, -1.0 + 0.0 * I, 0.0 + M_PI * I);
}

void test_casinh()
{
    TEST_FUNC(casinh, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(casinh, 1.0 + 0.0 * I, log(1.0 + sqrt(2.0)) + 0.0 * I);
    TEST_FUNC(casinh, -1.0 + 0.0 * I, log(-1.0 + sqrt(2.0)) + 0.0 * I);
}

void test_catanh()
{
    TEST_FUNC(catanh, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(catanh, 1.0 + 0.0 * I, INFINITY + 0.0 * I);
    TEST_FUNC(catanh, -1.0 + 0.0 * I, -INFINITY * log(2.0) + 0.0 * I);
}

void test_ccosh()
{
    TEST_FUNC(ccosh, 0.0 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(ccosh, M_PI + 0.0 * I, cosh(M_PI) + 0.0 * I);
    TEST_FUNC(ccosh, M_PI_2 + 0.0 * I, 2.5091784786580567 + 0.0 * I);
}

void test_csinh()
{
    TEST_FUNC(csinh, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(csinh, M_PI + 0.0 * I, 11.548739357257748 + 0.0 * I);
    TEST_FUNC(csinh, M_PI_2 + 0.0 * I, 2.3012989023072947 + 0.0 * I);
}

void test_ctanh()
{
    TEST_FUNC(ctanh, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(ctanh, M_PI_4 + 0.0 * I, 0.6557942026326724 + 0.0 * I);
    TEST_FUNC(ctanh, -M_PI_4 + 0.0 * I, -0.6557942026326724 + 0.0 * I);
}

void test_cexp()
{
    TEST_FUNC(cexp, 0.0 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(cexp, 1.0 + 0.0 * I, exp(1.0) + 0.0 * I);
    TEST_FUNC(cexp, 0.0 + M_PI_2 * I, cos(M_PI_2) + sin(M_PI_2) * I);
}

void test_clog()
{
    TEST_FUNC(clog, 1.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(clog, exp(1.0) + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(clog, 0.0 + 1.0 * I, M_PI_2 * I);
}

void test_cabs()
{
    TEST_FUNC(cabs, 3.0 + 4.0 * I, 5.0 + 0.0 * I);
    TEST_FUNC(cabs, 1.0 + 1.0 * I, sqrt(2.0) + 0.0 * I);
    TEST_FUNC(cabs, -1.0 - 1.0 * I, sqrt(2.0) + 0.0 * I);
}

void test_cpow()
{
    TEST_FUNC_2(cpow, 1.0 + 0.0 * I, 2.0 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC_2(cpow, 1.0 + 0.0 * I, 0.0 + 1.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC_2(cpow, 0.0 + 1.0 * I, 2.0 + 0.0 * I, -1.0 + 0.0 * I);
}

void test_csqrt()
{
    TEST_FUNC(csqrt, 1.0 + 0.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(csqrt, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(csqrt, -1.0 + 0.0 * I, 0.0 + 1.0 * I);
}

void test_carg()
{
    TEST_FUNC(carg, 1.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(carg, 0.0 + 1.0 * I, M_PI_2 + 0.0 * I);
    TEST_FUNC(carg, -1.0 + 0.0 * I, M_PI + 0.0 * I);
}

void test_cimag()
{
    TEST_FUNC(cimag, 1.0 + 2.0 * I, 2.0 + 0.0 * I);
    TEST_FUNC(cimag, 0.0 + 1.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(cimag, -1.0 + 0.0 * I, 0.0 + 0.0 * I);
}

void test_conj()
{
    TEST_FUNC(conj, 1.0 + 2.0 * I, 1.0 - 2.0 * I);
    TEST_FUNC(conj, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(conj, -1.0 + 1.0 * I, -1.0 - 1.0 * I);
}

void test_cproj()
{
    TEST_FUNC(cproj, 1.0 + 2.0 * I, 1.0 + 2.0 * I);
    TEST_FUNC(cproj, 0.0 + 0.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(cproj, -1.0 + 1.0 * I, -1.0 + 1.0 * I);
}

void test_creal()
{
    TEST_FUNC(creal, 1.0 + 2.0 * I, 1.0 + 0.0 * I);
    TEST_FUNC(creal, 0.0 + 1.0 * I, 0.0 + 0.0 * I);
    TEST_FUNC(creal, -1.0 + 0.0 * I, -1.0 + 0.0 * I);
}

int main()
{
    test_cacos();
    test_casin();
    test_catan();
    test_ccos();
    test_csin();
    test_ctan();
    test_cacosh();
    test_casinh();
    test_catanh();
    test_ccosh();
    test_csinh();
    test_ctanh();
    test_cexp();
    test_clog();
    test_cabs();
    //    test_cpow(); // NOTE: Not implement yet.
    test_csqrt();
    test_carg();
    test_cimag();
    test_conj();
    test_cproj();
    test_creal();

    return 0;
}
