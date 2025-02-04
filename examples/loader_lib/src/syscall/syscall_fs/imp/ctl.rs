use crate::{
    linux_env::{
        axfs_ext::api::OpenFlags,
        linux_fs::fd_manager::{FDM, alloc_fd},
    },
    syscall::{SyscallError, SyscallResult, ctypes::Fcntl64Cmd},
};
use axlog::{debug, info};

/// 功能:获取当前工作目录；
/// # Arguments
/// * `buf: *mut u8`, 一块缓存区,用于保存当前工作目录的字符串。当`buf`设为`NULL`,由系统来分配缓存区。
/// * `len: usize`, `buf`缓存区的大小。
/// # Return
/// 成功执行,则返回当前工作目录的字符串的指针。失败,则返回`NULL`。
/// 暂时:成功执行,则返回当前工作目录的字符串的指针 `as isize`。失败,返回0。
///
/// TODO: 当前写法存在问题,`cwd`应当是各个进程独立的,而这里修改的是整个`fs`的目录
pub fn syscall_getcwd(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:创建目录；
/// # Arguments
/// * `dirfd: usize`, 要创建的目录所在的目录的文件描述符。
/// * `path: *const u8`, 要创建的目录的名称。如果`path`是相对路径,则它是相对于`dirfd`目录而言的。如果`path`是相对路径,且`dirfd`的值为`AT_FDCWD`,则它是相对于当前路径而言的。如果`path`是绝对路径,则`dirfd`被忽略。
/// * `mode: u32`, 文件的所有权描述。详见`man 7 inode`。
/// 返回值:成功执行,返回0。失败,返回-1。
pub fn syscall_mkdirat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:切换工作目录；
/// # Arguments
/// * `path: *const u8`, 需要切换到的目录。
/// # Return
/// 成功执行:返回0。失败, 返回-1。
pub fn syscall_chdir(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// To get the `dirent` structures from the directory referred to by the open file descriptor `fd` into the buffer
/// # Arguments
/// * `fd: usize`, the file descriptor of the directory to be read
/// * `buf: *mut u8`, the buffer to store the `dirent` structures
/// * `len: usize`, the size of the buffer
///
/// # Return
/// * On success, the number of bytes read is returned. On end of directory, 0 is returned.
/// * On error, -1 is returned.
pub fn syscall_getdents64(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 276
/// 重命名文件或目录
// `todo!`
// 1. 权限检查
// 调用进程必须对源目录和目标目录都有写权限,才能完成重命名。
// 2. 目录和文件在同一个文件系统
// 如果目录和文件不在同一个文件系统,重命名会失败。`renameat2`不能跨文件系统重命名。
// 3. 源文件不是目标目录的子目录
// 如果源文件是目标目录的子孙目录,也会导致重命名失败。不能将目录重命名到自己的子目录中。
// 4. 目标名称不存在
// 目标文件名在目标目录下必须不存在,否则会失败。
// 5. 源文件被打开
// 如果源文件正被进程打开,默认情况下重命名也会失败。可以通过添加`RENAME_EXCHANGE`标志位实现原子交换。
// 6. 目录不是挂载点
// 如果源目录是一个挂载点,也不允许重命名。
/// # Arguments
/// * `old_dirfd: usize`, 旧文件所在的目录的文件描述符。
/// * `old_path: *const u8`, 旧文件的名称。如果`old_path`是相对路径,则它是相对于`old_dirfd`目录而言的。如果`old_path`是相对路径,且`old_dirfd`的值为`AT_FDCWD`,则它是相对于当前路径而言的。如果`old_path`是绝对路径,则`old_dirfd`被忽略。
/// * `new_dirfd: usize`, 新文件所在的目录的文件描述符。
/// * `new_path: *const u8`, 新文件的名称。如果`new_path`是相对路径,则它是相对于`new_dirfd`目录而言的。如果`new_path`是相对路径,且`new_dirfd`的值为`AT_FDCWD`,则它是相对于当前路径而言的。如果`new_path`是绝对路径,则`new_dirfd`被忽略。
/// * `flags: usize`, 重命名的标志位。目前只支持`RENAME_NOREPLACE`、`RENAME_EXCHANGE`和`RENAME_WHITEOUT`。
pub fn syscall_renameat2(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// # Arguments
/// * `fd: usize`
/// * `cmd: usize`
/// * `arg: usize`
pub fn syscall_fcntl64(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let cmd = args[1];
    let arg = args[2];
    let mut fd_table = FDM.fd_table.lock();

    if fd >= fd_table.len() {
        debug!("fd {} is out of range", fd);
        return Err(SyscallError::EBADF);
    }
    if fd_table[fd].is_none() {
        debug!("fd {} is none", fd);
        return Err(SyscallError::EBADF);
    }
    let file = fd_table[fd].clone().unwrap();
    info!("fd: {}, cmd: {}", fd, cmd);
    match Fcntl64Cmd::try_from(cmd) {
        Ok(Fcntl64Cmd::F_DUPFD) => {
            let new_fd = if let Ok(fd) = alloc_fd(&mut fd_table) {
                fd
            } else {
                // 文件描述符达到上限了
                return Err(SyscallError::EMFILE);
            };
            fd_table[new_fd] = fd_table[fd].clone();
            Ok(new_fd as isize)
        }
        Ok(Fcntl64Cmd::F_GETFD) => {
            if file.get_status().contains(OpenFlags::CLOEXEC) {
                Ok(1)
            } else {
                Ok(0)
            }
        }
        Ok(Fcntl64Cmd::F_SETFD) => {
            if file.set_close_on_exec((arg & 1) != 0) {
                Ok(0)
            } else {
                Err(SyscallError::EINVAL)
            }
        }
        Ok(Fcntl64Cmd::F_GETFL) => Ok(file.get_status().bits() as isize),
        Ok(Fcntl64Cmd::F_SETFL) => {
            if let Some(flags) = OpenFlags::from_bits(arg as u32) {
                if file.set_status(flags) {
                    return Ok(0);
                }
            }
            Err(SyscallError::EINVAL)
        }
        Ok(Fcntl64Cmd::F_DUPFD_CLOEXEC) => {
            let new_fd = if let Ok(fd) = alloc_fd(&mut fd_table) {
                fd
            } else {
                // 文件描述符达到上限了
                return Err(SyscallError::EMFILE);
            };

            if file.set_close_on_exec((arg & 1) != 0) {
                fd_table[new_fd] = fd_table[fd].clone();
                Ok(new_fd as isize)
            } else {
                Err(SyscallError::EINVAL)
            }
        }
        _ => Err(SyscallError::EINVAL),
    }
}

/// 53
/// 修改文件权限
/// mode: 0o777, 3位八进制数字
/// path为相对路径:
///     1. 若`dir_fd`为`AT_FDCWD`,则相对于当前工作目录
///     2. 若`dir_fd`为`AT_FDCWD`以外的值,则相对于`dir_fd`所指的目录
/// path为绝对路径:
///     忽视`dir_fd`,直接根据`path`访问
/// # Arguments
/// * `dir_fd: usize`, 目录的文件描述符
/// * `path: *const u8`, 文件的路径
/// * `mode: usize`, 文件的权限
pub fn syscall_fchmodat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 48
/// 获取文件权限
/// 类似上面的`fchmodat`
///        The mode specifies the accessibility `check(s)` to be performed,
///        and is either the value F_OK, or a mask consisting of the `bitwise`
///        OR of one or more of R_OK, W_OK, and X_OK. F_OK tests for the
///        existence of the file. R_OK, W_OK, and X_OK test whether the
///        file exists and grants read, write, and execute permissions,
///        respectively.
/// 0: F_OK, 1: X_OK, 2: W_OK, 4: R_OK
/// # Arguments
/// * `dir_fd: usize`, 目录的文件描述符
/// * `path: *const u8`, 文件的路径
/// * `mode: usize`, 文件的权限
pub fn syscall_faccessat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 29
/// 执行各种设备相关的控制功能
/// `todo`: 未实现
/// # Arguments
/// * `fd: usize`, 文件描述符
/// * `request: usize`, 控制命令
/// * `argp: *mut usize`, 参数
pub fn syscall_ioctl(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 88
/// 用于修改文件或目录的时间戳(timestamp)
/// 如果 `fir_fd < 0`,它和 `path` 共同决定要找的文件；
/// 如果 `fir_fd >=0`,它就是文件对应的 `fd`
/// # Arguments
/// * `dir_fd: usize`, 目录的文件描述符
/// * `path: *const u8`, 文件的路径
/// * `times: *const TimeSecs`, 时间戳
/// * `flags: usize`, 选项
pub fn syscall_utimensat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
