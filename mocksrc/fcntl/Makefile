# 读取配置文件
include ../config.mk

OUTFILE?=apps.bin

SRC_NAME?=fcntl
SRC_C=$(SRC_NAME).c
SRC_O=$(SRC_NAME).o
SRC_BIN=$(SRC_NAME).bin

all: static dynamic

static:
	$(CC) $(STATIC_CFLAGS) -I$(MOCK_LIBC_INC) -c $(SRC_C) -o $(SRC_O)
	$(LD) $(SRC_O) $(MOCK_LIBC_CRT1) $(MOCK_LIBC_LIB)/libmock.a -T$(LINK_LD) -o $(SRC_BIN)
	#
	$(STRIP) -s $(SRC_BIN)
	mv -f $(SRC_BIN) $(OUTFILE)

dynamic:
	$(CC) $(SRC_C) -o $(SRC_BIN)
	#
	$(STRIP) -s $(SRC_BIN)
	mv -f $(SRC_BIN) $(OUTFILE)

.PHONY: all static dynamic
