# Lib Loader (`loader_lib`) Documentation

**Read this in other languages: [English](./README.md), [中文](./README_zh.md).**

**Status:** Core component, under continuous development

## 1. Overview

This document details the `loader_lib` component, which is responsible for loading and managing applications and shared libraries (`Lib`) within the ArceOS environment.
It aims to provide guidance for contributors or developers who wish to understand the loader architecture, development progress, design decisions, and known issues.

**Project Goals (Phased):**

1. ~~Supports single applications statically linked with musl (application source code requires no modification).~~
2. ~~Supports single applications dynamically linked with musl (original binary applications require no modification).~~
3. ~~Supports launching multiple applications by supporting `fork` and multiple address spaces.~~
4. ~~Supports file systems such as `procfs`/`sysfs` to run `BusyBox`, `LTP`, etc., expanding system call coverage.~~
5. Supports an application compilation toolchain from musl to gcc.

## 2. Terminology Conventions

To ensure clarity, we define the following terms:

* **Application Package:** A single binary file (`apps.bin`) containing all components required by the kernel: libraries, execution scripts, and application code. (See `../../batch_apps/README.md` for details).
* **`Kernel [Code/Space]`:** Refers to the core code of `ArceOS` and its memory address space. Although `ArceOS` is a `Unikernel`, we still use "`Kernel`" to refer to its core part.
* **`App [Area/Code/Space]`:** Refers to the application-specific area within the application package, the application code, and its memory address space during execution.
* **`Lib [Area/Code/Space]`:** Refers to the shared library area within the application package, the library code (e.g., `mocklibc`), and its memory address space during execution.
* **`ABI CALL`:** A function call mechanism where `Lib` code jumps to execute functions implemented in the **`Kernel Code`** via `abi_entry`. These functions are bundled and released with the `Kernel`.
* **`ABI-SYS CALL (SYS CALL)`:** A special type of `ABI CALL` that passes system call numbers and arguments during invocation to simulate traditional kernel system calls. The `Kernel` dispatches these calls to specific handlers based on the system call number. They are released with the `Kernel` like `ABI CALL`s but provide a more standardized interface. `SYS CALL` can be used interchangeably when the context is clear. (Note the distinction; there are no traditional trap-based system calls in a Unikernel).
* **`Unikernel Mode (Uni Mode)`:** A kernel mode selectable in `Cargo.toml`. In this mode, it is assumed that there is only one application in the application package, and it conforms to the `Unikernel` model.
* **`Batch Mode`:** A kernel mode selectable in `Cargo.toml`. In this mode, it is assumed that there are multiple applications in the application package, each conforming to the `Unikernel` model. **This is the most mature mode currently under development and is the recommended mode for priority use.**
* **`pseudo_multi_process mode (PMP mode)`:** Pseudo multi-process mode (under development). In this mode, it is assumed that there is one multi-process mode application in the application package. This mode is still under development, and the ideal goal is to merge it with `Batch Mode` by loading other applications in the application package to achieve functions such as `fork`.
* **`Compile Runtime Function (CRT)`:** Functions provided to `App` or `Lib` to implement runtime functions in the musl library that are masked or missing due to compilation options (such as `-nostdlib`).

## 3. Design Ideas and Implementation Considerations

### 3.1. Core Approach: Migrate and Modify musl

This project chooses to migrate the source code based on the `musl libc 1.2.2` library and modify it to adapt to ArceOS (this is "Idea One"). The main modifications include:

* Replacing **system calls** in musl with **`ABI-SYS CALL`**.
* Replacing some musl functions (especially those that cannot run directly or contain prohibited instructions) with **`ABI CALL`**.

**Reasons for choosing this approach:**

* **Simplified Testing:** Only the modified code and the `ABI CALL`/`ABI-SYS CALL` interfaces need to be tested extensively; most of the unmodified musl code can be trusted for correctness.
* **Development Efficiency:** Compared to completely rewriting libc ("Idea Two"), migration and modification can achieve a fully functional library faster.
* **Security:** Reusing the widely verified musl code reduces the risk of introducing errors through self-implementation.

**Dynamic Linking Implementation Mechanism:**

