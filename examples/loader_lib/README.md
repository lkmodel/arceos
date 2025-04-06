# Lib Loader (`loader_lib`) 文档

**状态:** 核心组件，持续开发中

## 1. 概述

本文档详细说明了 `loader_lib` 组件，该组件负责在 ArceOS 环境中加载和管理应用程序及共享库 (`Lib`)。
它旨在为贡献者或希望理解加载器架构、开发进度、设计决策和已知问题的开发者提供指导。

**项目目标（阶段性）：**

1. ~~支持基于 musl 静态链接的单应用（应用源码无需修改）。~~
2. ~~支持基于 musl 动态链接的单应用（原始二进制应用无需修改）。~~
3. ~~通过支持 `fork` 和多地址空间，支持多应用启动。~~
4. ~~支持 `procfs`/`sysfs` 等文件系统，运行 `BusyBox`、`LTP` 等，扩大系统调用覆盖。~~
5. 支持从 musl 到 gcc 的应用编译工具链。

## 2. 术语约定

为确保清晰，我们约定以下术语：

* **应用包 (Application Package):** 一个包含内核所需所有组件的单一二进制文件 (`apps.bin`)：库、执行脚本和应用程序代码。（详见 `../../batch_apps/README.md`）。
* **`Kernel[代码/空间] (Kernel [Code/Space])`:** 指 `ArceOS` 核心代码及其内存地址空间。尽管 `ArceOS` 是一个 `Unikernel`，我们仍使用“`Kernel`”指代其核心部分。
* **`App[区/代码/空间] (App [Area/Code/Space])`:** 指应用包内特定于应用的区域、应用代码及其执行时的内存地址空间。
* **`Lib[区/代码/空间] (Lib [Area/Code/Space])`:** 指应用包内共享库的区域、库代码（例如 `mocklibc`）及其执行时的内存地址空间。
* **`ABI CALL`:** 一种函数调用机制，其中 `Lib` 代码通过 `abi_entry` 跳转执行在 **`Kernel` 代码** 中实现的函数。这些函数随 `Kernel` 一同捆绑和发布。
* **`ABI-SYS CALL (SYS CALL)`:** 一种特殊的 `ABI CALL`，在调用时传递系统调用号和参数，以模拟传统内核的系统调用。`Kernel` 根据系统调用号将这些调用分派给特定的处理程序。它们像 `ABI CALL` 一样随 `Kernel` 发布，但提供了更标准化的接口。在上下文清晰时，可互换使用 `SYS CALL`。（请注意区分，Unikernel 中没有传统意义上的陷入式系统调用）。
* **`Unikernel Mode(Uni Mode)`**: 在 `Cargo.toml` 中可以选择的内核模式。在此模式下，假定应用包中只有一个应用，且符合 `Unikernel` 模型。
* **`Batch Mode`**: 在 `Cargo.toml` 中可以选择的内核模式。在此模式下，假定应用包中有多个应用，每个应用都符合 `Unikernel` 模型。**这是目前开发最完善、建议优先使用的模式。**
* **`pseudo_multi_process mode(PMP mode)`**: 伪多进程模式（开发中）。在此模式下，假定应用包中有一个多进程模式的应用。此模式仍在开发中，理想目标是将其与 `Batch Mode` 融合，通过加载应用包中其他应用来实现 `fork` 等功能。
* **`Compile Runtime Function(CRT)`**: 提供给 `App` 或 `Lib` 的函数，用于实现 musl 库中因编译选项（如 `-nostdlib`）而被屏蔽或缺失的运行时函数。

## 3. 设计思路与实现考量

### 3.1. 核心方法：迁移并修改 musl

本项目选择基于 `musl libc 1.2.2` 库进行源码迁移，并修改适配 ArceOS（此为“思路一”）。主要修改包括：

* 将 musl 中的**系统调用**替换为 **`ABI-SYS CALL`**。
* 将部分 musl 函数（尤其是无法直接运行或包含禁止指令的）替换为 **`ABI CALL`**。

**选择此方法的原因：**

