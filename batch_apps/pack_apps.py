import os
import argparse


def pack_apps(header, apps_dir, lib_path, script_path, output_file):
    with open(output_file, "wb") as output:
        # 1. 写入头部
        print(f"Writing header from {header}")
        with open(header, "rb") as header_file:
            output.write(header_file.read())

        # 2. 写入应用程序
        print(f"Writing apps from {apps_dir}")
        for app_filename in sorted(os.listdir(apps_dir)):
            app_path = os.path.join(apps_dir, app_filename)
            if os.path.isfile(app_path):
                print(f"Packing app: {app_path}")
                with open(app_path, "rb") as app_file:
                    output.write(app_file.read())

        # 3. 写入 Lib 库
        print(f"Writing lib from {lib_path}")
        with open(lib_path, "rb") as lib_file:
            output.write(lib_file.read())

        # 4. 写入编码后的执行脚本
        print(f"Writing encoded script from {script_path}")
        with open(script_path, "rb") as script_file:
            output.write(script_file.read())

    print(f"Final packed binary written to {output_file}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Packing Apps, Lib and script for OS batch loader."
    )
    parser.add_argument("--header_file", required=True, help="File of header part.")
    parser.add_argument(
        "--apps_dir",
        required=True,
        help="Directory containing application binary files.",
    )
    parser.add_argument("--lib_file", required=True, help="File of lib part.")
    parser.add_argument("--script_file", required=True, help="File of encoded script.")
    parser.add_argument(
        "--outfile", required=True, help="Output File for the packed file."
    )

    args = parser.parse_args()

    pack_apps(
        args.header_file, args.apps_dir, args.lib_file, args.script_file, args.outfile
    )
