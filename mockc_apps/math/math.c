#include <math.h> // 假设使用自己的 mocklibc
#include <stdbool.h>
#include <stdio.h>

#define EPSILON 1e-6
#define GREEN   "\033[0;32m"
#define RED     "\033[0;31m"
#define RESET   "\033[0m"

// 比较浮点数的辅助函数
bool compare_double(double a, double b)
{
    return fabs(a - b) < EPSILON;
}

bool compare_float(float a, float b)
{
    return fabsf(a - b) < EPSILON;
}

// 测试宏定义
#define TEST_FUNC(func, input, expected)                                                  \
    do {                                                                                  \
        double result = func(input);                                                      \
        if (compare_double(result, expected)) {                                           \
            printf(GREEN "%s: PASS\n" RESET, #func);                                      \
        } else {                                                                          \
            printf(RED "%s: BAD - expected %f, got %f\n" RESET, #func, expected, result); \
        }                                                                                 \
    } while (0)

#define TEST_FUNC_F(func, input, expected)                                                \
    do {                                                                                  \
        float result = func(input);                                                       \
        if (compare_float(result, expected)) {                                            \
            printf(GREEN "%s: PASS\n" RESET, #func);                                      \
        } else {                                                                          \
            printf(RED "%s: BAD - expected %f, got %f\n" RESET, #func, expected, result); \
        }                                                                                 \
    } while (0)

#define TEST_FUNC_2(func, input1, input2, expected)                                       \
    do {                                                                                  \
        double result = func(input1, input2);                                             \
        if (compare_double(result, expected)) {                                           \
            printf(GREEN "%s: PASS\n" RESET, #func);                                      \
        } else {                                                                          \
            printf(RED "%s: BAD - expected %f, got %f\n" RESET, #func, expected, result); \
        }                                                                                 \
    } while (0)

#define TEST_FUNC_F_2(func, input1, input2, expected)                                     \
    do {                                                                                  \
        float result = func(input1, input2);                                              \
        if (compare_float(result, expected)) {                                            \
            printf(GREEN "%s: PASS\n" RESET, #func);                                      \
        } else {                                                                          \
            printf(RED "%s: BAD - expected %f, got %f\n" RESET, #func, expected, result); \
        }                                                                                 \
    } while (0)

#define TEST_FUNC_3(func, input1, input2, input3, expected)                               \
    do {                                                                                  \
        double result = func(input1, input2, input3);                                     \
        if (compare_double(result, expected)) {                                           \
            printf(GREEN "%s: PASS\n" RESET, #func);                                      \
        } else {                                                                          \
            printf(RED "%s: BAD - expected %f, got %f\n" RESET, #func, expected, result); \
        }                                                                                 \
    } while (0)

#define TEST_FUNC_F_3(func, input1, input2, input3, expected)                             \
    do {                                                                                  \
        float result = func(input1, input2, input3);                                      \
        if (compare_float(result, expected)) {                                            \
            printf(GREEN "%s: PASS" RESET, #func);                                        \
        } else {                                                                          \
            printf(RED "%s: BAD - expected %f, got %f\n" RESET, #func, expected, result); \
        }                                                                                 \
    } while (0)

// 测试函数
void test_acos()
{
    TEST_FUNC(acos, 1.0, 0.0);
    TEST_FUNC(acos, 0.0, M_PI / 2);
    TEST_FUNC(acos, -1.0, M_PI);
}

void test_acosh()
{
    TEST_FUNC(acosh, 1.0, 0.0);
    TEST_FUNC(acosh, 2.0, log(2.0 + sqrt(3.0)));
    TEST_FUNC(acosh, 3.0, log(3.0 + sqrt(8.0)));
}

void test_asin()
{
    TEST_FUNC(asin, 0.0, 0.0);
    TEST_FUNC(asin, 1.0, M_PI / 2);
    TEST_FUNC(asin, -1.0, -M_PI / 2);
}

void test_asinh()
{
    TEST_FUNC(asinh, 0.0, 0.0);
    TEST_FUNC(asinh, 1.0, 0.881373587019543);
    TEST_FUNC(asinh, -1.0, -0.881373587019543);
}

void test_atan()
{
    TEST_FUNC(atan, 0.0, 0.0);
    TEST_FUNC(atan, 1.0, M_PI / 4);
    TEST_FUNC(atan, -1.0, -M_PI / 4);
}

void test_atan2()
{
    TEST_FUNC_2(atan2, 1.0, 1.0, M_PI / 4);
    TEST_FUNC_2(atan2, -1.0, 1.0, -M_PI / 4);
    TEST_FUNC_2(atan2, 1.0, -1.0, 2.356194490192345);
}

void test_atanh()
{
    TEST_FUNC(atanh, 0.0, 0.0);
    TEST_FUNC(atanh, 0.5, log(3.0) / 2.0);
    TEST_FUNC(atanh, -0.5, -log(3.0) / 2.0);
}

void test_cbrt()
{
    TEST_FUNC(cbrt, 8.0, 2.0);
    TEST_FUNC(cbrt, -8.0, -2.0);
    TEST_FUNC(cbrt, 27.0, 3.0);
}

void test_ceil()
{
    TEST_FUNC(ceil, 2.3, 3.0);
    TEST_FUNC(ceil, -2.3, -2.0);
    TEST_FUNC(ceil, 0.0, 0.0);
}

void test_copysign()
{
    TEST_FUNC_2(copysign, 1.0, -1.0, -1.0);
    TEST_FUNC_2(copysign, -1.0, 1.0, 1.0);
    TEST_FUNC_2(copysign, 0.0, -1.0, -0.0);
}

void test_cos()
{
    TEST_FUNC(cos, 0.0, 1.0);
    TEST_FUNC(cos, M_PI, -1.0);
    TEST_FUNC(cos, M_PI / 2, 0.0);
}

void test_cosh()
{
    TEST_FUNC(cosh, 0.0, 1.0);
    TEST_FUNC(cosh, 1.0, (exp(1.0) + exp(-1.0)) / 2);
    TEST_FUNC(cosh, -1.0, (exp(1.0) + exp(-1.0)) / 2);
}

void test_erf()
{
    TEST_FUNC(erf, 0.0, 0.0);
    TEST_FUNC(erf, 1.0, 0.8427007929);
    TEST_FUNC(erf, -1.0, -0.8427007929);
}

void test_erfc()
{
    TEST_FUNC(erfc, 0.0, 1.0);
    TEST_FUNC(erfc, 1.0, 0.1572992);
    TEST_FUNC(erfc, -1.0, 1.842701);
}

void test_exp()
{
    TEST_FUNC(exp, 0.0, 1.0);
    TEST_FUNC(exp, 1.0, exp(1.0));
    TEST_FUNC(exp, -1.0, exp(-1.0));
}

void test_exp2()
{
    TEST_FUNC(exp2, 0.0, 1.0);
    TEST_FUNC(exp2, 1.0, 2.0);
    TEST_FUNC(exp2, 2.0, 4.0);
}

void test_expm1()
{
    TEST_FUNC(expm1, 0.0, 0.0);
    TEST_FUNC(expm1, 1.0, exp(1.0) - 1);
    TEST_FUNC(expm1, -1.0, exp(-1.0) - 1);
}

void test_fabs()
{
    TEST_FUNC(fabs, 3.0, 3.0);
    TEST_FUNC(fabs, -3.0, 3.0);
    TEST_FUNC(fabs, 0.0, 0.0);
}

void test_fdim()
{
    TEST_FUNC_2(fdim, 5.0, 3.0, 2.0);
    TEST_FUNC_2(fdim, 3.0, 5.0, 0.0);
    TEST_FUNC_2(fdim, 5.0, 5.0, 0.0);
}

void test_floor()
{
    TEST_FUNC(floor, 2.3, 2.0);
    TEST_FUNC(floor, -2.3, -3.0);
    TEST_FUNC(floor, 0.0, 0.0);
}

void test_fma()
{
    TEST_FUNC_3(fma, 2.0, 3.0, 4.0, 10.0);
    TEST_FUNC_3(fma, 1.0, 1.0, 1.0, 2.0);
    TEST_FUNC_3(fma, -1.0, -1.0, -1.0, 0.0);
}

void test_fmax()
{
    TEST_FUNC_2(fmax, 2.0, 3.0, 3.0);
    TEST_FUNC_2(fmax, 5.0, 5.0, 5.0);
    TEST_FUNC_2(fmax, -1.0, 1.0, 1.0);
}

void test_fmin()
{
    TEST_FUNC_2(fmin, 2.0, 3.0, 2.0);
    TEST_FUNC_2(fmin, -5.0, -2.0, -5.0);
    TEST_FUNC_2(fmin, 5.0, 5.0, 5.0);
}

void test_fmod()
{
    TEST_FUNC_2(fmod, 5.0, 2.0, 1.0);
    TEST_FUNC_2(fmod, -5.0, 2.0, -1.0);
    TEST_FUNC_2(fmod, 5.0, -2.0, 1.0);
}

void test_frexp()
{
    int exp;
    TEST_FUNC_2(frexp, 5.0, &exp, 0.625);
    TEST_FUNC_2(frexp, -5.0, &exp, -0.625);
    TEST_FUNC_2(frexp, 0.0, &exp, 0.0);
}

void test_hypot()
{
    TEST_FUNC_2(hypot, 3.0, 4.0, 5.0);
    TEST_FUNC_2(hypot, 0.0, 0.0, 0.0);
    TEST_FUNC_2(hypot, 0.0, 3.0, 3.0);
}

void test_ilogb()
{
    TEST_FUNC(ilogb, 1.0, 0.0);
    TEST_FUNC(ilogb, 0.5, -1.0);
    TEST_FUNC(ilogb, 2.0, 1.0);
}

void test_ldexp()
{
    TEST_FUNC_2(ldexp, 1.0, 2, 4.0);
    TEST_FUNC_2(ldexp, 1.0, -1, 0.5);
    TEST_FUNC_2(ldexp, 1.0, 0, 1.0);
}

void test_lgamma()
{
    TEST_FUNC(lgamma, 1.0, 0.0);
    TEST_FUNC(lgamma, 2.0, 0.0);
    TEST_FUNC(lgamma, 0.5, 0.5723649429247004);
}

void test_log()
{
    TEST_FUNC(log, 1.0, 0.0);
    TEST_FUNC(log, exp(1.0), 1.0);
    TEST_FUNC(log, exp(2.0), 2.0);
}

void test_log10()
{
    TEST_FUNC(log10, 10.0, 1.0);
    TEST_FUNC(log10, 100.0, 2.0);
    TEST_FUNC(log10, 1.0, 0.0);
}

void test_log1p()
{
    TEST_FUNC(log1p, 0.0, 0.0);
    TEST_FUNC(log1p, 0.5, log(1.5));
    TEST_FUNC(log1p, -0.5, log(0.5));
}

void test_log2()
{
    TEST_FUNC(log2, 2.0, 1.0);
    TEST_FUNC(log2, 4.0, 2.0);
    TEST_FUNC(log2, 1.0, 0.0);
}

void test_logb()
{
    TEST_FUNC(logb, 1.0, 0.0);
    TEST_FUNC(logb, 2.0, 1.0);
    TEST_FUNC(logb, 0.5, -1.0);
}

void test_lrint()
{
    TEST_FUNC(lrint, 3.6, 4.0);
    TEST_FUNC(lrint, -3.6, -4.0);
    TEST_FUNC(lrint, 0.0, 0.0);
}

void test_lround()
{
    TEST_FUNC(lround, 3.6, 4.0);
    TEST_FUNC(lround, -3.6, -4.0);
    TEST_FUNC(lround, 0.0, 0.0);
}

void test_modf()
{
    double intpart;
    TEST_FUNC_2(modf, 3.5, &intpart, 0.5);
    TEST_FUNC_2(modf, -3.5, &intpart, -0.5);
    TEST_FUNC_2(modf, 0.0, &intpart, 0.0);
}

void test_nan()
{
    TEST_FUNC(nan, "", NAN);
    TEST_FUNC(nan, "", NAN);
    TEST_FUNC(nan, "", NAN);
}

void test_nearbyint()
{
    TEST_FUNC(nearbyint, 3.6, 4.0);
    TEST_FUNC(nearbyint, -3.6, -4.0);
    TEST_FUNC(nearbyint, 0.0, 0.0);
}

void test_nextafter()
{
    TEST_FUNC_2(nextafter, 1.0, 2.0, 1.0);
    TEST_FUNC_2(nextafter, 2.0, 1.0, 1.9999999999999998);
    TEST_FUNC_2(nextafter, 0.0, 1.0, 5e-324);
}

void test_nexttoward()
{
    TEST_FUNC_2(nexttoward, 1.0, 2.0, 1.0);
    TEST_FUNC_2(nexttoward, 2.0, 1.0, 1.0);
    TEST_FUNC_2(nexttoward, 0.0, 1.0, 1.0);
}

void test_pow()
{
    TEST_FUNC_2(pow, 2.0, 3.0, 8.0);
    TEST_FUNC_2(pow, 3.0, 2.0, 9.0);
    TEST_FUNC_2(pow, 4.0, 0.5, 2.0);
}

void test_remainder()
{
    TEST_FUNC_2(remainder, 5.0, 2.0, 1.0);
    TEST_FUNC_2(remainder, -5.0, 2.0, -1.0);
    TEST_FUNC_2(remainder, 5.0, -2.0, 1.0);
}

void test_remquo()
{
    int quo;
    TEST_FUNC_3(remquo, 5.0, 2.0, &quo, 1.0);
    TEST_FUNC_3(remquo, -5.0, 2.0, &quo, -1.0);
    TEST_FUNC_3(remquo, 5.0, -2.0, &quo, 1.0);
}

void test_rint()
{
    TEST_FUNC(rint, 3.6, 4.0);
    TEST_FUNC(rint, -3.6, -4.0);
    TEST_FUNC(rint, 0.0, 0.0);
}

void test_round()
{
    TEST_FUNC(round, 3.6, 4.0);
    TEST_FUNC(round, -3.6, -4.0);
    TEST_FUNC(round, 0.0, 0.0);
}

void test_scalbln()
{
    TEST_FUNC_2(scalbln, 1.0, 1, 2.0);
    TEST_FUNC_2(scalbln, 1.0, -1, 0.5);
    TEST_FUNC_2(scalbln, 1.0, 0, 1.0);
}

void test_scalbn()
{
    TEST_FUNC_2(scalbn, 1.0, 2, 4.0);
    TEST_FUNC_2(scalbn, 1.0, -1, 0.5);
    TEST_FUNC_2(scalbn, 1.0, 0, 1.0);
}

void test_sin()
{
    TEST_FUNC(sin, 0.0, 0.0);
    TEST_FUNC(sin, M_PI / 2, 1.0);
    TEST_FUNC(sin, M_PI, 0.0);
}

void test_sinh()
{
    TEST_FUNC(sinh, 0.0, 0.0);
    TEST_FUNC(sinh, 1.0, (exp(1.0) - exp(-1.0)) / 2);
    TEST_FUNC(sinh, -1.0, (exp(-1.0) - exp(1.0)) / 2);
}

void test_sqrt()
{
    TEST_FUNC(sqrt, 4.0, 2.0);
    TEST_FUNC(sqrt, 9.0, 3.0);
    TEST_FUNC(sqrt, 0.0, 0.0);
}

void test_tan()
{
    TEST_FUNC(tan, 0.0, 0.0);
    TEST_FUNC(tan, M_PI / 4, 1.0);
    TEST_FUNC(tan, -M_PI / 4, -1.0);
}

void test_tanh()
{
    TEST_FUNC(tanh, 0.0, 0.0);
    TEST_FUNC(tanh, 1.0, (exp(1.0) - exp(-1.0)) / (exp(1.0) + exp(-1.0)));
    TEST_FUNC(tanh, -1.0, -((exp(1.0) - exp(-1.0)) / (exp(1.0) + exp(-1.0))));
}

void test_tgamma()
{
    TEST_FUNC(tgamma, 1.0, 1.0);
    TEST_FUNC(tgamma, 2.0, 1.0);
    TEST_FUNC(tgamma, 5.0, 24.0);
}

void test_trunc()
{
    TEST_FUNC(trunc, 3.6, 3.0);
    TEST_FUNC(trunc, -3.6, -3.0);
    TEST_FUNC(trunc, 0.0, 0.0);
}

int main()
{
    test_acos();
    test_acosh();
    test_asin();
    test_asinh();
    test_atan();
    test_atan2();
    test_atanh();
    test_cbrt();
    test_ceil();
    test_copysign();
    test_cos();
    test_cosh();
    test_erf();
    test_erfc();
    test_exp();
    test_exp2();
    test_expm1();
    test_fabs();
    test_fdim();
    test_floor();
    test_fma();
    test_fmax();
    test_fmin();
    test_fmod();
    test_frexp();
    test_hypot();
    test_ilogb();
    test_ldexp();
    test_lgamma();
    test_log();
    test_log10();
    test_log1p();
    test_log2();
    test_logb();
    test_lrint();
    test_lround();
    test_modf();
    test_nan();
    test_nearbyint();
    test_nextafter();
    // test_nexttoward(); // NOTE: Not implement yet.
    test_pow();
    test_remainder();
    test_remquo();
    test_rint();
    test_round();
    test_scalbln();
    test_scalbn();
    test_sin();
    test_sinh();
    test_sqrt();
    test_tan();
    test_tanh();
    test_tgamma();
    test_trunc();
    return 0;
}
