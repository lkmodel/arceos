# ArceOS 支持 ELF 文件运行

项目链接：[https://github.com/lkmodel/arceos](https://github.com/lkmodel/arceos)

## 项目安排

### 整体目标

目标：在ArceOS Unikernel形态下，支持加载和运行原始的Linux应用。通过把系统调用转为函数调用，提升系统的整体运行效率；代价是降低了安全性。

核心问题：Linux应用假定自己将运行在Linux环境中，它本身以及依赖库以及工具链都遵循这一假定；现在我们要把应用放到ArceOS之上运行，让应用觉察不到自己的运行环境变化了；所以就需要ArceOS制造出这么一种运行环境。采取的方案是保持libc接口兼容前提下替换libc的实现。

![libc](./doc/figures/libc.excalidraw.png)

### 两种实现思路

![mocklibc](./doc/figures/mocklibc.png)

办法一特点：
+ 不需要单独编写C侧lib函数，只需要对应用使用到的标准库函数进行编写注册即可
+ 实际上的lib位于内核中，在加载动态链接应用时被重定向以供使用
+ 应用加载链接时一次完成，后续调用函数时均无类似syscall的查表环节
+ 可以做到较大一部分在Rust侧编写函数，充分利用Rust特性

方法一缺点：
+ 因为模拟libc的层次在标准库函数层，并不像方法二一样通过重写兼容musl的库的方法。所以需要对**所有**应用需要的标准库函数进行重写或改写，特别是包括各种函数的变体（例如prinf的各种变体）。可能需要耗费更多的开发时间。
+ 对于编译器编译过程中增加的`hidden`函数（如软件浮点数计算函数等）暂时无能为力。
### 后续工作总体规划

1. 完成规划的全部5个阶段，实现Unikernel模式下直接运行Linux的原始应用，通过系统调用转函数调用，达到提升效率的目标。
   + ~~阶段1：支持基于musl静态链接的单应用。应用虽然需要重新编译和链接，但是源码不需要修改。~~
   + ~~阶段2：支持基于musl动态链接的单应用。原始的二进制应用不需要修改，能够直接运行。~~
   + ~~阶段3：支持基于多地址空间从而支持多应用。通过支持`fork`，可以启动其它进程。~~
   + 阶段4：支持`procfs`和`sysfs`等文件系统。通过支持BusyBox、LTP等测试用例，扩大系统调用支持范围。(目前工作位于此处)
   + 阶段5：支持编译应用的工具链从musl到gcc。扩大对常见Linux应用的支持。
2. 优化构建过程，能够体现出当前的组件化内核构建方法相对传统方法的便捷性。

注：其中核心组件来自ArceOS公共组件，仅增加少量面向本场景组件。

## 整体架构
### 目录结构
```
.
├── Cargo.lock
├── Cargo.toml
├── CONTRIBUTE.md
├── examples
│   ├── loader （lib中间层）
│   └── ├── abi_macro （abi注册宏）
│       │   ├── Cargo.toml
│       │   ├── core
│       │   │   ├── Cargo.toml
│       │   │   ├── src
│       │   │   └── tests
│       │   └── src
│       │       └── lib.rs
│       ├── Cargo.toml
│       └── src
│          ├── abi （对应libc中的头文件）
│          │   ├── env.rs
│          │   ├── errno.rs
│          │   ├── exit.rs
│          │   ├── fcntl.rs
│          │   ├── fenv.rs
│          │   ├── init.rs
│          │   ├── mem.rs
│          │   ├── mod.rs
│          │   ├── process.rs
│          │   ├── setjmp.rs
│          │   ├── string.rs
│          │   ├── thread.rs
│          │   ├── time.rs
│          │   └── unistd.rs
│          ├── config.rs
│          ├── elf （ELF处理）
│          │   ├── auxv.rs
│          │   ├── elf.rs
│          │   └── mod.rs
│          ├── main.rs
│          └── process （多进程相关）
│              ├── api.rs
│              ├── flags.rs
│              ├── mod.rs
│              ├── process.rs
│              └── task_ext.rs
├── Makefile
├── payload （APP目录）
│   ├── apps.bin
│   └── *_APP 
├── README.md
├── rust-toolchain.toml
└── xtask （xtask目录）
    ├── Cargo.toml
    └── src
        └── main.rs
```

### 主要目录
`examples/loader`是工作主要目录，包括了ELF文件的加载、修改和执行，各类ABI函数，以及ABI函数注册宏等。
`xtask`是辅助运行工具目录，包括了工具链下载，应用编译运行等开发用步骤。
`payload`下存放需要运行的APP，文件夹名通常为`*_APP`，重命名等选项请参考下文`xtask`详细解释。

### 主要函数
主要函数是`load_elf()`，它负责整个ELF文件的加载过程，根据程序类型分为两个加载路径:`load_exec()`和`load_dyn()`，包含辅助函数如`load_segment()`和`modify_plt()`用于具体的加载和修改操作。

#### load_elf() 函数的主要流程

+ 读取ELF文件大小
+ 解析ELF头部
+ 检测是否需要INTERP
+ 根据是否存在INTERP段选择不同的加载方式
+ 返回程序入口点地址

#### PIE检测机制

+ 通过检查程序头(Program Headers)中是否存在PT_INTERP段来判断
+ 如果存在PT_INTERP段,则认为是PIE程序
+ 这影响了后续的加载方式和入口点地址计算

### 两种加载方式

a) 静态加载 (load_exec):