* **简化测试:** 只需重点测试被修改的代码和 `ABI CALL`/`ABI-SYS CALL` 接口，大部分未修改的 musl 代码可信赖其正确性。
* **开发效率:** 相比完全重写 libc（“思路二”），迁移和修改可以更快地获得功能完备的库。
* **安全性:** 复用经过广泛验证的 musl 代码，减少了自行实现引入错误的风险。

**动态链接实现机制：**

当加载动态链接应用时，`loader_lib` 将 `mocklibc.so` (Lib) 和 `app.bin` (App) 从 `apps.bin` 应用包中加载到内存。通过修改 ELF 文件的重定位表（如 PLT），将 App 对库函数的调用指向 Lib 中的实现，将 Lib 可能对 App 函数（如 `main`）的调用重定向，并重定位 Lib 中的 `ABI CALL` 表地址。

### 3.2. 方法权衡 (思路一 vs. 思路二)

**当前方法 (思路一) 的优势：**

* **库的灵活性:** `Kernel`、`Lib`、`App` 代码分离。可以分版本、分需求发布 `mocklibc`（如选择性包含多线程、网络、数学库支持），无需修改 ArceOS Kernel 即可按需加载，平衡内存开销与功能。
* **加载速度:** 无需运行时符号查找算法。
* **ArceOS 本体大小:** Kernel 本身可以更小。

**当前方法 (思路一) 的劣势：**

* **内存占用:** 运行时需要同时加载 App 和 Lib，内存占用相对较高。
* **实现复杂度:** 加载和重定位过程相对复杂。
* **部署流程:** 需要额外的打包步骤将 App 和 Lib 打包成 `apps.bin`。

(思路二：完全重新实现兼容 musl 的库。优点是内存占用小、实现可能更简单；缺点是工作量巨大、测试验证复杂、库与内核实现紧耦合。)

### 3.3. 处理 musl 中的汇编代码

在迁移 musl 中使用汇编（`.S` 文件或内联汇编）的代码时，必须特别小心：

1. **陷阱风险:** 汇编代码常包含特定于架构的陷阱指令（如 RISC-V 上的 `ecall`），这是禁止的。
2. **环境假设:** 汇编代码可能依赖于特定的 CPU 状态、内存布局或 ArceOS 未必提供的操作系统行为。

鉴于这些风险，当遇到 musl 中复杂或可能存在问题的汇编代码时，**强烈建议在 `mocklibc` 中使用 `ABI CALL` 来实现相应功能**，而不是试图直接、可能不安全地迁移汇编代码。

### 3.4. 评估库质量与安全性

虽然通常信任来自 musl 的未修改代码，但 `mocklibc` 中引入的偏差需要评估。

* **建议:** 使用 `diff` 命令比较 `mocklibc` 源码树与原始 `musl-1.2.2` 源码树。
* **度量标准:** **不相似文件的数量**可作为偏差程度的粗略指标。数量越少，表明修改越少，相对于原始 musl 的潜在安全性可能更高。（使用*比例*可能会产生误导性的乐观估计）。鼓励定期进行 `diff` 检查。

## 4. 项目背景与相关组件

`loader_lib` 与 ArceOS 项目的其他几个部分集成：

* `../../batch_apps/` (**已冻结**): 处理批处理模式的应用打包。详见其 `README.md`。
* `../../examples/loader_lib/` (**当前目录**): 包含 `loader_lib` 的核心源代码。
* `../../mockc_apps/` (**待修订**): 一个用于 "uni" 加载模式的简单编译/打包脚本。计划修订并重命名为 `uni_apps`。详见其 `README.md`。
* `../../mocksrc/` (**开发中**): 规划中未来存放所有源码编译和链接脚本的位置。详见其 `README.md`。
* `../../ulib/mocklibc_lib/` (**Lib库源码**): 我们修改的 `mocklibc` 库，用于加载基于 musl 的动态链接应用。详见其 `README.md`。

## 5. `loader_lib` 内部结构

`./src/` 目录下的关键子目录：

