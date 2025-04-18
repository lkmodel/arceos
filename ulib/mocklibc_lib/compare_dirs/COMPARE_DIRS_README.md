# 📂 compare_dirs.py

Compares file differences between two directories, supporting:

- Excluding specified directories
- Controlling the maximum comparison depth
- Showing detailed differences (`--verbose`)
- Automatically saving reports as `.txt` and `.json`

---

## 📦 Usage

```bash
python3 compare_dirs.py \
  [--dir1 path1] \
  [--dir2 path2] \
  [--exclude subdirectory1 subdirectory2 ...] \
  [--depth max_depth] \
  [--verbose]
```

## 🆕 Parameter Details

| Parameter | Description |
| --- | --- |
| --dir1 | The first directory to compare, default ../c/ |
| --dir2 |  The second directory to compare, default /home/.../src/ |
| --exclude | List of subdirectories to exclude (default values are preset) |
| --depth | Maximum comparison depth, default 2 |
| --verbose | Output all different and unique filenames when enabled |

## 💾 Automatic Report Generation

- Text Report: compare_report.txt

- JSON Report: compare_report.json
