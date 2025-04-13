import os
import argparse
import hashlib
import json
from pathlib import Path


def parse_args():
    parser = argparse.ArgumentParser(description="比较两个目录的文件内容差异")
    parser.add_argument(
        "--dir1", type=str, default="./c/", help="第一个目录（默认: ./c/）"
    )
    parser.add_argument(
        "--dir2",
        type=str,
        default="/home/marinatoo/App/musl-1.2.2/src/",
        help="第二个目录（默认: /home/marinatoo/App/musl-1.2.2/src/）",
    )
    parser.add_argument(
        "--exclude",
        type=str,
        nargs="*",
        default=[
            "thread",
            "thread_riscv64",
            "network",
            "process",
            "math",
            "malloc",
            "signal",
            "safe_malloc",
            "internal",
            "include",
        ],
        help="需要排除的子目录（默认已配置）",
    )
    parser.add_argument(
        "--depth", type=int, default=2, help="控制比较的最大子目录深度（默认: 2）"
    )
    parser.add_argument("--verbose", action="store_true", help="打印不同或独有的文件名")
    return parser.parse_args()


def hash_file(path):
    h = hashlib.sha256()
    try:
        with open(path, "rb") as f:
            for chunk in iter(lambda: f.read(4096), b""):
                h.update(chunk)
        return h.hexdigest()
    except Exception:
        return None


def collect_files(base_dir, exclude_dirs, max_depth):
    file_map = {}
    base_path = Path(base_dir).resolve()

    for root, dirs, files in os.walk(base_path):
        rel_path = Path(root).relative_to(base_path)
        depth = len(rel_path.parts)

        if depth >= max_depth:
            dirs[:] = []
            continue

        dirs[:] = [d for d in dirs if d not in exclude_dirs]

        for file in files:
            abs_path = os.path.join(root, file)
            rel_file_path = os.path.relpath(abs_path, base_dir)
            file_map[rel_file_path] = abs_path

    return file_map


def get_total_size(file_paths):
    try:
        return sum(os.path.getsize(p) for p in file_paths)
    except Exception:
        return -1


def format_size(size):
    if size < 0:
        return "读取失败"
    return f"{size / 1024 / 1024:.2f} MB"


def save_reports(summary, different_files, only1, only2):
    with open("compare_report.txt", "w", encoding="utf-8") as f:
        for line in summary:
            f.write(line + "\n")
        if different_files:
            f.write("\n内容不同的文件：\n")
            f.writelines(f"  {f}\n" for f in different_files)
        if only1:
            f.write("\n目录1 独有文件：\n")
            f.writelines(f"  {f}\n" for f in only1)
        if only2:
            f.write("\n目录2 独有文件：\n")
            f.writelines(f"  {f}\n" for f in only2)

    report_json = {
        "summary": summary,
        "diff_files": different_files,
        "only_in_dir1": only1,
        "only_in_dir2": only2,
    }
    with open("compare_report.json", "w", encoding="utf-8") as f:
        json.dump(report_json, f, ensure_ascii=False, indent=2)


def main():
    args = parse_args()

    dir1 = args.dir1
    dir2 = args.dir2
    exclude_dirs = args.exclude
    max_depth = args.depth
    verbose = args.verbose

    files1 = collect_files(dir1, exclude_dirs, max_depth)
    files2 = collect_files(dir2, exclude_dirs, max_depth)

    all_keys = set(files1.keys()).union(files2.keys())
    common_keys = set(files1.keys()).intersection(files2.keys())

    same_content = 0
    diff_content = 0
    different_files = []
    only1 = []
    only2 = []

    for key in all_keys:
        if key in files1 and key in files2:
            hash1 = hash_file(files1[key])
            hash2 = hash_file(files2[key])
            if hash1 and hash2 and hash1 == hash2:
                same_content += 1
            else:
                diff_content += 1
                different_files.append(key)
        elif key in files1:
            only1.append(key)
        elif key in files2:
            only2.append(key)

    total_compared = len(common_keys) + len(only1) + len(only2)
    same_ratio = same_content / total_compared * 100 if total_compared else 0
    diff_ratio = diff_content / total_compared * 100 if total_compared else 0
    only1_ratio = len(only1) / total_compared * 100 if total_compared else 0
    only2_ratio = len(only2) / total_compared * 100 if total_compared else 0

    summary = [
        f"比较目录: {dir1} vs {dir2}",
        "-----------------------------------------",
        f"目录1 文件总数: {len(files1)}",
        f"目录1 总大小: {format_size(get_total_size(files1.values()))}",
        f"目录2 文件总数: {len(files2)}",
        f"目录2 总大小: {format_size(get_total_size(files2.values()))}",
        "-----------------------------------------",
        f"相同文件名文件数: {len(common_keys)}",
        f"内容完全相同文件数: {same_content}",
        f"内容不同的同名文件数: {diff_content}",
        f"目录1 独有文件数: {len(only1)}",
        f"目录2 独有文件数: {len(only2)}",
        "-----------------------------------------",
        f"相同内容文件比例 (相对于比较的文件总数): {same_ratio:.2f}%",
        f"不同内容文件比例 (相对于比较的文件总数): {diff_ratio:.2f}%",
        f"目录1 独有文件比例 (相对于比较的文件总数): {only1_ratio:.2f}%",
        f"目录2 独有文件比例 (相对于比较的文件总数): {only2_ratio:.2f}%",
    ]

    for line in summary:
        print(line)

    if verbose:
        if different_files:
            print("\n内容不同的文件：")
            for f in different_files:
                print(f"  {f}")
        if only1:
            print("\n目录1 独有文件：")
            for f in only1:
                print(f"  {f}")
        if only2:
            print("\n目录2 独有文件：")
            for f in only2:
                print(f"  {f}")

    save_reports(summary, different_files, only1, only2)


if __name__ == "__main__":
    main()