* **`abi/`**: 实现 `ABI CALL` 和 `ABI-SYS CALL` 机制。
  * `mem.rs`: 通过 `ABI CALL` 实现手动内存分配函数（malloc 系列）。长期目标：在不会引起内存泄露的情况下，用 `ABI-SYS CALL` 替换内存分配函数。
  * `thread.rs`: 通过 `ABI CALL` 实现线程管理函数。*（注意：需要在进入应用前设置 `tp` 寄存器 - 见已知问题）*。短期目标：用 `ABI-SYS CALL` 替换线程函数族。
  * `syscall.rs`: 实现 `ABI-SYS CALL` 的内核端实现与系统调用号分发。
  * `noimpl.rs`: 未实现的 `ABI CALL` 的占位符、主动占用符。
  * `mod.rs`: 管理 `ABI CALL` 编号分配、注册和分发。未注册的调用默认指向 `abi_noimpl`。这里还实现了 `CRT` 函数。
* **`elf_load/`**: 包含所有 ELF 文件加载和执行逻辑。
  * `batch/`: 实现 `Batch Mode` 的加载与运行。*（此结构应被其他模式复用）*。
  * `uni_load.rs`: (**待重构**) 当前 `Uni Mode` 的加载器。应移入新的 `uni/` 子目录。
  * `load.rs`: (**待重构**) 当前 `PMP Mode` 的加载器。结构需重组； `PMP Mode` 仍在积极开发中。
  * `decoder.rs`: 解码应用包头和脚本的通用函数（目前仅 `Batch Mode` 使用）。
  * `auxv.rs`: (**待重构**) 实现辅助向量 (auxiliary vector) 设置，目前与 `PMP Mode` 相关；目录结构需审视。
  * `verify.rs`: ELF 格式校验代码。
* **`linux_env/`**: 模拟部分 Linux 环境，并为核心 ArceOS 模块提供扩展。目标：最小化对核心模块本身的修改。
* **`syscall/`**: 实现处理 `ABI-SYS CALL` 请求的 **服务端**（内核侧）逻辑。
* **`config.rs`**: 定义 `loader_lib` 使用的常量和配置值。
* **`init.rs`**: (**待重构**) 包含 `uni` 和 `batch` 模式的初始化逻辑；结构应优化。
* **`main.rs`**: 核心逻辑入口点，协调加载过程。

## 6. 构建配置

构建选项和特性可通过 `Cargo.toml` 进行配置。请参阅 `Cargo.toml` 文件中的注释以了解可用的特性标志 (feature flags) 详情。

## 7. 内存布局 (概念性)

| 地址范围 | 描述|
|--------------- | --------------- |
| 0xFFFF_FFC0_8000_0000 |<- SBI 保留内存起始|
| ... | SBI 区域  |
| 0xFFFF_FFC0_8010_0000 | <- SBI 结束 / Lib 区域起始|
| ... |  Lib 代码/数据加载于此 (复用 SBI 间隙)|
| 0xFFFF_FFC0_8020_0000 | <- Lib 区域结束 / Kernel 区域起始 |
| ... | Kernel 代码/数据位于此 |
| 0xFFFF_FFC0_8060_0000 (?)  | <- Kernel 区域结束 / App 区域起始 (地址可配置) |
| ... | App 代码/数据加载于此 |
| 0xFFFF_FFC0_8080_0000 (?)  | <- App 区域结束 (地址可配置) |

*(注意: 确切的 Kernel/App 边界可能因配置而异。)*

## 8. 当前状态与支持特性

* **模式支持:** `Batch Mode` 是目前功能最完善、最稳定的模式，建议优先使用。`Uni Mode` 和 `PMP Mode` 仍在开发和重构中。
* **应用兼容性:** 当前可运行简单的 `ash` 终端以及 `BusyBox` 中大多数不依赖高级进程 (`fork`)、信号处理的应用。

## 9. 已知问题与重要说明 (保留错误)

