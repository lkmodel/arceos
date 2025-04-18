import os
import sys
import struct
import argparse

SCRIPT_MAGIC = 0x5F7470697263735F  # "_script_"
HEADER_MAGIC = 0x5F7265646165685F  # "_header_"


def encode_script_and_extract_app(apps_dir, script_file, encoded_out):
    with open(script_file, "r") as f:
        lines = [line.strip() for line in f if line.strip()]
    if len(lines) != 1:
        sys.exit("❌ 僅支持一行命令，請檢查腳本文件")

    parts = lines[0].strip().split()
    if not parts:
        sys.exit("❌ 命令行為空，請填入有效命令")

    app_name = os.path.basename(parts[0])
    app_path = os.path.join(apps_dir, app_name)
    if not os.path.isfile(app_path):
        sys.exit(f"❌ 未找到應用程式 '{app_name}' 於 '{apps_dir}' 中")

    argc = len(parts)
    with open(encoded_out, "wb") as outf:
        outf.write(struct.pack("Q", SCRIPT_MAGIC))  # 魔數
        outf.write(struct.pack("I", argc))  # 參數數量

        argv = parts  # argv[0] 是程序名，後面是參數
        argc = len(argv)

        outf.write(struct.pack("I", argc))  # 写入 argc

        # 写入参数`argv[i]`长度和具体内容，每个参数加上'\0'
        for arg in argv:
            arg_bytes = (arg + "\0").encode("utf-8")  # 加上 C 字符串的 \0 终止符
            outf.write(arg_bytes)  # 参数值

    print(f"✅ 指令編碼完成：'{script_file}' -> '{encoded_out}'")
    return app_name, app_path


def generate_header(app_path, lib_path, script_path, output_file):
    base_offset = 8  # 魔数（8字节）
    header = []

    # App
    app_name = os.path.basename(app_path)
    app_name_bytes = app_name.encode("utf-8") + b"\0"  # 应用名字，在名字后面添加 `\0`
    app_size = os.path.getsize(app_path)
    base_offset += (
        8 + len(app_name_bytes) + 8
    )  # 应用大小（8字节）+ 变长名字长度 + 起始地址（8字节）

    # Lib
    if lib_path:
        lib_size = os.path.getsize(lib_path)
    else:
        lib_size = 0

    base_offset += 8 + 8  # `Lib` 大小（8字节）+ 偏移地址（8字节）

    # Script
    script_size = os.path.getsize(script_path)

    # 基于计算好的 `base_offset`，确定存放起始地址
    base_offset += 8 + 8  # `Script` 大小（8字节） + 偏移地址（8字节）
    app_offset = base_offset
    if lib_path:
        lib_offset = base_offset + app_size
    else:
        lib_offset = 0
    script_offset = base_offset + app_size + lib_size

    # Header structure
    header.append(struct.pack("Q", HEADER_MAGIC))
    header.append(struct.pack("Q", app_size))
    header.append(app_name_bytes)
    header.append(struct.pack("Q", app_offset))
    header.append(struct.pack("Q", lib_size))
    header.append(struct.pack("Q", lib_offset))
    header.append(struct.pack("Q", script_size))
    header.append(struct.pack("Q", script_offset))

    with open(output_file, "wb") as f:
        for part in header:
            f.write(part)

    print(f"[✓] Header 打包完成: '{output_file}'")
    print(f"    - App: {app_name} size={app_size} offset={app_offset}")
    print(f"    - Lib: size={lib_size} offset={lib_offset}")
    print(f"    - Script: size={script_size} offset={script_offset}")


def pack_app(header, app_path, lib_path, script_path, outfile):
    with open(outfile, "wb") as output:
        # 1. 写入头部
        print(f"Writing header from {header}")
        with open(header, "rb") as header_file:
            output.write(header_file.read())

        # 2. 写入应用程序
        print(f"Writing apps from {app_path}")
        with open(app_path, "rb") as app_file:
            output.write(app_file.read())

        # 3. 写入 Lib 库(如果提供了)
        if lib_path:
            print(f"Writing lib from {lib_path}")
            with open(lib_path, "rb") as lib_file:
                output.write(lib_file.read())
        else:
            print("Skipping lib file, not provided.")

        # 4. 写入编码后的执行脚本
        print(f"Writing encoded script from {script_path}")
        with open(script_path, "rb") as script_file:
            output.write(script_file.read())

    print(f"Final packed binary written to {outfile}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="單命令腳本 + Header + apps.bin 打包工具"
    )
    parser.add_argument("--apps_dir", required=True, help="應用程式所在目錄")
    parser.add_argument("--script", required=True, help="單行命令腳本檔案 (.txt)")
    parser.add_argument("--lib", help="可選的 library 檔案 (.so)")
    parser.add_argument("--script_out", required=True, help="腳本二進位檔")
    parser.add_argument("--header_out", required=True, help="header 檔案")
    parser.add_argument("--outfile", required=True, help="最終打包 *.bin 檔案")
    args = parser.parse_args()

    app_name, app_path = encode_script_and_extract_app(
        args.apps_dir, args.script, args.script_out
    )
    generate_header(app_path, args.lib, args.script_out, args.header_out)
    pack_app(args.header_out, app_path, args.lib, args.script_out, args.outfile)
