use crate::{
    linux_env::{
        axfs_ext::api::FileIOType,
        linux_fs::{
            fd_manager::FDM,
            link::{AT_FDCWD, FilePath, raw_ptr_to_ref_str},
        },
    },
    syscall::SyscallError,
};
use alloc::{format, string::ToString};
use axerrno::AxError;
use axlog::{debug, warn};

/// To handle common Linux system call errors that occur when passing file or directory paths and encapsulates them into a `SyscallError`, then set `FilePath` value as `None`.
/// Under normal circumstances, it converts each provided address into a standardized path in the form of an absolute address that warped in `Some`. In this case, `SyscallError` should be ignored.
///
/// * `dir_fd` - The file descriptor of the directory, if it is AT_FDCWD, the call operates on the current working directory
///
/// * `path_addr` - The address of the path, if it is null or an empty string, `AxError::InvalidInput` will be returned
///
/// * `force_dir` - If true, the path will be treated as a directory
pub fn deal_path_linstyle(
    dir_fd: usize,
    path_addr: Option<*const u8>,
    force_dir: bool,
) -> (Option<FilePath>, SyscallError) {
    let mut path = "".to_string();
    if let Some(path_addr) = path_addr {
        if path_addr.is_null() {
            debug!("path address is null(out of accessable space), return EFAULT");
            return (None, SyscallError::EFAULT);
        }
        // FIX: 检查指针是否被分配，检查是不是在范围内，并返回错误
        path = unsafe { raw_ptr_to_ref_str(path_addr) }.to_string().clone();
    }

    if path.is_empty() {
        debug!("path address is empty, return EINVAL");
        return (None, SyscallError::EINVAL);
    } else if !path.starts_with('/') && dir_fd != AT_FDCWD && dir_fd as u32 != AT_FDCWD as u32 {
        // 如果不是绝对路径, 且dir_fd不是AT_FDCWD, 则需要将dir_fd和path拼接起来
        let fd_table = FDM.fd_table.lock();
        if dir_fd >= fd_table.len() {
            debug!("dir_fd out of the fd_table bound. return EBADF");
            return (None, SyscallError::EBADF);
        }
        match fd_table[dir_fd].as_ref() {
            Some(dir) => {
                if dir.get_type() != FileIOType::DirDesc {
                    debug!("selected fd {} is not a dir, return ENOTDIR", dir_fd);
                    return (None, SyscallError::ENOTDIR);
                }
                let dir = dir.clone();
                path = format!("{}{}", dir.get_path(), path);
            }
            None => {
                debug!("fd not exist in fd_table, return ENOENT");
                return (None, SyscallError::ENOENT);
            }
        }
    }

    if force_dir && !path.ends_with('/') {
        path = format!("{}/", path);
    }
    if path.ends_with('.') {
        // 如果path以`.`或`..`结尾, 则加上/告诉FilePath::new它是一个目录
        path = format!("{}/", path);
    }
    match FilePath::new(path.as_str()) {
        Ok(path) => (Some(path), SyscallError::EPERM),
        Err(e) => match e {
            AxError::NotFound => {
                warn!(
                    "error when creating FilePath: {:?}, maybe you need check path exists first?",
                    e
                );
                (None, SyscallError::ENOENT)
            }
            _ => {
                warn!("error when creating FilePath: {:?}", e);
                (None, SyscallError::EPERM)
            }
        },
    }
}