When loading a dynamically linked application, `loader_lib` loads `mocklibc.so` (Lib) and `app.bin` (App) from the `apps.bin` application package into memory. By modifying the relocation table of the ELF file (such as PLT), calls from App to library functions are directed to the implementations in Lib, calls from Lib that might target App functions (such as `main`) are redirected, and the `ABI CALL` table address in Lib is relocated.

### 3.2. Method Trade-offs (Idea One vs. Idea Two)

**Advantages of the current method (Idea One):**

* **Library Flexibility:** Separation of `Kernel`, `Lib`, and `App` code. `mocklibc` can be released in different versions and with different features (such as selectively including multithreading, network, and math library support) without modifying the ArceOS Kernel. It can be loaded on demand, balancing memory overhead and functionality.
* **Loading Speed:** No runtime symbol lookup algorithm is required.
* **ArceOS Core Size:** The Kernel itself can be smaller.

**Disadvantages of the current method (Idea One):**

* **Memory Footprint:** Both App and Lib need to be loaded at runtime, resulting in a relatively higher memory footprint.
* **Implementation Complexity:** The loading and relocation process is relatively complex.
* **Deployment Process:** An additional packaging step is required to package App and Lib into `apps.bin`.

(Idea Two: Completely reimplement a musl-compatible library. Advantages: smaller memory footprint, potentially simpler implementation; Disadvantages: huge workload, complex testing and verification, tight coupling between library and kernel implementation.)

### 3.3. Handling Assembly Code in musl

Special care must be taken when migrating code in musl that uses assembly (`.S` files or inline assembly):

1. **Trap Risks:** Assembly code often contains architecture-specific trap instructions (such as `ecall` on RISC-V), which are prohibited.
2. **Environment Assumptions:** Assembly code may rely on specific CPU states, memory layouts, or operating system behaviors that ArceOS may not provide.

Given these risks, when encountering complex or potentially problematic assembly code in musl, **it is strongly recommended to use `ABI CALL` in `mocklibc` to implement the corresponding functionality** instead of attempting to directly and potentially unsafely migrate the assembly code.

### 3.4. Evaluating Library Quality and Security

Although unmodified code from musl is generally trusted, the deviations introduced in `mocklibc` need to be evaluated.

* **Recommendation:** Use the `diff` command to compare the `mocklibc` source tree with the original `musl-1.2.2` source tree.
* **Metric:** The **number of dissimilar files** can serve as a rough indicator of the degree of deviation. A smaller number indicates fewer modifications and potentially higher security relative to the original musl. (Using *proportions* might yield misleadingly optimistic estimates). Regular `diff` checks are encouraged.

## 4. Project Background and Related Components

`loader_lib` is integrated with several other parts of the ArceOS project:

* `../../batch_apps/` (**Frozen**): Handles application packaging for batch mode. See its `README.md` for details.
* `../../examples/loader_lib/` (**Current directory**): Contains the core source code of `loader_lib`.
* `../../mockc_apps/` (**To be revised**): A simple compilation/packaging script for "uni" loading mode. Planned to be revised and renamed to `uni_apps`. See its `README.md` for details.
* `../../mocksrc/` (**Under development**): The planned future location for all source code compilation and linking scripts. See its `README.md` for details.
* `../../ulib/mocklibc_lib/` (**Lib library source code**): Our modified `mocklibc` library used for loading dynamically linked musl-based applications. See its `README.md` for details.

## 5. `loader_lib` Internal Structure

File tree overview:

