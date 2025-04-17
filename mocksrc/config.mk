MOCK_LIBC=../../ulib/mocklibc_lib
PLATFORM=riscv64-linux-musl-
CC=$(PLATFORM)gcc
LD=$(PLATFORM)ld
STRIP=$(PLATFORM)strip
OBJCOPY=$(PLATFORM)objcopy

MOCK_LIBC_INC=$(MOCK_LIBC)/include
MOCK_LIBC_LIB=$(MOCK_LIBC)/lib
MOCK_LIBC_CRT1=$(MOCK_LIBC)/obj/crt1.o

LINK_LD=./link.ld

STATIC_CFLAGS += -nostdlib -nodefaultlibs -ffreestanding -O0 -mcmodel=medany -nostartfiles
STATIC_CFLAGS += -static -no-pie
