# 读取配置文件
include ../config.mk
OUTFILE?=apps.bin

SRC_C=startup.c sqlite3.c
SRC_BIN=app.bin

DEFINE_FLAGS += -DSQLITE_OMIT_LOAD_EXTENSION -DSQLITE_THREADSAFE=0

all: static dynamic

static:
	$(CC) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC_INC) -I./ -c startup.c -o startup.c
	$(CC) $(DEFINE_FLAGS) $(STATIC_CFLAGS) -I$(MOCK_LIBC_INC) -I./ -c sqlite3.c -o sqlite3.o
	$(LD) startup.o sqlite3.o $(MOCK_LIBC_CRT1) $(MOCK_LIBC_LIB)/libmock.a -T$(LINK_LD) -o $(SRC_BIN)
	#
	$(STRIP) -s $(SRC_BIN)
	mv -f $(SRC_BIN) $(OUTFILE).bin

dynamic:
	$(CC) $(SRC_C) $(DEFINE_FLAGS) -I./ -o $(SRC_BIN)
	#
	$(STRIP) -s $(SRC_BIN)
	mv -f $(SRC_BIN) $(OUTFILE).bin

.PHONY: all static dynamic
