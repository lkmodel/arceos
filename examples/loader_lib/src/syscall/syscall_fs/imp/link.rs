use crate::{
    linux_env::linux_fs::link::{FilePath, deal_with_path, remove_link},
    syscall::{SyscallError, SyscallResult},
};
use axfs::api::remove_dir;
use axlog::debug;

pub const AT_REMOVEDIR: usize = 0x200; // Remove directory instead of `unlinking` file.
/// 功能:创建文件的链接；
/// # Arguments
/// * `old_dir_fd: usize`, 原来的文件所在目录的文件描述符。
/// * `old_path: *const u8`, 文件原来的名字。如果`old_path`是相对路径,则它是相对于`old_dir_fd`目录而言的。如果`old_path`是相对路径,且`old_dir_fd`的值为`AT_FDCWD`,则它是相对于当前路径而言的。如果`old_path`是绝对路径,则`old_dir_fd`被忽略。
/// * `new_dir_fd: usize`, 新文件名所在的目录。
/// * `new_path: *const u8`, 文件的新名字。`new_path`的使用规则同`old_path`。
/// * `flags: usize`, 在2.6.18内核之前,应置为0。其它的值详见`man 2 linkat`。
/// # Return
/// 成功执行,返回0。失败,返回-1。
#[allow(dead_code)]
pub fn sys_linkat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:移除指定文件的链接(可用于删除文件);
/// # Arguments
/// * `dir_fd: usize`, 要删除的链接所在的目录。
/// * `path: *const u8`, 要删除的链接的名字。如果`path`是相对路径,则它是相对于`dir_fd`目录而言的。如果`path`是相对路径,且`dir_fd`的值为`AT_FDCWD`,则它是相对于当前路径而言的。如果`path`是绝对路径,则`dir_fd`被忽略。
/// * `flags: usize`, 可设置为0或`AT_REMOVEDIR`。
/// # Return
/// 成功执行,返回0。失败,返回-1。
pub fn syscall_unlinkat(args: [usize; 6]) -> SyscallResult {
    let dir_fd = args[0];
    let path = args[1] as *const u8;
    let flags = args[2];
    let path = deal_with_path(dir_fd, Some(path), false).unwrap();

    if path.start_with(&FilePath::new("/proc").unwrap()) {
        return Ok(-1);
    }

    // `Unlink` file
    if flags == 0 {
        if remove_link(&path).is_none() {
            debug!("unlink file error");
            return Err(SyscallError::EINVAL);
        }
    }
    // Remove `dir`
    else if flags == AT_REMOVEDIR {
        if let Err(e) = remove_dir(path.path()) {
            debug!("rmdir error: {:?}", e);
            return Err(SyscallError::EINVAL);
        }
    }
    // Flags error
    else {
        debug!("flags error");
        return Err(SyscallError::EINVAL);
    }
    Ok(0)
}
