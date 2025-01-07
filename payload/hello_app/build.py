import os
import subprocess

APP_SIZE = 0
CC = "riscv64-linux-musl-gcc"

STATIC_FLAG = [
    "-nostdlib",
    "-nostartfiles",
    "-nodefaultlibs",
    "-ffreestanding",
    "-O0",
    "-mcmodel=medany",
    "-static",
    "-fno-pie",
    "-L./",
    "-lmocklibc",
    "-T./linker.ld",
]

DYNAMIC_FLAG = [
    "-nostdlib",
    "-nostartfiles",
    "-nodefaultlibs",
    "-ffreestanding",
    "-O0",
    "-mcmodel=medany",
    "-L./",
    "-lmocklibc",
]

def run_command(command):
    result = subprocess.run(command, shell=True, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print(result.stdout.decode())
    if result.stderr:
        print(result.stderr.decode())

def clean():
    files_to_remove = ["hello", "hello.elf", "hello.dump", "apps.bin"]
    for file in files_to_remove:
        if os.path.exists(file):
            os.remove(file)
    print("Cleaned up generated files.")

def build_static_hello_auto():
    command = f"{CC} hello.c {' '.join(STATIC_FLAG)} -o hello"
    run_command(command)
    generate_bin()

def build_dynamic_hello_auto():
    command = f"{CC} hello.c {' '.join(DYNAMIC_FLAG)} -o hello"
    run_command(command)
    generate_bin()

def generate_bin():
    run_command("riscv64-linux-musl-readelf -a hello > hello.elf")
    run_command("riscv64-linux-musl-objdump -x -d hello > hello.dump")

    run_command("dd if=/dev/zero of=./apps.bin bs=1M count=32")
    app_size = os.path.getsize("./hello")
    write_app_size(app_size)

    run_command("dd if=./hello of=./apps.bin conv=notrunc bs=1 seek=8")
    run_command("mv ./apps.bin ../apps.bin")

def write_app_size(size):
    run_command(f"printf \"%016x\" {size} | tac -rs .. | xxd -r -p > ./temp.bin")
    run_command("dd if=./temp.bin of=./apps.bin conv=notrunc bs=1 seek=0")
    os.remove("./temp.bin")

if __name__ == "__main__":
    import sys
    if len(sys.argv) != 2:
        print("Usage: python build.py [dynamic|static|clean]")
        sys.exit(1)

    target = sys.argv[1]
    if target == "dynamic":
        clean()
        build_dynamic_hello_auto()
    elif target == "static":
        clean()
        build_static_hello_auto()
    elif target == "clean":
        clean()
    else:
        print(f"Unknown target: {target}")
        sys.exit(1)
