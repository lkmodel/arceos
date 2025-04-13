# mockc_apps - Uni Mode Application Packager (To be Revised)

**Read this in other languages: [English](./README.md), [中文](./README_zh.md).**

**Status:** To be Revised (Needs Revision) - Planned to be renamed `uni_apps`

## Overview

`mockc_apps` contains a set of **simple** build and packaging scripts, primarily used for packaging single applications in **`Uni Mode`** (Unikernel Mode).

Its purpose is to package a pre-compiled application (statically or dynamically linked to `mocklibc`) into a basic application package that can be loaded and executed by `loader_lib` in `Uni Mode`.

**Note:** The scripts in this directory have relatively basic functionality and are planned to be revised and integrated into the new `uni_apps` directory. For the most complete packaging tools currently available, please refer to `../../batch_apps/` (used for `Batch Mode`).

## Usage

Execute the `make` command in the `mockc_apps` directory to perform packaging.

```bash
# Enter the mockc_apps directory
cd mockc_apps

# Execute packaging
# SRC: Specifies the name of the application to be packaged (corresponding to the filename in the APPS/ directory)
# TYPE: Specifies the linking type (dynamic or static)
make SRC=<app_name> TYPE=<dynamic|static>

# Example: Package a dynamically linked application named 'sqlite3'
make SRC=sqlite3 TYPE=dynamic

# Return to the parent directory
cd ..
```

The packaging process typically includes:

1. Locating the specified application file (<app_name>) in the APPS/ directory.
2. Determining whether to include the `mocklibc` dynamic library (`.so`) or link the static library (`.a`) based on the TYPE parameter (the specific linking logic may be handled in this script or during the application compilation stage).
3. Generating a simple application package structure for loader_lib to use in `Uni` Mode.

## Dependencies

Pre-compiled application executable files placed in the `APPS/` directory.
If packaging a dynamically linked application (TYPE=dynamic), the `ulib/mocklibc_lib/lib/libmock.so` file needs to exist at the expected relative path.
If the application is statically linked to `mocklibc` (TYPE=static), it needs to have been correctly linked with `ulib/mocklibc_lib/lib/libmock.a` during application compilation.

## Future Plans

* Rename this directory to `uni_apps`.
* Revise and improve the packaging scripts, possibly drawing on the mature features in batch_apps (such as parameter passing and more flexible configuration).
* Better integration with the `mocksrc` directory (responsible for source code compilation).