* **`tp` 寄存器要求:** `musl` libc 假定 `tp` 寄存器指向当前线程的 `pthread_t` 结构体（即 `pthread_self()` 的返回值）。`mocklibc` 目前通过将直接访问 `tp` 寄存器的代码替换为调用 `pthread_self` 实现的 `ABI CALL` 来绕过此问题。**然而，加载器在进入应用程序之前，并未正确设置 `tp` 寄存器。** 这意味着 `Lib` 内部任何直接访问 `tp` 的代码或依赖 `tp` 的线程局部存储 (TLS) 实现都可能失败。这需要在加载器的上下文设置阶段解决。
* **`abi_entry` 边界检查:** 将 `abi_entry` 调用表传递给 Lib 的机制可能允许 Lib 请求一个超出表实际大小的调用号。应在 `abi_entry` 分发逻辑（可能在 `abi/mod.rs`?）中实现安全检查，以防止越界访问，可以默认调用 `abi_noimpl` 或安全地触发 panic。

## 10. 构建与运行指南

1. **克隆仓库:**

    ```bash
    git clone https://github.com/inchinaxiaofeng/arceos.git # 或核心仓库 lkmodel/arceos
    cd arceos/
    ```

2. **编译 `mocklibc` (如果需要更新):**

    ```bash
    cd ulib/mocklibc/
    make
    cd ../../
    ```

3. **打包应用:**
    * **Uni Mode (示例):**

        ```bash
        cd mockc_apps/ # 之后会更名为 uni_apps
        make SRC=<app_name> TYPE=<dynamic|static> # 例如: make SRC=sqlite3 TYPE=dynamic
        cd ..
        ```

    * **Batch Mode:**

        ```bash
        cd batch_apps/
        make
        cd ..
        ```

4. **运行 `loader_lib`:**

    ```bash
    # 默认参数: -l warn -q y
    # 运行复杂应用建议参数:
    ./loader_lib.sh -l off -q n
    # 可选日志级别: -l <debug|warn|info|off|trace>
    # 是否使用 QEMU 图形界面: -q <y|n>
    ```

## 11. 代码改进建议

* 目录结构重构 (如 README 所述):
  * 建议操作: 创建 src/elf_load/uni/ 和 src/elf_load/pmp/ 目录。
  * 建议操作: 将 src/elf_load/uni_load.rs 的逻辑移入新的 src/elf_load/uni/ 模块（例如，src/elf_load/uni/load.rs 并更新 src/elf_load/uni/mod.rs）。
  * 建议操作: 将 src/elf_load/load.rs (PMP 加载器) 的逻辑移入新的 src/elf_load/pmp/ 模块。
  * 建议操作: 将 src/elf_load/auxv.rs 移入 src/elf_load/pmp/ 或一个更合适的公共位置（如果未来其他模式也使用）。
  * 建议操作: 分析 src/init.rs。将不同模式（uni, batch）的初始化逻辑拆分到它们各自的模块中（例如 src/elf_load/batch/init.rs, src/elf_load/uni/init.rs），或者拆分成函数，由 init.rs 根据检测到的模式调用。
  * 理由: 改善代码组织，使职责更清晰，并使结构与 README 中描述的预期设计保持一致。
* 解决已知问题:
  * tp 寄存器:
    * 建议操作: 在负责设置进入应用程序入口点之前的初始上下文的代码中，添加必要的汇编或函数调用，为主线程正确初始化 tp 寄存器。
    * 建议操作: 在 src/abi/thread.rs 以及靠近跳转到应用程序入口点的代码处，添加显著的注释（// FIXME:, // TODO:, 或特定警告），解释 tp 寄存器的依赖性以及加载器需要正确设置它。
    * 理由: 修复 musl/mocklibc 线程模型所依赖的关键正确性问题。
  * abi_entry 边界检查:
    * 建议操作: 定位 ABI CALL 的分发逻辑（可能在 src/abi/mod.rs 或从 abi_entry 调用的函数中）。在使用提供的调用号索引 ABI 函数表/切片之前，添加检查。
    * 理由: 防止因越界访问可能导致的崩溃或内存损坏，提高安全性和稳定性。
