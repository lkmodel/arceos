
# 📂 compare_dirs.py

对比两个目录中的文件差异，支持：

- 排除指定目录
- 控制最大比较深度
- 显示详细差异（`--verbose`）
- 自动保存报告为 `.txt` 和 `.json`

---

## 📦 用法

```bash
python3 compare_dirs.py \
  [--dir1 路径1] \
  [--dir2 路径2] \
  [--exclude 子目录1 子目录2 ...] \
  [--depth 最大深度] \
  [--verbose]
```

## 🆕 参数详解

| 参数 | 说明 |
| --- | --- |
| --dir1 | 第一个比较目录，默认 ./c/ |
| --dir2 | 第二个比较目录，默认 /home/.../src/ |
| --exclude | 排除的子目录列表（默认值已预设） |
| --depth | 最大比较深度，默认 2 |
| --verbose | 启用后输出所有不同和独有的文件名 |

## 💾 自动报告生成

* 文本报告：compare_report.txt

* JSON 报告：compare_report.json