```txt
.
├── batch_apps (Frozen). Application packages used for batch mode. More information can be found in the `README.md` under this directory.
│   ├── APPS
│   ├── build
│   ├── generate_header.py
│   ├── Makefile
│   ├── pack_apps.py
│   ├── README.md
│   ├── script_encoder.py
│   └── script.txt
├── Cargo.lock
├── Cargo.toml
├── examples
│   ├── loader_lib (Core code, under continuous development)
│   │   ├── Cargo.toml
│   │   ├── loader_lib_riscv64-qemu-virt.bin
│   │   ├── loader_lib_riscv64-qemu-virt.elf
│   │   ├── README.md
│   │   └── src
│   │       ├── abi
│   │       ├── config.rs
│   │       ├── elf_load
│   │       │   ├── auxv.rs
│   │       │   ├── batch
│   │       │   ├── decoder.rs
│   │       │   ├── load.rs
│   │       │   ├── mod.rs
│   │       │   ├── uni
│   │       │   ├── uni_load.rs
│   │       │   └── verify.rs
│   │       ├── init.rs
│   │       ├── linux_env
│   │       │   ├── axfs_ext
│   │       │   ├── axhal_ext
│   │       │   ├── eventfd_ext
│   │       │   ├── linux_api
│   │       │   ├── mem_ext
│   │       │   ├── mod.rs
│   │       │   ├── process_ext
│   │       │   └── task_ext
│   │       ├── main.rs
│   │       └── syscall
│   │           ├── api.rs
│   │           ├── ctypes.rs
│   │           ├── mod.rs
│   │           ├── syscall_fs
│   │           ├── syscall_mem
│   │           ├── syscall.rs
│   │           └── syscall_task
├── LICENSE.Apache2
├── LICENSE.GPLv3
├── LICENSE.MulanPSL2
├── LICENSE.MulanPubL2
├── loader_lib.sh
├── Makefile
├── mockc_apps (Needs modification)
├── mocksrc (Needs further development)
│   ├── link.ld
│   └── Makefile
├── README.md
├── rust-toolchain.toml
└── ulib
    └── mocklibc_lib (Lib library). This is our modified `mocklibc` library used for loading dynamically compiled `musl` applications. More information can be found in the `README.md` under this directory.
```

Key subdirectories under `./src/`:

* `abi/`: Implements the `ABI CALL` and `ABI-SYS CALL` mechanisms.
  * `mem.rs`: Implements manual memory allocation functions (malloc family) via `ABI CALL`. Long-term goal: Replace memory allocation functions with `ABI-SYS CALL` without causing memory leaks.
  * `thread.rs`: Implements thread management functions via `ABI CALL`. *(Note: The `tp` register needs to be set before entering the application - see known issues). Short-term goal: Replace the thread function family with `ABI-SYS CALL`.
  * `syscall.rs`: Implements the kernel-side implementation of `ABI-SYS CALL` and system call number dispatching.
  * `noimpl.rs`: Placeholders and active occupiers for unimplemented `ABI CALLs`.
  * `mod.rs`: Manages `ABI CALL` number allocation, registration, and dispatching. Unregistered calls default to `abi_noimpl`. This also implements CRT functions.
* `elf_load/`: Contains all ELF file loading and execution logic.
  * `batch/`: Implements loading and running for Batch Mode. (This structure should be reused by other modes).
  * `uni_load.rs`: (To be refactored) Current loader for `Uni` Mode. Should be moved to a new `uni/` subdirectory.
  * `load.rs`: (To be refactored) Current loader for `PMP` Mode. The structure needs reorganization; `PMP` Mode is still under active development.
  * `decoder.rs`: Common functions for decoding application package headers and scripts (currently used only by Batch Mode).
  * `auxv.rs`: (To be refactored) Implements auxiliary vector setting, currently related to `PMP` Mode; directory structure needs review.
  * `verify.rs`: ELF format verification code.
* `linux_env/`: Simulates parts of the Linux environment and provides extensions for core ArceOS modules. Goal: Minimize modifications to the core modules themselves.
* `syscall/`: Implements the server-side (kernel-side) logic for handling `ABI-SYS CALL` requests.
* `config.rs`: Defines constants and configuration values used by loader_lib.
* `init.rs`: (To be refactored) Contains initialization logic for `uni` and `batch` modes; the structure should be optimized.
* `main.rs`: The entry point for the core logic, coordinating the loading process.

## Build Configuration

Build options and features can be configured through `Cargo.toml`. Please refer to the comments in the `Cargo.toml` file for details on available feature flags.

## 7. Memory Layout (Conceptual)

| Address Range             | Description                                      |
|---------------------------|--------------------------------------------------|
| `0xFFFF_FFC0_8000_0000`   | <- Start of SBI reserved memory                  |
| `...`                     | SBI Region                                       |
| `0xFFFF_FFC0_8010_0000`   | <- End of SBI / Start of Lib Region             |
| `...`                     | Lib code/data loaded here (reuses SBI gap)       |
| `0xFFFF_FFC0_8020_0000`   | <- End of Lib Region / Start of Kernel Region   |
| `...`                     | Kernel code/data located here                    |
| `0xFFFF_FFC0_8700_0000 (?)`| <- End of Kernel Region / Start of App Region (configurable address) |
| `...`                     | App code/data loaded here                       |
| `0xFFFF_FFC0_8800_0000 (?)`| <- End of App Region (configurable address)      |

