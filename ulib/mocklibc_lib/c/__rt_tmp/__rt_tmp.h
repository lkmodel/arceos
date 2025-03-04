#ifndef __RT_TMP_H
#define __RT_TMP_H

#include <features.h>
#include <stdint.h>
void __cxa_finalize(void *d);
void _init(void);
void _fini(void);
void __register_frame_info(void);
void __deregister_frame_info(void);
void _ITM_registerTMCloneTable(void);
void _ITM_deregisterTMCloneTable(void);

#endif
