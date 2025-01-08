import os
import subprocess

CC = "riscv64-linux-musl-gcc"

DYNAMIC_FLAG = [
]

def run(cmd):
    subprocess.run(cmd, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)

def generate_bin(elf_file):
    # 创建32MB的空文件
    run("dd if=/dev/zero of=apps.bin bs=1M count=32")
    
    # 写入应用大小(8字节)
    app_size = os.path.getsize(elf_file)
    size_hex = format(app_size, '016x')
    with open('apps.bin', 'rb+') as f:
        # 将16进制字符串转换为字节并写入(需要反转字节序)
        size_bytes = bytes.fromhex(size_hex)[::-1]
        f.write(size_bytes)
    
    # 写入应用程序内容
    run(f"dd if={elf_file} of=apps.bin conv=notrunc bs=1 seek=8")
    
    # 复制到上级目录
    run("mv apps.bin ../apps.bin")

def build_dynamic_hello_auto(elf_file):
    command = f"{CC} -v {elf_file}.c {' '.join(DYNAMIC_FLAG)} -o {elf_file}"
    run(command)
    run(f"riscv64-linux-musl-objdump -d {elf_file} > {elf_file}.S")
    run(f"riscv64-linux-musl-readelf -a {elf_file} > {elf_file}.elf")
    run(f"riscv64-linux-musl-objdump -x -d {elf_file} > {elf_file}.dump")
    generate_bin(elf_file)

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python build.py <elf_file>")
        sys.exit(1)
    
    build_dynamic_hello_auto(sys.argv[1])
