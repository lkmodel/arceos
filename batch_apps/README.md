# Batch Mode Build Script Instructions

**Read this in other languages: [English](./README.md), [中文](./README_zh.md).**

## 1. Overview

This document describes a batch mode build script and its specifications for packaging multiple applications, optional shared libraries (`Lib`), and execution instructions into a single binary file (`apps.bin`).
This tool is designed to simplify the aggregation and deployment of applications in a specific target environment (ArceOS).

## 2. Core Specifications

### 2.1. Application Package Type Definition

To clearly manage dependencies and package content, we define the following application package types based on the presence or absence of the `Lib` library and the types of applications within the package:

**Determining the Existence of the `Lib` Library:**

* **`Lib` Library Does Not Exist**: The package is considered not to contain a `Lib` library when the size of the `Lib` library is `0` **and** its starting address is also `0`.
* **`Lib` Library Exists**: The package is considered to contain a `Lib` library when the size of the `Lib` library is **not** `0` **and** its starting address is also **not** `0`.

**Package Type Classification:**

* **Pure Static Application Package**
  * **Condition**: `Lib` library **does not exist**.
  * **Content**: The package **only contains** static applications and is not allowed to contain any dynamic applications.
* **Mixed Dynamic Application Package**
  * **Condition**: `Lib` library **exists**.
  * **Content**: The package **contains both** dynamic and static applications.
* **Dynamic Application Package**
  * **Condition**: `Lib` library **exists**.
  * **Content**: The package **only contains** dynamic applications.

**Undefined Cases:**

* Any situation that does not satisfy the above conditions for the existence/non-existence of the `Lib` library (such as size being `0` but address not being `0`, or vice versa) is considered **undefined**.
* Any combination of package content that does not conform to the above three type definitions is also considered **undefined**.

### 2.2. Application Package (`apps.bin`) File Structure

The generated `apps.bin` file consists of **header metadata** and **actual data**:

**Header Metadata:**

| Field              | Size (Bytes) | Description                                                              |
| :----------------- | :---------- | :----------------------------------------------------------------------- |
| Magic              | 8          | File identifier                                                          |
| Count              | 4          | The number of applications included in the package                       |
| HdrSize            | 8          | The total number of bytes in the entire header region                    |
| **[Application Information Block]** |            | *Repeated 'Count' times* |
| > Application N - Size | 8          | The file size of the Nth application                                   |
| > Application N - Name | Variable   | The file name of the Nth application (null-terminated string `\0`)      |
| > Application N - Start Address | 8          | The offset of the Nth application's data within the file           |
| **Lib Library Information** |            |                                                                          |
| > Lib - Size         | 8          | Size of the `Lib` library file (0 if none)                               |
| > Lib - Start Address | 8          | Offset of the `Lib` library data within the file (0 if none)              |
| **Script Information** |            |                                                                          |
| > Script - Size      | 8          | Size of the encoded execution script                                     |
| > Script - Start Address | 8          | Offset of the encoded execution script data within the file             |

**Actual Data:** (Immediately follows the header, arranged according to the start addresses specified in the header)

1. File data of Application 1
2. File data of Application 2
3. ...
4. `Lib` library data (if it exists)
5. Encoded execution script data
6. Padding bytes (Padding zeros to 32MB total size)

### 2.3. Execution Script Format (Encoded)

The execution script (`script.txt`) is encoded into a binary format and stored with the following structure:

| Field                  | Size (Bytes) | Description                                                                 |
| :--------------------- | :---------- | :-------------------------------------------------------------------------- |
| Magic                  | 8          | Script file identifier                                                     |
| Lines                  | 4          | The number of valid command lines                                           |
| **[Command Line Data]** |            | *Repeated 'Lines' times* |
| > Line Header Check (0xFF) | 1          | Fixed byte, used for verification or synchronization                       |
| > Argument Count (argc) | 4          | The number of arguments for the current command (including the application name) |
| > Application Name (argv[0]) | Variable   | Application name (null-terminated `\0`, **without path**)               |
| > Argument 1 (argv[1])   | Variable   | The first argument (null-terminated `\0`, if `argc > 1`)                  |
| > ...                  | ...        | ...                                                                         |
| > Argument N (argv[argc-1]) | Variable   | The last argument (null-terminated `\0`, if `argc > 1`)                   |

