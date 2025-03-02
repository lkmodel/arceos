#ifndef __RT_INTEGER_H
#define __RT_INTEGER_H

#include <features.h>
#include <stdint.h>
// 3.1.1 Arithmetic functions

// int __ashlsi3(int a, int b);
// long __ashldi3(long a, int b);
// long long __ashlti3(long long a, int b);
// // These functions return the result of shifting a left by b bits.
//
// int __ashrsi3(int a, int b);
// long __ashrdi3(long a, int b);
// long long __ashrti3(long long a, int b);
// // These functions return the result of arithmetically shifting a right by b bits.
//
// int __divsi3(int a, int b);
// long __divdi3(long a, long b);
// long long __divti3(long long a, long long b);
// // These functions return the quotient of the signed division of `a` and `b`.
//
// int __lshrsi3(int a, int b);
// long __lshrdi3(long a, int b);
// long long __lshrti3(long long a, int b);
// // These functions return the result of logically shifting a right by b bits.
//
// int __modsi3(int a, int b);
// long __moddi3(long a, long b);
// long long __modti3(long long a, long long b);
// // These functions return the remainder of the signed division of `a` and `b`.
//
// int __mulsi3(int a, int b);
// long __muldi3(long a, long b);
// long long __multi3(long long a, long long b);
// // These functions return the product of `a` and `b`.
//
// long __negdi2(long a);
// long long __negti2(long long a);
// // These functions return the negation of `a`.
//
// unsigned int __udivsi3(unsigned int a, unsigned int b);
// unsigned long __udivdi3(unsigned long a, unsigned long b);
// unsigned long long __udivti3(unsigned long long a, unsigned long long b);
// // These functions return the quotient of the unsigned division of `a` and `b`.
//
// unsigned long __udivmoddi4(unsigned long a, unsigned long b, unsigned long *c);
// unsigned long long __udivmodti4(unsigned long long a, unsigned long long b, unsigned long long
// *c);
// // These functions calculate both the quotient and remainder of the unsigned division of `a` and
// // `b`. The return value is the quotient, and the remainder is placed in variable pointed to by
// `c`.
//
// unsigned int __umodsi3(unsigned int a, unsigned int b);
// unsigned long __umoddi3(unsigned long a, unsigned long b);
// unsigned long long __umodti3(unsigned long long a, unsigned long long b);
// // These functions return the remainder of the unsigned division of `a` and `b`.

// // 3.1.2 Comparison functions
//
// int __cmpdi2(long a, long b);
// int __cmpti2(long long a, long long b);
// // These functions perform a signed comparison of `a` and `b`. If `a` is less than `b`, they
// return
// // 0; if `a` is greater than `b`, they return 2; and if `a` and `b` are equal they return 1.
//
// int __ucmpdi2(unsigned long a, unsigned long b);
// int __ucmpti2(unsigned long long a, unsigned long long b);
// // These functions perform an unsigned comparison of `a` and `b`. If `a` is less than `b`, they
// // return 0; if `a` is greater than `b`, they return 2; and if `a` and `b` are equal they
// return 1.
//
// // 3.1.3 Trapping arithmetic functions
//
// int __absvsi2(int a);
// long __absvdi2(long a);
// // These functions return the absolute value of `a`.
//
// int __addvsi3(int a, int b);
// long __addvdi3(long a, long b);
// // These functions return the sum of `a` and `b`; that is `a` + `b`.
//
// int __mulvsi3(int a, int b);
// long __mulvdi3(long a, long b);
// // The functions return the product of `a` and `b`; that is `a` * `b`.
//
// int __negvsi2(int a);
// long __negvdi2(long a);
// // These functions return the negation of `a`; that is `-a`.
//
// int __subvsi3(int a, int b);
// long __subvdi3(long a, long b);
// // These functions return the difference between `b` and `a`; that is `a` - `b`

// 3.1.4 Bit operations

int __clzsi2(unsigned int a);
int __clzdi2(unsigned long a);
int __clzti2(unsigned long long a);
// These functions return the number of leading 0-bits in `a`, starting at the most significant bit
// position. If `a` is zero, the result is undefined.

// int __ctzsi2(unsigned int a);
// int __ctzdi2(unsigned long a);
// int __ctzti2(unsigned long long a);
// // These functions return the number of trailing 0-bits in `a`, starting at the least significant
// // bit position. If `a` is zero, the result is undefined.
//
// int __ffsdi2(unsigned long a);
// int __ffsti2(unsigned long long a);
// // These functions return the index of the least significant 1-bit in `a`, or the value zero if
// `a`
// // is zero. The least significant bit is index one.
//
// int __paritysi2(unsigned int a);
// int __paritydi2(unsigned long a);
// int __parityti2(unsigned long long a);
// // These functions return the value zero if the number of bits set in `a` is even, and the value
// one
// // otherwise.
//
// int __popcountsi2(unsigned int a);
// int __popcountdi2(unsigned long a);
// int __popcountti2(unsigned long long a);
// These functions return the number of bits set in `a`.

int32_t __bswapsi2(int32_t a);
int64_t __bswapdi2(int64_t a);
// These functions return the a `byteswapped`.

// //3.1.5 Bit-precise integer arithmetic functions
//
// void __mulbitint3 (UBILtype *ret, int32_t retprec, const UBILtype *u, int32_t uprec, const
// UBILtype *v, int32_t vprec);
// // This function multiplies bit-precise integer operands u and v and stores result into retprec
// precision bit-precise integer result ret.
//
// void __divmodbitint4 (UBILtype *q, int32_t qprec, UBILtype *r, int32_t rprec, const UBILtype *u,
// int32_t uprec, const UBILtype *v, int32_t vprec);
// // This function divides bit-precise integer operands u and v and stores quotient into qprec
// precision bit-precise integer result q (unless q is NULL and qprec is 0, in that case quotient is
// not stored anywhere) and remainder into rprec precision bit-precise integer result r (similarly,
// unless r is NULL and rprec is 0).

#endif
