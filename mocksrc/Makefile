# 读取配置文件
include config.mk

BATCH_APPS ?= ../batch_apps/APPS/
UNI_APPS ?= ../uni_apps/APPS/

OUTFILE ?= apps.bin
DIR ?=
TYPE ?= dynamic

all: subdir toapps

subdir:
	$(MAKE) -C $(DIR) $(TYPE) OUTFILE=$(OUTFILE)

# 添加尾部的.bin以便避免被git跟蹤
toapps:
	cp $(DIR)/$(OUTFILE).bin $(BATCH_APPS)$(OUTFILE)
	cp $(DIR)/$(OUTFILE).bin $(UNI_APPS)$(OUTFILE)

.PHONY: all