**`argv` Handling Notes:**
Different from standard C `main` function arguments:

* `argv[0]` is only the application name and does not include the path. This is to simplify script writing.
* Although `argc` is counted correctly, the system (ArceOS) loader **is not expected** to access `argv[argc]`, so the script encoding does not guarantee it to be `NULL`.

## 3. Writing and Usage Specifications

### 3.1. Execution Script (`script.txt`) Writing Specifications

* Create a `script.txt` file in the project root directory (or as specified by the `SCRIPT` variable in the `Makefile`).
* Adopt a **`Linux`-like command** style, with one command per line, used to launch an application.
* The **first word** must be the **application name**, and this name must **exactly match** a file name in the `APPS` directory.
* After the application name, use **spaces** to separate the parameters passed to the application.
* Empty lines are allowed and will be ignored.
* Example:

    ```txt
    app1 param1 param2
    app_without_suffix arg_a
    app3
    ```

### 3.2. `APPS` Directory Specifications

* This directory (path specified by the `APP_DIR` variable in the `Makefile`, defaults to `APPS`) is used to store **all** the application **executable files** that need to be packaged.
* **Strict Requirements**:
    1. The **file names** in the directory must **exactly match** the application names used in `script.txt`. It is not recommended to use suffixes like `.bin`. If used, the script must also include them.
    2. The directory **cannot** contain any **non-executable files** (such as source code, documents, subdirectories, etc.). This is because the packaging script directly treats all files in this directory as applications to be packaged, and the loader depends on file name matching.

## 4. Building and Running

### 4.1. Build Configuration (Makefile Variables)

You can modify the following main configuration variables at the top of the `Makefile` file:

* `APP_DIR ?= APPS`: The directory where application executable files are located.
* `INCLUDE_LIB ?= 1`: Whether to include the `Lib` library.
  * `1` (default): Include. **Must be set to `1` when there are dynamic applications in `APPS` that depend on `Lib`**.
  * `0`: Do not include. Used for pure static application packages.
* `MOCK_LIBC_SO ?= ../ulib/mocklibc_lib/lib/libmock.so`: Specifies the path to the `Lib` library file (only effective when `INCLUDE_LIB=1`).
* `SCRIPT ?= script.txt`: Specifies the name of the execution script file.

### 4.2. Executing the Build

1. **Prepare the `APPS` Directory**: Place the application executable files according to the specifications in `3.2`.
2. **Write `script.txt`**: Write the execution script according to the specifications in `3.1`.
3. **Ensure `Lib` Library Exists**: If `INCLUDE_LIB=1`, please confirm that the path pointed to by `MOCK_LIBC_SO` is valid.
4. **Run Make**: Execute the following command in the directory where the `Makefile` is located:

    ```bash
    make
    ```

5. **Obtain Results**: After a successful build, the final `apps.bin` file (size 32MB) will be generated in the `build/` directory and a copy will also be placed in the `../payload/` directory.

### 4.3. Cleaning Build Artifacts

Execute the following command to delete the `build/` directory and all its contents:

```bash
make clean
```

## 5. Status and Known Issues

* Directory Status: **Frozen**: The current functionality and structure of the script are frozen.
  * We have retained the old documentation for original developers to review and correct. Any new developers and users should not read it (as it may contain errors or unclear descriptions).

* Known Issues:
  * Insufficient Testing of Static Applications and Missing Lib Scenarios: The combination of a "pure static application package" (i.e., only static applications in `APPS` and `INCLUDE_LIB=0`) has not been fully tested. However, considering that this scenario is not included in the manual, no special action will be taken.
  * Library Updates: If the `Lib` library file (the file pointed to by `MOCK_LIBC_SO`) is updated, `make` must be re-executed to package the updated library into `apps.bin`.

## 6. Future Development Suggestions

* Simultaneously support loading multiple `Libs`, with each application specifying its Lib version. When an application is packaged, determine its required `mocklibc` release version through the script and write the information into the application package. Package multiple `mocklibc` release versions within the application package.
* Do not rely on a specific directory to load applications (`APPS`), but instead search for them in a common directory through the script, instead of packaging all of them.。