* 提高可读性与注释:
  * 建议操作: 审阅复杂模块，如 elf_load（特别是 verify.rs, decoder.rs, 以及特定模式的加载器）和 syscall 处理程序。添加模块级 (//!) 和函数级 (///) 文档注释，解释其目的、高层逻辑和任何不明显的假设。
  * 建议操作: 为复杂的逻辑步骤、unsafe 代码块或权宜之计 (workaround) 的实现添加内联注释 (//)。
  * 建议操作: 确保函数、变量和模块使用一致的命名约定。
  * 理由: 使代码更容易被其他开发者（以及未来的你）理解、维护和调试。
* 错误处理:
  * 建议操作: 查找代码中 .unwrap() 和 .expect() 的使用。评估这些地方是否代表真正不可恢复的错误，或者使用 Result 传播（? 操作符）或更具体的错误处理（例如 match）是否会更健壮。特别注意文件 I/O、内存分配/映射以及 ELF 解析部分。
  * 理由: 通过防止意外 panic 并在可能的情况下允许更优雅的错误报告或恢复来提高代码的健壮性。
* 标记未完成的工作:
  * 建议操作: 确保文档中提到的正在开发中的部分（PMP 模式、uni_load.rs 重构、abi/thread.rs 中潜在的 ABI-SYS CALL 替换）在代码中用 // TODO:, // FIXME:, 或 unimplemented!() 宏清晰标记。
  * 理由: 清晰地传达开发状态和剩余任务。

## 12. 下一步开发方向建议

* **完善系统调用 & ABI 替换:**
  * 继续基于应用加载试验，从 `musl-1.2.2` 迁移源码以完善 `mocklibc` 功能。
  * 将更多 `ABI CALL` 实现（尤其是 `pthread`, `malloc` 相关）替换为基于 `syscall` 的 `ABI-SYS CALL`。
* **完善 Compile Runtime (CRT) 函数:**
  * 继续拓展 `CRT` 函数（400+），并自行实现 Crates 中目前不支持的运行时函数。
* **代码质量与框架完善:**
  * 完善代码框架，减少 `unwrap`、`expect`、`unsafe` 等。
  * 检查 `mocklibc` 源码，尽量保持与 `musl-1.2.2` 的一致性（`diff` 检查），明确记录必要修改。
* **功能增强与灵活性:**
  * **Lib 版本与模块化管理:**
    * 支持根据应用脚本加载指定版本和配置的 `mocklibc` 库。
    * 支持应用包中存在多个 `Lib` 库。
    * 将 `mocklibc` 分模块打包发布，按需加载（类似 ArceOS 组件化）。
  * **PMP 模式改进:** 探索基于 `hypervisor` 支持实现更健壮的进程与信号功能 (`hyper-process`/`hyper-signal`?)，克服当前 PMP 模式的限制（低效且不安全）。
  * **工具链支持:** 在完善 musl 支持基础上，逐步添加对 `GCC` 工具链编译应用的支持。
* **项目结构调整:**
  * 将 `batch mode` 的参数传递等完善功能迁移到 `uni mode`。
  * 将 `mockc_apps` 目录更名为 `uni_apps`，专门负责 `uni mode` 打包。
  * 为 `PMP` 模式创建新的打包脚本目录。
  * 创建新的目录 (`mocksrc`?) 统一存放源码编译和链接脚本。
* **实施策略讨论:** 探讨复杂功能（如 `pthread`, `malloc`）是在 `mocklibc` (Lib) 中基于 musl 实现更优，还是在 `loader_lib` (Rust/Kernel) 中实现 `ABI CALL`/`ABI-SYS CALL` 更合适。

## 13. 其他资源与说明

* **文档状态:** 本项目文档仍在完善中。最新版本请关注开发者 fork 的仓库及提交的 Pull Requests。
* **开发者手记/博客:** <https://inchinaxiaofeng.github.io/>
* **工作内容周报:** <https://docs.qq.com/sheet/DTmNIeXFzanlObXN1?tab=BB08J2>
* **核心代码仓库:** <https://github.com/lkmodel/arceos/tree/mocklibc_libloader> (注意合并可能延迟)
* **开发者 Fork (可能更新):** <https://github.com/inchinaxiaofeng/arceos/tree/mocklibc_libloader>