*(Note: The exact Kernel/App boundaries may vary depending on the configuration.)*

## 8. Current Status and Supported Features

* **Mode Support:** `Batch Mode` is currently the most feature-complete and stable mode, recommended for priority use. `Uni Mode` and `PMP Mode` are still under development and refactoring.
* **Application Compatibility:** Currently, simple `ash` terminals and most `BusyBox` applications that do not rely on advanced process (`fork`) or signal handling can be run.

## 9. Known Issues and Important Notes (Reserved Errors)

* **`tp` Register Requirement:** `musl` libc assumes the `tp` register points to the `pthread_t` structure of the current thread (i.e., the return value of `pthread_self()`). `mocklibc` currently bypasses this issue by replacing code that directly accesses the `tp` register with `ABI CALL`s that implement `pthread_self`. **However, the loader does not correctly set the `tp` register before entering the application.** This means that any code within `Lib` that directly accesses `tp` or relies on thread-local storage (TLS) implementations that depend on `tp` may fail. This needs to be resolved during the loader's context setup phase.
* **`abi_entry` Boundary Check:** The mechanism for passing the `abi_entry` call table to Lib may allow Lib to request a call number that exceeds the actual size of the table. Security checks should be implemented in the `abi_entry` dispatch logic (possibly in `abi/mod.rs`?) to prevent out-of-bounds access, potentially by defaulting to `abi_noimpl` or safely triggering a panic.
* Memory Distribution Conflict: We have a defined range for the kernel, and theoretically, the space from `0xFFFF_FFC0_8700_0000` to `0xFFFF_FFC0_8800_0000` can also be used by the kernel. However, in the current situation, we have not encountered any errors that require correction.
* Thread Control Block Error: We return the thread control block through `pthread_self()`, but there is a critical issue in that the returned value does not seem to satisfy musl's assumptions about the thread control block, leading to errors when accessing the returned structure. The fundamental solution is to replace `ABI CALL` with `ABI-SYS CALL`.
* `malloc` Allocation Issue: It is certain that allocating a large amount of memory at once (2M) may cause block allocation errors, likely because a large memory block exhausts the already allocated heap or pages. We have temporarily worked around this issue by manually performing a small memory allocation before each larger allocation to manually trigger the lazy initialization logic for heap or page allocation.

## 10. Build and Run Guide

1. **Clone the Repository:**

    ```bash
    git clone [https://github.com/inchinaxiaofeng/arceos.git](https://github.com/inchinaxiaofeng/arceos.git) # Or the core repository lkmodel/arceos
    cd arceos/
    ```

2. **Compile `mocklibc` (if updates are needed):**

    ```bash
    cd ulib/mocklibc/
    make
    cd ../../
    ```

3. **Package Applications:**

     * **Uni Mode (Example):**
  
    ```bash
    cd mockc_apps/ # Will be renamed to uni_apps later
    make SRC=<app_name> TYPE=<dynamic|static> # For example: make SRC=sqlite3 TYPE=dynamic
    cd ..
    ```
  
    * **Batch Mode:**
  
    ```bash
    cd batch_apps/
    make
    cd ..
    ```

4. **Run `loader_lib`:**

    ```bash
    # Default parameters: -l warn -q y
    # Recommended parameters for running complex applications:
    ./loader_lib.sh -l off -q n
    # Optional log levels: -l <debug|warn|info|off|trace>
    # Whether to use QEMU graphical interface: -q <y|n>
    ```
5. **Execute using `cargo-xtask`**
  * **Uni Mode:**
   ```bash
   cargo xtask uni <app_name> [dynamic(default)|static]
   # e.g. cargo xtask string
   # Performs compilation, packaging, and execution in sequence
   ```
  * **Batch Mode:**
   ```bash
   cargo xtask batch [SCRIPT]
   # Performs compilation, packaging, and execution in sequence
   # Specifying [SCRIPT] overrides the script.txt under batch_app for packaging and execution
   # e.g. cargo xtask batch "busybox echo Hello,ArceOS!"
   ```
  * **Debug Mode**
   ```bash
   cargo xtask --debug <batch|uni>
   # Requires installation zellij
   ```
  Also supports:
  
  * -l <debug|warn|info|off|trace> (log level configuration)
  
  * -q <y|n> (quiet mode toggle)