+ 针对静态链接的程序
+ 主要加载.text段到指定内存区域
+ 直接使用ELF头中的入口点地址

b) 动态加载 (load_dyn):

+ 针对动态链接的程序
+ 加载所有PT_LOAD类型的段
+ 需要处理PLT(Procedure Linkage Table)重定位
+ 入口点需要加上基地址偏移

### PLT修改机制

+ 解析动态符号表和字符串表
+ 处理.rela.plt重定位段
+ 将外部函数地址填入PLT表中

### 函数注册机制
增加了独立的`abi-macro`库，使用`#[abi()]`属性宏来注册函数，供动态链接重定向使用
#### 使用方法
1. 在文件顶端加上相关use
2. 在写好的abi函数上方加入`#[abi(函数名)]`即可
```rust
use crate::{AbiEntry, ABI_TABLE};
use abi_macro::abi;

#[abi(hello)]
#[unsafe(no_mangle)]
pub extern "C" fn abi_hello() {
    info!("[ABI:Hello] Hello, Apps!");
}
```
宏展开时会自动为该函数添加静态的AbiEntry，并通过linkme的分布式切片集合到一起，效果如下：
```text
//和Arceos的IRQ与PAGE_FAULT一起放置在.tbss后
ffffffc08022f0e0 g       linkme_ABI_TABLE       0000000000000000 .protected __start_linkme_ABI_TABLE
ffffffc08022f4e8 g       linkme_ABI_TABLE       0000000000000000 .protected __stop_linkme_ABI_TABLE
```

## 编译运行
> [!IMPORTANT]
> 在开发过程中，因原定义的应用运行内存空间不足，暂时地迁移到kernel后，即 0x8020_0000- 0x8100_0000
> 若需修改，可在`modules/axhal/src/mem.rs:163-164`处进行修改
### 尝试运行

``` bash
git clone https://github.com/lkmodel/arceos.git
cd arceos
git switch mocklibc
cargo xtask all
```

> 注：如果所需工具已经安装，您可以跳过此步骤。
> 如果在运行 cargo xtask 时遇到网络问题，或希望手动安装musl-cross-make工具，请按照以下步骤操作：
>
> ```bash
> wget https://musl.cc/riscv64-linux-musl-cross.tgz
> tar zxf riscv64-linux-musl-cross.tgz -C xtask/riscv64-linux-musl-cross
> # export PATH=$PATH:/opt/musl_riscv64/bin
> ```
>
> 安装完成后，您可以通过运行以下命令来验证工具链是否正确安装：
>
> ```bash
> which riscv64-linux-musl-gcc
> ```

> [!IMPORTANT]
> 因为本项目使用了固定版本的`riscv64 musl`工具链（目前是GCC 11.2.1+musl 1.2.2），为了不对可能的已安装的在环境变量中的工具链造成切换上的困扰，目前的`xtask`运行时会将工具链下载到`xtask`目录下，并能正确使用，所以不再需要特别设置环境变量。

