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

toapps:
	cp $(DIR)/$(OUTFILE) $(BATCH_APPS)
	cp $(DIR)/$(OUTFILE) $(UNI_APPS)

.PHONY: all
