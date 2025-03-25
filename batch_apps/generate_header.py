import struct
import os
import argparse

MAGIC_NUMBER = 0x5F7265646165685F


def generate_header(apps_dir, lib_path, script_path, output_file):
    header_data = []
    base_offset = 8 + 4 + 8  # 魔数（8字节）+ 应用总数（4字节）+ 头部大小（8字节）

    # 获取应用程序文件信息
    apps_info = []
    for app in sorted(os.listdir(apps_dir)):  # 按文件名排序，确保顺序一致
        app_name = app.encode("utf-8") + b"\0"  # 应用名字，在名字后面添加 `\0`
        app_size = os.path.getsize(os.path.join(apps_dir, app))
        entry_size = (
            8 + len(app_name) + 8
        )  # 应用大小（8字节）+ 变长名字长度 + 起始地址（8字节）
        base_offset += entry_size
        apps_info.append((app_size, app_name))

    # 增加 `Lib` 和 `Script` 的偏移量部分
    base_offset += 8 + 8  # `Lib` 大小（8字节）+ 偏移地址（8字节）
    base_offset += 8 + 8  # `Script` 大小（8字节） + 偏移地址（8字节）

    # 基于计算好的 `base_offset`，确定应用存放起始地址
    offset = base_offset

    # 获取应用程序文件信息并添加实际的其实偏移量
    apps_offsets = []
    for app_size, app_name in apps_info:
        apps_offsets.append((app_size, app_name, offset))
        offset += app_size

    # 获取 Lib库大小和偏移
    lib_size = os.path.getsize(lib_path)
    lib_offset = offset
    offset += lib_size

    # 获取执行脚本大小和偏移
    script_size = os.path.getsize(script_path)
    script_offset = offset

    # 构建头部
    header_data.append(struct.pack("L", MAGIC_NUMBER))  # 魔数
    header_data.append(struct.pack("I", len(apps_offsets)))  # 应用总数
    header_data.append(struct.pack("L", base_offset))  # 头部大小

    for app_size, app_name, app_offset in apps_offsets:
        header_data.append(struct.pack("L", app_size))  # 应用大小
        header_data.append(app_name)  # 应用名字，变长，包含尾部的 `\0`
        header_data.append(struct.pack("L", app_offset))  # 应用起始地址

    header_data.append(struct.pack("L", lib_size))  # Lib库大小
    header_data.append(struct.pack("L", lib_offset))  # Lib库起始地址
    header_data.append(struct.pack("L", script_size))  # 执行脚本大小
    header_data.append(struct.pack("L", script_offset))  # 执行脚本起始地址

    # 将头部写入文件
    with open(output_file, "wb") as f:
        for entry in header_data:
            f.write(entry)

    print(f"Header generated and saved to {output_file}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate header for OS batch loader.")
    parser.add_argument(
        "--appsdir",
        required=True,
        help="Directory containing application binary files.",
    )
    parser.add_argument("--lib", required=True, help="Path to the Lib library binary.")
    parser.add_argument(
        "--script", required=True, help="Path to the encoded script file."
    )
    parser.add_argument(
        "--outfile", required=True, help="Output file for the generated header."
    )
    args = parser.parse_args()

    generate_header(args.appsdir, args.lib, args.script, args.outfile)
