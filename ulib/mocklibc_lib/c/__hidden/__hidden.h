#ifndef __HIDDEN_H
#define __HIDDEN_H

#include <features.h>

hidden void __getf2();
hidden void __eqtf2();
hidden void __addtf3();
hidden void __floatsitf();
hidden void __divtf3();
hidden void __letf2();

hidden void __fixtfdi();
hidden void __fixtfsi();
hidden void __lttf2();
hidden void __netf2();
hidden void __extenddftf2();
hidden void __extendsftf2();
hidden void __multf3();
hidden void __gttf2();
hidden void __subtf3();
hidden void __trunctfsf2();
hidden void __trunctfdf2();

hidden void __floatditf();
hidden _Complex double __muldc3(double, double, double, double);
hidden _Complex float __mulsc3(float, float, float, float);
hidden _Complex long double __multc3(long double, long double, long double, long double);

#endif // !__HIDDEN_H