## 11. Code Improvement Suggestions

* Directory Structure Refactoring (as described in README):
  * Suggested Action: Create src/elf_load/uni/ and src/elf_load/pmp/ directories.
  * Suggested Action: Move the logic of src/elf_load/uni_load.rs into the new src/elf_load/uni/ module (e.g., src/elf_load/uni/load.rs and update src/elf_load/uni/mod.rs).
  * Suggested Action: Move the logic of src/elf_load/load.rs (PMP loader) into the new src/elf_load/pmp/ module.
  * Suggested Action: Move src/elf_load/auxv.rs into src/elf_load/pmp/ or a more suitable common location (if other modes use it in the future).
  * Suggested Action: Analyze src/init.rs. Split the initialization logic for different modes (uni, batch) into their respective modules (e.g., src/elf_load/batch/init.rs, src/elf_load/uni/init.rs) or into functions called by init.rs based on the detected mode.
  * Rationale: Improve code organization, make responsibilities clearer, and align the structure with the expected design described in the README.
* Resolve Known Issues:
  * tp Register:
    * Suggested Action: In the code responsible for setting up the initial context before jumping to the application's entry point, add the necessary assembly or function calls to correctly initialize the tp register for the main thread.
    *Suggested Action: In src/abi/thread.rs and near the code that jumps to the application's entry point, add prominent comments (`// FIXME:`, `// TODO:`, or specific warnings) explaining the dependency on the tp register and the need for the loader to set it correctly.
    * Rationale: Fix a critical correctness issue that the musl/mocklibc threading model depends on.
  * abi_entry Boundary Check:
    * Suggested Action: Locate the dispatch logic for ABI CALLs (likely in src/abi/mod.rs or a function called by abi_entry). Add checks before indexing the ABI function table/slice with the provided call number.
    * Rationale: Prevent potential crashes or memory corruption due to out-of-bounds access, improving security and stability.
* Improve Readability and Comments:
  * Suggested Action: Review complex modules such as elf_load (especially verify.rs, decoder.rs, and mode-specific loaders) and syscall handlers. Add module-level (`//!`) and function-level (`///`) documentation comments explaining their purpose, high-level logic, and any non-obvious assumptions.
  * Suggested Action: Add inline comments (`//`) for complex logical steps, unsafe code blocks, or workaround implementations.
  * Suggested Action: Ensure consistent naming conventions for functions, variables, and modules.
  * Rationale: Make the code easier for other developers (and your future self) to understand, maintain, and debug.
* Error Handling:
  * Suggested Action: Look for uses of `.unwrap()` and `.expect()` in the code. Evaluate whether these represent truly unrecoverable errors or if using `Result` propagation (`?` operator) or more specific error handling (e.g., `match`) would be more robust. Pay particular attention to file I/O, memory allocation/mapping, and ELF parsing sections.
  * Rationale: Improve code robustness by preventing unexpected panics and allowing for more graceful error reporting or recovery where possible.
* Mark Unfinished Work:
  * Suggested Action: Ensure that parts mentioned in the documentation as being under development (PMP mode, uni_load.rs refactoring, potential ABI-SYS CALL replacements in abi/thread.rs) are clearly marked in the code with `// TODO:`, `// FIXME:`, or `unimplemented!()` macros.
  * Rationale: Clearly communicate the development status and remaining tasks.

## 12. Next Development Direction Suggestions

* **Improve System Calls & ABI Replacement:**
  *Continue migrating source code from `musl-1.2.2` to enhance `mocklibc` functionality based on application loading experiments.
  * Replace more `ABI CALL` implementations (especially those related to `pthread` and `malloc`) with `ABI-SYS CALL` based on `syscall`.
* **Improve Compile Runtime (CRT) Functions:**
  * Continue to expand the `CRT` functions (400+) and implement runtime functions not currently supported by Crates ourselves.