### xtask
本项目使用[`cargo xtask`](https://github.com/matklad/cargo-xtask)的方式来简化编译运行步骤，在`.cargo/config.toml`中可以看到
```toml
[alias]
xtask = "run --package xtask --release --"
```

对于`cargo xtask`命令

可以使用`CC=/path/to/gcc`来指定使用的编译器

可用参数:

+ `<APP>` 指定需要运行的APP文件夹名,使用"all"来运行所有应用

可用选项:

+ `--arch <ARCH>`: 目标架构: `x86_64`, `riscv64`, `aarch64`, 目前仅支持`riscv64`。
+ `-l, --log <LOG>`: 日志等级: `warn`, `error`, `info`, `debug`, `trace`, 默认为`warn`。
+ `--qemu-log <QEMU_LOG>`: 是否开启QMEU日志 (日志文件为 "qemu.log"), 默认为`n`。
+ `-t, -ttype <TTYPE>`: 运行测试类型：`static`, `dynamic`, `all`, 默认为`dynamic`。
+ `-s, --snapshot`: 仅审阅编译快照，不运行应用
+ `-b, --blk <BLK>`: 启用QEMU文件系统 (y/n)，默认为`n`
+ `--skip`: 跳过应用编译阶段

也可在应用目录下根据情况进行自行修改`config.toml`

现在支持的所有配置如下

```toml
[dev]
rename = "hello" # 覆盖文件夹名,例如`hello_app`文件夹下的`hello.c`
ttype = "all" # 链接方式。特别的,"all"="static"+"dynamic"
snapshot = true # 是否为该应用开启快照测试
dynamic_flags = [] # 用于动态链接的参数,在开发阶段,默认为所有应用启用了`-fPIE`,在此处填写可以覆盖
static_flags = [] # 用于静态链接的参数
```
## 快照审阅（默认关闭）
为了方便调试与测试，引入了[insta](https://insta.rs/)来存储与比对编译生成的应用。
当在`config.toml`中指定`snapshot=true`后，运行`xtask`编译完成应用后，如果应用的`S` `dump` `elf`发生变化（或本来并没有.snap文件）会在运行前触发审阅。此时有两种选择：
1. 如果明确发生的变化在预期中 <br>
   对于每个snap，按下`a`来接受审阅即可，会自动使用snap.new覆盖原有.snap
2. 如果改动是非预期的，需要进一步比对和修改 <br>
   对于每个snap，按下`s`来暂时跳过审阅来运行应用。xxx_app/snapshot下会出现.snap.new，当确认无误后，可再次运行`cargo xtask xxx_app -s`来审阅快照。
   > cargo_insta的比对是上下排列的，如果需要更好的对比体验，推荐使用[difftastic](https://difftastic.wilfred.me.uk/)或其他工具来获得更好的体验

**注意**：为了健壮的编码与方便他人，如果还有快照尚未审阅就向仓库提交审阅，会被git hooks制止

> [!CAUTION]
> 在开发早期，转储与快照功能对于了解对比应用的结构及汇编变化有较大作用。但当基础部分实现后，不再需要频繁地对比不同编译情况下的应用细微改变。所以目前版本的快照审阅功能默认关闭。

## 协作开发流程

在参与项目协作时，请按照以下步骤进行操作：

1. Fork 和 Clone 仓库 <br>
   首先，Fork [中心仓库](https://github.com/lkmodel/arceos)到自己的 GitHub 账号下，并 Clone 到本地环境。在后续开发中，基于 mocklibc 分支进行协作，所有的 Pull Request（PR）和合并操作都将在该分支上进行。

2. 提出想法并讨论（建议） <br>
   在正式实现前，可以在项目 1 的微信群中提出自己的想法，与其他开发者进行讨论或协商，以确保思路清晰并避免重复开发。

3. 本地实现与测试 <br>
   根据讨论结果，在本地进行功能的开发与实现，并确保经过充分的本地测试，确保代码质量和功能的正确性。

4. 同步中心仓库并解决冲突 <br>
   在提交 PR 之前，确保自己的 Fork 仓库与中心仓库保持同步。可以通过以下步骤实现：
   + 从中心仓库拉取最新代码，并在本地进行 Rebase：

      ``` bash
      git pull --rebase
      ```

   + 如果存在冲突，解决冲突并重新测试代码。

5. 推送到 Fork 仓库并提交 PR <br>
   将修改后的代码推送到自己 Fork 的仓库：

   ``` bash
   git push
   ```

   随后，在GitHub上提交 Pull Request 到中心仓库的 `mocklibc` 分支，并等待代码审查和合并。

确保协作开发的有序性和代码库的一致性。

## 重点工作内容

（1）扩大动态链接应用的支持范围，后面不再单独关注静态应用 <br>
（2）优化内部实现，包括简化代码，提升效率，简化构建脚本等等 <br>
（3）完善CI测试等，尤其支持新特性要先加上测试 <br>
（4）对各种bug的fix <br>
（5）尽量复用arceos的现有组件 <br>
（6）手册与文档的完善 <br>

## TODO

+ [X] 支持启动运行简易的原生基于musl动态链接的Linux应用
+ [ ] 构建一个成熟可用的CI测试框架
+ [ ] 扩大动态链接应用的支持范围
