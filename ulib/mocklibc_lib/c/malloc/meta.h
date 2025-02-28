#ifndef MALLOC_META_H
#define MALLOC_META_H

#include <errno.h>
#include <limits.h>
#include <stdint.h>

// __attribute__((__visibility__("hidden"))) extern const uint16_t size_classes[];
//
// static inline int size_to_class(size_t n)
// {
//     n = (n + IB - 1) >> 4;
//     if (n < 10)
//         return n;
//     n++;
//     int i = (28 - a_clz_32(n)) * 4 + 8;
//     if (n > size_classes[i + 1])
//         i += 2;
//     if (n > size_classes[i])
//         i++;
//     return i;
// }

#endif
