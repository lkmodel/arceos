import struct
import argparse

MAGIC = 0x5F7470697263735F  # `_script_`


def encode_script(input_file, output_file):
    with open(input_file, "r") as f:
        lines = f.readlines()

    # 打开二进制文件写入
    with open(output_file, "wb") as outf:
        outf.write(struct.pack("L", MAGIC))  # 魔术数
        outf.write(struct.pack("I", len(lines)))  # 总命令行数量

        for line in lines:
            parts = line.strip().split()  # 分割命令名和参数
            if len(parts) == 0:
                continue  # 跳过空行

            # ANSI-C argv[] 风格：将第一个单词视为 argv[0]
            argv = parts  # argv[0] 是程序名，后面是参数
            argc = len(argv)

            # 写入命令行的标记（`0xff`作为标记位）
            outf.write(struct.pack("B", 0xFF))
            outf.write(struct.pack("I", argc))  # 写入 argc

            # 写入参数`argv[i]`长度和具体内容，每个参数加上'\0'
            for arg in argv:
                arg_bytes = (arg + "\0").encode("utf-8")  # 加上 C 字符串的 \0 终止符
                outf.write(arg_bytes)  # 参数值

    print(f"Script encoded and saved to {output_file}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Generate script for OS batch loader.")
    parser.add_argument("--infile", required=True, help="Path to the script file.")
    parser.add_argument(
        "--outfile", required=True, help="Output file for the generate script."
    )
    args = parser.parse_args()

    encode_script(args.infile, args.outfile)
