SRC ?=
MOCK_LIBC=../ulib/mocklibc_lib
TYPE ?= static

all: subdir toplash

# 对于`subdir`的要求，仅仅是要在同名文件夹下，存在文件`apps.bin`就可以。
subdir:
	$(MAKE) -C $(SRC) $(TYPE)

toplash:
	rm -f ../payload/apps.bin
	dd if=/dev/zero of=./apps.bin bs=1M count=32
	touch size.bin
	bash size.sh --infile ./$(SRC)/apps.bin --outfile size.bin
	dd if=./size.bin of=./tmp.bin bs=1 count=$$(stat -c %s ./size.bin)
	rm size.bin
	dd if=./$(SRC)/apps.bin of=./tmp.bin bs=1 oflag=append conv=notrunc
	#
	touch size.bin
	bash size.sh --infile $(MOCK_LIBC)/lib/libmock.so --outfile size.bin
	dd if=./size.bin of=./tmp.bin bs=1 oflag=append conv=notrunc
	rm size.bin
	dd if=$(MOCK_LIBC)/lib/libmock.so of=./tmp.bin bs=1 oflag=append conv=notrunc
	dd if=./tmp.bin of=./apps.bin conv=notrunc
	rm ./tmp.bin
	cp ./apps.bin ../payload/apps.bin

.PHONY: all toplash
