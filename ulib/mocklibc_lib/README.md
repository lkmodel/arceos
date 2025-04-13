# mocklibc

**Read this in other languages: [English](./README.md), [中文](./README_zh.md).**

## 1. Introduction

**mocklibc** is a C standard library modified based on **musl libc version 1.2.2** (sourced from Ubuntu 22.04 LTS).
It has been specifically adapted for the **ArceOS** Unikernel project, aiming to provide standard C library support for dynamically loaded applications.

Its main goal is to support the development and execution of dynamically linked applications within the ArceOS environment, utilizing familiar C library interfaces while adhering to the constraints of the Unikernel architecture.

## 2. Core Design Principles and Constraints

The development of `mocklibc` follows these key principles:

* **No Trap Instructions (`ecall`):** Applications using `mocklibc` run in Supervisor mode (S mode) within the ArceOS kernel.
Executing the `ecall` instruction would cause an unexpected trap into User mode (U mode), which contradicts the Unikernel execution model.
Therefore, **the code of `mocklibc` must strictly avoid any `ecall` or other trap instructions.**
* **System Call Replacement:** Standard system calls in musl are replaced with specific ArceOS mechanisms:
  ***`ABI-SYS CALL`**: This is the **preferred** mechanism. It delegates the required functionality to the underlying ArceOS kernel or loader through a well-defined Application Binary Interface (ABI).
  (See `../../examples/loader_lib/README.md` for the specific definition). The goal is for `ABI-SYS CALL` to handle the vast majority of system-level operations.
  * **`ABI CALL`**: This mechanism refers to functions implemented directly within `mocklibc` itself, rather than being delegated through `ABI-SYS CALL`.
 The use of `ABI CALL` should be **minimized** and ideally only used for:
    *Compiler runtime support functions (e.g., built-in functions provided by `libgcc`).
    * Implementing functions from musl that heavily rely on assembly code, cannot be safely adapted, or may contain traps (see Section 3.1).
* **Minimize `ABI CALL`, Maximize `ABI-SYS CALL`:** The ideal state is that almost all libc functionality, except for necessary compiler runtime helper functions, is implemented through `ABI-SYS CALL`. This improves portability and maintainability.
* **Library/Kernel Code Separation:** `ABI-SYS CALL` relies on interfaces provided by the kernel/loader. The entire implementation of `ABI CALL` resides within the `Kernel`.
Maintaining this distinction allows `mocklibc` (potentially based on different musl versions) to work with a stable kernel ABI, facilitating future support for multiple musl versions simultaneously.

## 3. Implementation Considerations

### 3.1. Handling Assembly Code in musl

Special care must be taken when migrating code in musl that uses assembly (`.S` files or inline assembly):

1. **Trap Risks:** Assembly code often contains architecture-specific trap instructions (such as `ecall` on RISC-V), which are prohibited.
2. **Environment Assumptions:** Assembly code may rely on specific CPU states, memory layouts, or operating system behaviors that ArceOS may not provide.

Given these risks, when encountering complex or potentially problematic assembly code in musl, **it is strongly recommended to use `ABI CALL` in `mocklibc` to implement the corresponding functionality** instead of attempting to directly and potentially unsafely migrate the assembly code.

### 3.2. Evaluating Library Quality and Security

Although unmodified code from musl is generally trusted, the deviations introduced in `mocklibc` need to be evaluated.

* **Recommendation:** Use the `diff` command to compare the `mocklibc` source tree with the original `musl-1.2.2` source tree.
* **Metric:** The **number of dissimilar files** can serve as a rough indicator of the degree of deviation.
A smaller number indicates fewer modifications and potentially higher security relative to the original musl. (Using *proportions* might yield misleadingly optimistic estimates).
Regular `diff` checks are encouraged.

We have implemented a script for evaluation; please read `./compare_dirs/COMPARE_DIRS_README.md`.

## 4. Development and Contribution

### 4.1. Standard Operating Procedure (SOP) for Migrating/Adding Functions

To safely add missing functions or migrate existing functions from musl, please **strictly follow these steps**:

1. **Analyze musl Source Code:** Read the relevant musl source code. Identify potential risks:
    *Are there any direct system calls?
    * Does it rely on complex assembly?
    * Are there subtle dependencies or assumptions?
2. **Evaluate Impact (musl):** Understand the scope of the function in musl and consider how to test its behavior.
3. **Evaluate Impact (mocklibc):** Assess the extent of missing support in `mocklibc` and estimate the scope of modifications required.
4. **Migrate Code:** Use the `cp` command to copy the relevant source files from the musl tree to the corresponding location in the `mocklibc` tree.
    ***Crucial:** **Do not use copy-paste.** Avoid opening/saving files with code formatters or editors that might change whitespace/line endings, as this will break `diff` comparisons.
    * **Crucial:** Musl often has strict requirements for the order of header file inclusions. Formatters might reorder headers, leading to compilation errors. Preserve the original order.
5. **Adapt Code and Build System:**
    *Modify necessary definitions in header files (e.g., remove comments blocking definitions, add system call number definitions).
    * If system calls are used, replace them with appropriate `ABI-SYS CALL` or `ABI CALL` implementations. Currently, `ABI-SYS CALL` is fully supported and `syscall` related macros can be used directly. Avoid any form of `ABI CALL`.
    * If new source files/directories are added, update the `LIBSRC` variable in the `Makefile`.
