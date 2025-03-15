MOCK_LIBC=../../ulib/mocklibc_lib
PLATFORM=riscv64-linux-musl
CC=$(PLATFORM)-gcc
LD=$(PLATFORM)-ld
STRIP=$(PLATFORM)-strip
OBJCOPY=$(PLATFORM)-objcopy

READELF := riscv64-unknown-elf-readelf
HEADER_FILE=header.bin
OUTPUT_BIN?=apps.bin

SRC?=
# SRC_F = crashtest1.c NOT SUPPORT [sqlite.h]
# SRC_F = dbfuzz2.c 还没有修整好
# SRC_F = fp-speed-1.c WRONG
# SRC_F = optfuzz.c 源文件缺少
# SRC_F = threadtest1.c NOT SUPPORT [sqlite.h]
# SRC_F = threadtest2.c NOT SUPPORT [sqlite.h]
# SRC_F = threadtest3.c NOT SUPPORT [test_multipl.h]
# SRC_F = threadtest4.c NOT SUPPORT [sqlite.h]
# SRC_F = threadtest5.c NOT SUPPORT [sqlite.h]
# SRC_O=$(SRC_FILES:.c=.o)
SRC_BIN=$(OUTPUT_BIN)
SRC_DIR=$(SRC)_apps

CFLAGS := -nostdinc -nostdlib -nodefaultlibs -ffreestanding -O0 -mcmodel=medany -nostartfiles
DEFINE_FLAGS += -DSQLITE_OMIT_LOAD_EXTENSION -DSQLITE_THREADSAFE=0
STATIC_CFLAGS := -static -no-pie
LINK_FLAGS := # -T$(ARCEOS_HOME)/mockc_apps/link.ld

all: default_build

default_build: dynamic

static:
	$(CC) $(CFLAGS) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC)/include -I./ -c $(SRC_F).c -o $(SRC_F).o
	$(CC) $(CFLAGS) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC)/include -I./ -c sqlite3.c -o sqlite3.o
	$(LD) $(SRC_F).o sqlite3.o $(MOCK_LIBC)/obj/crt1.o $(MOCK_LIBC)/lib/libmock.a $(LINK_FLAGS) -T./link.ld -o $(SRC_BIN)
	$(STRIP) -s $(SRC_BIN)

dynamic:
	$(CC) $(SRC_F) sqlite3.c $(DEFINE_FLAGS) -o $(SRC_BIN) -I./
	$(STRIP) -s $(SRC_BIN)

gcc: 
	gcc $(SRC_F) $(DEFINE_FLAGS) -o gcc.out -I./

.PHONY: all build
