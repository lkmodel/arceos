MOCK_LIBC=../../ulib/mocklibc_lib
PLATFORM=riscv64-linux-musl
CC=$(PLATFORM)-gcc
LD=$(PLATFORM)-ld
STRIP=$(PLATFORM)-strip
OBJCOPY=$(PLATFORM)-objcopy

APP_CHOICE?=
READELF := riscv64-unknown-elf-readelf
HEADER_FILE=header.bin
OUTPUT_BIN?=apps.bin

SRC?=
SRC_F1 = threadtest3.c
SRC_F2 = test_multiplex.c
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
	$(CC) $(CFLAGS) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC)/include -I./ -c $(SRC_F1).c -o $(SRC_F1).o
	$(CC) $(CFLAGS) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC)/include -I./ -c $(SRC_F2).c -o $(SRC_F2).o
	$(CC) $(CFLAGS) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC)/include -I./ -c sqlite3.c -o sqlite3.o
	$(LD) $(SRC_F1).o $(SRC_F2).o sqlite3.o $(MOCK_LIBC)/obj/crt1.o $(MOCK_LIBC)/lib/libmock.a $(LINK_FLAGS) -T./link.ld -o $(SRC_BIN)
	$(STRIP) -s $(SRC_BIN)

dynamic:
	$(CC) $(SRC_F1) $(SRC_F2) sqlite3.c $(DEFINE_FLAGS) -o $(SRC_BIN) -I./
	$(STRIP) -s $(SRC_BIN)


.PHONY: all build
