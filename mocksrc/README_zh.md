# mocksrc - 源码编译与链接中心 (开发中)

**其他语言版本: [English](./README.md), [中文](./README_zh.md).**

**状态:** 开发中 (Under Development)

## 概述

`mocksrc` 目录是 ArceOS 项目中**统一存放应用程序源码编译和链接脚本**的核心位置。

它提供了一套工具和脚本，允许开发者直接从应用程序的源代码出发，**编译、链接**生成适用于 ArceOS `loader_lib` 环境的可执行文件。这些可执行文件可以静态或动态链接到 `mocklibc`。

`mocksrc` 与 `batch_apps` 和 `uni_apps`（原 `mockc_apps`）目录形成清晰的分工：

* **`mocksrc`**: 负责应用程序源代码的**编译和链接**，生成可执行文件。
* **`batch_apps` 和 `uni_apps`**: 负责将 `mocksrc` 生成的可执行文件**打包**成 `apps.bin` 应用包，以供 `loader_lib` 加载。

## 目录结构

* `link.ld`: 基础的 RISC-V 链接脚本，用于控制应用程序的内存布局。
* `Makefile`: 用于自动化编译和链接过程的构建脚本。
* `config.mk`: 配置文件，用于定义工具链路径（目前主要支持 musl）、通用编译选项等全局设置。

**注意:** `mocksrc` 目录下的 `apps/` 子目录以及其他子目录和文件，目前主要用于存放之前的测试用例，并非当前构建流程的必要组成部分。应用程序的编译和链接通常在 `mocksrc` 根目录下通过 `Makefile` 完成。

## 使用方法

在 `mocksrc` 目录下，你可以使用 `make` 命令来编译和链接应用程序。你需要指定以下参数：

* **`BATCH_APPS`**: 用于指定编译生成的可执行文件将要迁移到的 `batch_apps` 目录中待打包程序的位置。
* **`UNI_APPS`**: 用于指定编译生成的可执行文件将要迁移到的 `uni_apps` 目录中待打包程序的位置。
* **`OUTFILE`**: 指定输出的可执行文件的名称（不包含扩展名）。
* **`DIR`**: 指定当前目录下包含应用程序源代码的子目录的名称。
* **`TYPE`**: 指定编译类型，可以是 `static`（静态链接到 `mocklibc`）或 `dynamic`（动态链接到 `mocklibc`，如果支持）。

**基本编译命令示例:**

```bash
make TYPE=<static|dynamic> DIR=<app_source_dir> OUTFILE=<output_name>
```

## 具体示例

编译名为 `sqlite3_speedtest1` 的应用程序，静态链接到 `mocklibc`，并将输出文件命名为 `sqlite3_speedtest1`，最终将编译好的可执行文件复制到 `batch_apps` 和 `uni_apps` 目录：

```bash
make TYPE=static DIR=sqlite3_speedtest1 OUTFILE=sqlite3_speedtest1
```

### 注意

* 请确保在执行 `make` 命令之前，已经正确配置了根目录下的 `config.mk` 文件，特别是 `RISCV_PREFIX` 等变量需要指向你的 `musl` 工具链的正确路径。
* `DIR` 参数指定的是 `mocksrc` 目录下的一个子目录，该子目录应包含应用程序的源代码和相应的构建脚本（通常是 Makefile）。

## 与其他组件的关系

* 依赖于:
  * ulib/mocklibc_lib: 提供 mocklibc 库和头文件，应用程序需要链接到它才能使用 C 标准库功能。
  * RISC-V 工具链 (例如 riscv64-unknown-elf-gcc, riscv64-unknown-elf-ld 等): 用于编译和链接源代码，当前主要支持 musl 工具链。
  * config.mk: 提供全局的构建配置信息。
* 服务于:
  * batch_apps: 提供编译好的、适用于 Batch Mode 的可执行文件，这些文件会被打包到 apps.bin 中。
  * uni_apps: 提供编译好的、适用于 Uni Mode 的可执行文件，这些文件会被打包到 apps.bin 中。
目标环境:
  * examples/loader_lib: 编译出的应用程序最终由 loader_lib 加载并在 ArceOS 上执行。
