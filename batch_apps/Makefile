# 可修改变量设置
#
# 应用程序目录
APP_DIR ?= APPS
# 应用程序原码目录
SRC_DIR ?= SRCS
# 是否包含 Lib，默认包含 (1)
INCLUDE_LIB ?= 1
# 库目录
MOCK_LIBC_SO ?= ../ulib/mocklibc_lib/lib/libmock.so
# 执行脚本
SCRIPT ?= script.txt

# 不可改变量设置
#
# 构建目录
BUILD_DIR = build
# 编码后执行脚本
ENCODED_SCRIPT = $(BUILD_DIR)/encoded_script.bin
# 生成后的头部数据
HEADER = $(BUILD_DIR)/header.bin
# 输出文件名
OUTPUT = $(BUILD_DIR)/apps.bin
# PAYLOAD 目录
PAYLOAD_DIR = ../payload

# 处理 Lib 选项
ifeq ($(INCLUDE_LIB), 1)
	LIB_ARG = --lib_file $(MOCK_LIBC_SO)
else
	LIB_ARG =
endif

all: toplash

# 打包头部 + App + Lib + 脚本的最终二进制文件
toplash:
	mkdir -p $(BUILD_DIR) # 创建构建目录
	rm -f $(OUTPUT)
	
	# 调用 `python` 脚本编码执行脚本
	python3 script_encoder.py \
		--infile $(SCRIPT) \
		--outfile $(ENCODED_SCRIPT)

	# 调用 `python` 脚本生成头部元信息
	python3 generate_header.py \
		--apps_dir $(APP_DIR) \
		$(LIB_ARG) \
		--script_file $(ENCODED_SCRIPT) \
		--out_file $(HEADER)

	# 调用打包脚本，将头部、应用、Lib库和脚本按顺序写入到 `apps.bin`
	python3 pack_apps.py \
		--header_file $(HEADER) \
		--apps_dir $(APP_DIR) \
		$(LIB_ARG) \
		--script_file $(ENCODED_SCRIPT) \
		--out_file $(OUTPUT)

	# 通过 `dd` 填充空字节，确保最终文件大小为 `32MB`
	dd if=/dev/zero bs=1 count=0 seek=32M of=$(OUTPUT)

	# 将最终文件移动到 `payload` 目录
	mkdir -p $(PAYLOAD_DIR)
	cp $(OUTPUT) $(PAYLOAD_DIR)/apps.bin

# 清理生成的所有中间文件
clean:
	rm -rf $(BUILD_DIR)

.PHONY: all toplash clean
