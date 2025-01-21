#ifndef __HIDDEN_H
#define __HIDDEN_H

#include <features.h>

void __getf2();
void __eqtf2();
void __addtf3();
void __floatsitf();
void __divtf3();
void __letf2();

void __fixtfdi();
void __fixtfsi();
void __lttf2();
void __netf2();
void __extenddftf2();
void __extendsftf2();
void __multf3();
void __gttf2();
void __subtf3();
void __trunctfsf2();
void __trunctfdf2();

void __floatditf();
_Complex double __muldc3(double, double, double, double);
_Complex float __mulsc3(float, float, float, float);
_Complex long double __multc3(long double, long double, long double, long double);

#endif // !__HIDDEN_H