* **Improve Code Quality and Framework:**
  *Improve the code framework and reduce the use of `unwrap`, `expect`, `unsafe`, etc.
  * Check the `mocklibc` source code and try to maintain consistency with `musl-1.2.2` (`diff` checks), clearly documenting necessary modifications.
* **Feature Enhancement and Flexibility:**
  * **Lib Versioning and Modular Management:**
    * Support loading specific versions and configurations of the `mocklibc` library based on application scripts.
    * Support multiple `Lib` libraries within an application package.
    * Package and release `mocklibc` in modules for on-demand loading (similar to ArceOS componentization).
  * **PMP Mode Improvements:** Explore implementing more robust process and signal functionality based on hypervisor support (`hyper-process`/`hyper-signal`?) to overcome the limitations of the current PMP mode (inefficient and insecure).
    * ==Based on this, we can try to envision a more complex, hypervisor-based operating system.==
  * **Toolchain Support:** Gradually add support for applications compiled with the `GCC` toolchain, building upon the existing musl support.
  * Support running multiple libs, especially supporting application-dependent Lib libraries.
* **Project Structure Adjustment:**
  *Migrate the parameter passing and other improved functionalities of `batch mode` to `uni mode`.
  * Rename the `mockc_apps` directory to `uni_apps` to specifically handle `uni mode` packaging.
  *Create a new packaging script directory for `PMP` mode.
  * Create a new directory (`mocksrc`?) to uniformly store source code compilation and linking scripts.
* **Implementation Strategy Discussion:** Discuss whether complex functions (such as `pthread`, `malloc`) are better implemented in `mocklibc` (Lib) based on musl or in `loader_lib` (Rust/Kernel) as `ABI CALL`/`ABI-SYS CALL`.

## 13. Other Resources and Notes

* **Documentation Status:** The documentation for this project is still being improved. For the latest version, please follow the developer's forked repository and submitted Pull Requests.
* **Developer's Notes/Blog:** <https://inchinaxiaofeng.github.io/>
* **Weekly Work Report:** <https://docs.qq.com/sheet/DTmNIeXFzanlObXN1?tab=BB08J2>
* **Core Code Repository:** <https://github.com/lkmodel/arceos/tree/mocklibc_libloader> (Note that merging may be delayed)
* **Developer's Fork (Potentially More Up-to-Date):** <https://github.com/inchinaxiaofeng/arceos/tree/mocklibc_libloader>

## 14. Imagination of Future Full Support

The goal of `loader_lib` is to evolve into a core and highly flexible component in ArceOS, responsible for loading and managing native Linux applications and shared libraries (Libs), building upon the existing Linux community to lay the foundation for a rich and diverse application ecosystem.

* Broad Application Compatibility: `loader_lib` will be able to seamlessly load and execute various applications compiled using both `musl` and `GCC` toolchains. Whether statically or dynamically linked, applications will run efficiently within the ArceOS environment, significantly expanding ArceOS's applicability.
* Mature Multi-Application Support: Building upon the current state, we will implement a more complete and efficient multi-application management mechanism. A more advanced process model based on a hypervisor will be implemented to achieve stronger isolation and security.
* Flexible Library Management: `loader_lib` will support more fine-grained management of shared libraries (Libs). Different versions and configurations of `mocklibc` or other shared libraries will be loadable and manageable according to application needs. Application packages can contain multiple `Lib` libraries with on-demand loading support, optimizing memory usage and enhancing system flexibility. The ultimate goal is to achieve a componentized library management model, allowing ArceOS's functionality to be built and extended in a modular way.
* Rich System Functionality Support: By continuously improving the coverage of `ABI-SYS CALL`, including support for more file systems and the implementation of more advanced system calls, ArceOS will be able to run more complex applications. This will significantly enhance ArceOS's functional completeness and practicality.
* Powerful Toolchain Support: In addition to the existing `musl` support, full support for the `GCC` toolchain will be added in the future, enabling developers to leverage a wider range of development tools and libraries to build applications on ArceOS.
* Optimized Performance and Security: While pursuing functional completeness, `loader_lib` will also continuously focus on performance optimization and security enhancement. Through finer-grained memory management, more secure `ABI CALL` mechanisms, and continuous assessment and remediation of potential security risks, we will ensure that ArceOS can run various applications stably and reliably.
