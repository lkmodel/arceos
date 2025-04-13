# mocksrc - Source Code Compilation and Linking Center (Under Development)

**Read this in other languages: [English](./README.md), [中文](./README_zh.md).**

**Status:** Under Development

## Overview

The `mocksrc` directory is planned to be the central location in the ArceOS project for **uniformly storing application source code compilation and linking scripts**.

Its **goal** is to provide a set of tools and scripts that allow developers to start directly from application source code, compile, and link to generate executable files suitable for the ArceOS `loader_lib` environment (including static and dynamic linking to `mocklibc`).

This will form a division of labor with `batch_apps` and `uni_apps` (formerly `mockc_apps`): `mocksrc` will be responsible for **compilation and linking**, while the latter two will be responsible for **packaging** the compiled executables into `apps.bin` application packages.

## Current Content

Currently, this directory may contain:

* `link.ld`: Example or basic linker script.
* `Makefile`: Initial or placeholder build script.

**Note:** The current functionality is very limited and is still in the early stages of development.

## Development Goals

* Establish a unified application compilation framework.
* Manage compilation configurations and dependencies for different applications.
* Correctly handle static and dynamic linking against `mocklibc`.
* Support the musl toolchain, with plans to support the GCC toolchain in the future.
* Be compatible with different loading modes of `loader_lib` (`Uni`, `Batch`, `PMP`).
* Work collaboratively with application packaging scripts (`batch_apps`, `uni_apps`).

## Usage

**None yet.**

Detailed usage instructions will be provided after the functionality is developed.

## Relationship with Other Components

* **Depends on:** `ulib/mocklibc_lib` (provides the `mocklibc` library and header files).
* **Serves:** `batch_apps`, `uni_apps` (provides compiled executables for packaging).
* **Target Environment:** `examples/loader_lib` (the compiled applications are ultimately executed by the loader).
