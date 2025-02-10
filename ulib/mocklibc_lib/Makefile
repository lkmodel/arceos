# 读取配置文件
include config.mk

PLATFORM=riscv64-linux-musl
CC=$(PLATFORM)-gcc
AS=$(PLATFORM)-as
LD=$(PLATFORM)-ld
AR=$(PLATFORM)-ar
STRIP=$(PLATFORM)-strip
OBJCOPY=$(PLATFORM)-objcopy

INTERNAL_INCLUDE = -I./c/math/ -I./c/include/ -I./include -I./c/internal/ -I./c/time/ -I./arch/riscv64/ -I./c/__hidden/
CFLAGS = -nostartfiles -ffreestanding -nostdlib -mcmodel=medany $(INTERNAL_INCLUDE) -fPIC
# CFLAGS += -fvisibility=hidden # 尝试修正compile-rt被-nolib屏蔽的问题，也就是__getf2符号没有的问题。
ASFLAGS = 
LDFLAGS := #"$($(CC) -print-libgcc-file-name)"# -T link.ld
#LINK_FLAGS += /home/marinatoo/App/riscv64-linux-musl-cross/bin/../lib/gcc/riscv64-linux-musl/11.2.1/libgcc.a # 尝试修正compile-rt被-nolib屏蔽的问题，也就是__getf2符号没有的问题。
# 定义源目录和目标目录
SRC_DIR := c
OBJ_DIR := obj
LIB_DIR = lib

STATIC_LIB = $(LIB_DIR)/libmock.a
DYNAMIC_LIB = $(LIB_DIR)/libmock.so
LIBSRC +=  $(wildcard $(SRC_DIR)/stdio/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/ctype/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/stdlib/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/time/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/string/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/fenv/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/thread/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/unistd/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/__hidden/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/__file/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/math/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/complex/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/errno/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/internal/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/fcntl/*.c)
LIBSRC += $(wildcard $(SRC_DIR)/linux/*.c)
# LIBSRC += $(wildcard $(SRC_DIR)/network/*.c)

# 根据配置选择 malloc 库实现
ifeq ($(USE_SAFE_MALLOC), 1)
	LIBSRC += $(wildcard $(SRC_DIR)/safe_malloc/*.c)
else
	LIBSRC += # TODO
endif

# 编译选项
ifeq ($(OPTIMIZE), 0)
	CFLAGS += -O0
else ifeq ($(OPTIMIZE), 1)
	CFLAGS += -O1
else ifeq ($(OPTIMIZE), 2)
	CFLAGS += -O2
else ifeq ($(OPTIMIZE), 3)
	CFLAGS += -O3
else
	CFLAGS += -Os
endif

LIB_OBJ = $(patsubst c/%, $(OBJ_DIR)/%, $(patsubst %.c, %.o, $(LIBSRC)))
CRT_OBJ = $(OBJ_DIR)/crt1.o

# 获取所有子目录名，并替换为 obj/子目录 的结构
SUBDIRS := $(subst $(SRC_DIR)/,$(OBJ_DIR)/,$(wildcard $(SRC_DIR)/*))

all: clean makedir $(STATIC_LIB) $(DYNAMIC_LIB) $(CRT_OBJ) $(LIB_OBJ)

static: clean makedir $(STATIC_LIB) $(CRT_OBJ) $(LIB_OBJ)

dynamic: clean makedir $(DYNAMIC_LIB) $(CRT_OBJ) $(LIB_OBJ)

$(LIB_OBJ): | $(dir $(LIB_OBJ))
$(CRT_OBJ): | $(dir $(CRT_OBJ))

$(CRT_OBJ): c/crt1.c include/crt_arch.h
	$(CC) $(CFLAGS) -c -o $@ $<

$(OBJ_DIR)/%.o: c/%.c
	$(CC) $(CFLAGS) -c -o $@ $<

$(OBJ_DIR)/%.o: c/%.s
	$(AS) $(ASFLAGS) -o $@ $<

$(STATIC_LIB): $(LIB_OBJ) $(CRT_OBJ)
	$(AR) rcs $@ $(LIB_OBJ)

$(DYNAMIC_LIB): $(LIB_OBJ) $(CRT_OBJ)
	$(CC) $(CFLAGS) $(LDFLAGS) -shared -fPIC -o $@ $^

makedir:
	mkdir -p $(OBJ_DIR)
	mkdir -p $(LIB_DIR)
	mkdir -p $(SUBDIRS)

clean:
	@rm -rf $(LIB_DIR)
	@rm -rf $(OBJ_DIR)

.PHONYD: all clean static dynamic makedir
