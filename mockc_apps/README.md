# mockc_apps - Uni Mode 应用打包器 (待修订)

**状态:** 待修订 (Needs Revision) - 计划重命名为 `uni_apps`

## 概述

`mockc_apps` 包含一套**简单的**构建和打包脚本，主要用于处理 **`Uni Mode`** (Unikernel 模式) 下的单个应用程序打包。

其目的是将一个预编译的应用程序（静态或动态链接到 `mocklibc`）打包成一个可供 `loader_lib` 在 `Uni Mode` 下加载和执行的基础应用包。

**注意:** 此目录下的脚本功能相对基础，且计划进行修订并整合到新的 `uni_apps` 目录中。目前功能最完善的打包工具请参见 `../../batch_apps/` (用于 `Batch Mode`)。

## 使用方法

在 `mockc_apps` 目录下执行 `make` 命令进行打包。

```bash
# 进入 mockc_apps 目录
cd mockc_apps

# 执行打包
# SRC: 指定要打包的应用名称 (对应 APPS/ 目录下的文件名)
# TYPE: 指定链接类型 (dynamic 或 static)
make SRC=<app_name> TYPE=<dynamic|static>

# 示例: 打包名为 'sqlite3' 的动态链接应用
make SRC=sqlite3 TYPE=dynamic

# 返回上级目录
cd ..
```

打包过程通常包括:

1. 查找位于 APPS/ 目录下的指定应用文件 (<app_name>)。
2. 根据 TYPE 参数决定是否需要包含 mocklibc 动态库 (.so) 或链接静态库 (.a) (具体链接逻辑可能在此脚本或应用编译阶段处理)。
3. 生成一个简单的应用包结构，供 loader_lib 在 Uni Mode 下使用。

## 依赖

* 预编译的应用程序可执行文件，放置于 APPS/ 目录下。
* 如果打包动态链接应用 (TYPE=dynamic)，需要 ulib/mocklibc_lib/lib/libmock.so 文件存在于预期的相对路径。
* 如果应用是静态链接到 mocklibc (TYPE=static)，则需要在应用编译时已正确链接 ulib/mocklibc_lib/lib/libmock.a。

## 未来计划

* 将此目录重命名为 uni_apps。
* 修订和完善打包脚本，可能借鉴 batch_apps 中的成熟功能（如参数传递、更灵活的配置）。
* 与 mocksrc 目录（负责源码编译）进行更好的整合。