6. **Compile and Test:**
    *Compile `mocklibc`. The goal is zero errors/warnings (except for potential `NULL` redefinition warnings, which may be acceptable depending on the specific case).
    * Rebuild/repackage any dependent components (e.g., loader or test applications).
    * **Run all existing tests** to ensure that the changes do not introduce regressions. If possible, specifically test the newly added functionality.
7. **Commit Changes:** Use `git` to commit the changes for this specific function migration **as a separate, isolated commit**. This makes it easier to roll back later if issues are found.

**Adhering to this SOP is crucial for avoiding difficult-to-diagnose problems.**

### 4.2. Build System (Makefile)

* **Configuration File:** The build system first includes the `config.mk` file from the project root directory. Users can configure build options by editing this file.
* **Toolchain:**
  *The default target platform prefix is `riscv64-linux-musl`.
  * The relevant cross-compilation tools (`gcc`, `as`, `ld`, `ar`, `strip`, `objcopy`) are all defined based on this platform prefix.
* **Main Compilation Options (`CFLAGS`):**
  *`-nostartfiles -ffreestanding -nostdlib -nostdinc`: Key options indicating that we are building a library independent of the standard host environment, without using standard startup files, standard libraries, or standard header files.
  * `-mcmodel=medany`: RISC-V specific code model.
  *`-fPIC -pie`: Generate position-independent code, suitable for building dynamic libraries (`.so`) and position-independent executables.
  * `-I...`: Automatically include header file paths for `arch/riscv64/`, `include/`, and all subdirectories within `c/`.
* **Optimization Level:** Can be controlled by the `OPTIMIZE` variable in the `config.mk` file:
  *`OPTIMIZE = 0`: `-O0` (no optimization)
  * `OPTIMIZE = 1`: `-O1`
  *`OPTIMIZE = 2`: `-O2`
  * `OPTIMIZE = 3`: `-O3`
  * Other values (default): `-Os` (optimize for size)
* **Malloc Implementation Selection:** Can be controlled by the `USE_SAFE_MALLOC` variable in the `config.mk` file:
  *`USE_SAFE_MALLOC = 1`: Use the Malloc implementation under the `c/safe_malloc/` directory.
  * `USE_SAFE_MALLOC != 1`: No alternative implementation is currently specified in the `Makefile` (marked as `# TODO`).
* **Source Code (`LIBSRC`):** The `Makefile` uses `wildcard` to collect `.c` files from various subdirectories within `c/` as source files for the library, including C runtime (`__rt_*`), various standard library modules (ctype, stdio, math, etc.), and architecture-specific implementations (setjmp_riscv64, signal_riscv64, etc.).
* **Object Files:**
  *`.c` and `.s` files are compiled/assembled into `.o` files, stored in the `obj/` directory, maintaining the subdirectory structure from `c/`.
  * `c/crt1.c` is compiled separately as `obj/crt1.o`.
* **Output Library Files:**
  *Static library: `lib/libmock.a`
  * Dynamic library: `lib/libmock.so`
* **Current Status:** The `Makefile` is currently quite basic and primarily supports the RISC-V (RV) architecture.
* **Future Goals:** The build system needs significant improvement. Ideally, relevant parts of the original musl build system should be migrated to provide better cross-architecture support and configurability.

## 5. Current Status and Future Work

* **Architecture Support:** The current `Makefile` is mainly targeted at the `riscv64-linux-musl` platform.
* **Directory Structure:** Aims to simulate `musl-1.2.2` as much as possible for familiarity but is intentionally simplified to reduce the difficulty of making changes.
* **`./rela/` Directory:** This directory is believed to be an unnecessary remnant and should be investigated and possibly **removed**.
* **`start_c`:** The current implementation of `start_c` (responsible for C runtime initialization) is rather rough and needs further refinement for both static and dynamic linking scenarios.
* **Build Configuration:** Ideally, `mocklibc` should support configurable builds (e.g., through a `config` file) to allow enabling/disabling major features (such as multi-process, networking, math library, etc.), allowing users to build customized library versions (from minimal support to full functionality). This feature is not yet implemented.
* **Versioned Releases:** Ideally, `mocklibc` should provide source code branches or tags for multiple musl versions and allow users to choose which musl version to build against. This feature is not yet implemented.
* **Makefile Robustness:** The current `Makefile` has relatively basic functionality. Future goals include making it more robust and potentially migrating more features from the native musl build system for better flexibility and cross-platform capabilities.

## 6. Build and Run

### 6.1. Configure Build

Before compiling, edit the `config.mk` file in the project root directory to set the desired configuration options, for example:

```makefile
# config.mk Example
OPTIMIZE = 0        # Use -O0 optimization
USE_SAFE_MALLOC = 1 # Use safe_malloc implementation
## 7. Running Method

The current Makefile provides two compilation methods: dynamic library compilation and static library compilation.

Compile All:

```bash
make
```

### Execute Compilation

Execute the following commands in the directory where the `Makefile` is located:

* Compile all (static library + dynamic library):

```bash
make
# Or make all
```

* Compile static library only (`lib/libmock.a`):

```bash
make static
```

* Compile dynamic library only (`lib/libmock.so`):

```bash
make dynamic
```

The compilation products will appear in the `lib/` directory.

### 6.3. Clean Compilation Results

Execute the following command to delete the `obj/` and `lib/` directories and all their contents:

```bash
make clean
```
