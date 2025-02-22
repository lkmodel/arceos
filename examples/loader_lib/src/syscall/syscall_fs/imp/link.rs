use std::string::ToString;

use crate::{
    linux_env::linux_fs::{
        link::{FilePath, new_link, remove_link},
        utils::{UtilsError, deal_path},
    },
    syscall::{SyscallError, SyscallResult, UnlinkatFlags},
};
use axerrno::AxError;
use axfs::api::{current_dir, metadata, remove_dir};
use axlog::{debug, warn};

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
pub fn sys_linkat(args: [usize; 6]) -> SyscallResult {
    let old_dir_fd = args[0];
    let old_path = args[1] as *const u8;
    let new_dir_fd = args[2];
    let new_path = args[3] as *const u8;
    let _flags = args[4];

    let old_path = match deal_path(old_dir_fd, Some(old_path), false) {
        Ok(path) => path,
        Err(e) => match e {
            UtilsError::NULL | UtilsError::CannotAcce => return Err(SyscallError::EFAULT),
            UtilsError::StrTooLong => return Err(SyscallError::ENAMETOOLONG),
            UtilsError::InvalidArg => return Err(SyscallError::ENOTDIR),
            UtilsError::OutOfTable | UtilsError::NoEntryInTable => return Err(SyscallError::EBADF),
            UtilsError::PanicMe => {
                panic!("{:?}", e);
            }
            _ => {
                panic!("{:?}", e);
            }
        },
    };
    let new_path = match deal_path(new_dir_fd, Some(new_path), false) {
        Ok(path) => path,
        Err(e) => match e {
            UtilsError::NULL | UtilsError::CannotAcce => return Err(SyscallError::EFAULT),
            UtilsError::StrTooLong => return Err(SyscallError::ENAMETOOLONG),
            UtilsError::InvalidArg => return Err(SyscallError::ENOTDIR),
            UtilsError::OutOfTable | UtilsError::NoEntryInTable => return Err(SyscallError::EBADF),
            UtilsError::PanicMe => {
                panic!("{:?}", e);
            }
            _ => {
                panic!("{:?}", e);
            }
        },
    };

    match metadata(old_path.path()) {
        Ok(_) => {}
        Err(e) => match e {
            AxError::NotFound => {
                debug!(
                    "ENOENT A directory component in oldpath or newpath does not exist or is a dangling symbolic link."
                );
                return Err(SyscallError::ENOENT);
            }
            _ => {
                warn!("Unexpect errno catched {:?}", e);
                return Err(SyscallError::EPERM);
            }
        },
    }

    match metadata(new_path.path()) {
        Ok(_) => return Err(SyscallError::EEXIST),
        Err(e) => match e {
            AxError::NotFound => {
                if new_link(&new_path, &old_path) {
                    Ok(0)
                } else {
                    Err(SyscallError::EINVAL)
                }
            }
            _ => {
                warn!("Unexpect errno catched {:?}", e);
                return Err(SyscallError::EPERM);
            }
        },
    }
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
    let path = match deal_path(dir_fd, Some(path), false) {
        Ok(t) => t,
        Err(e) => match e {
            UtilsError::NULL | UtilsError::CannotAcce => return Err(SyscallError::EFAULT),
            UtilsError::StrTooLong => return Err(SyscallError::ENAMETOOLONG),
            UtilsError::InvalidArg => return Err(SyscallError::ENOTDIR),
            UtilsError::OutOfTable | UtilsError::NoEntryInTable => return Err(SyscallError::EBADF),
            UtilsError::PanicMe => {
                panic!("{:?}", e);
            }
            _ => {
                panic!("{:?}", e);
            }
        },
    };

    let flags = if let Some(ans) = UnlinkatFlags::from_bits(flags as u32) {
        ans
    } else {
        debug!("Err: EINVAL An invalid flag was specified in flags.");
        return Err(SyscallError::EINVAL);
    };

    // FIX: 这里可能出错
    if path.start_with(&FilePath::new("/proc").unwrap()) {
        return Ok(-1);
    }

    let metadata = match metadata(path.path()) {
        Ok(t) => t,
        Err(e) => match e {
            AxError::NotFound => {
                debug!("A component in pathname does not exist or is a dangling symbolic link");
                return Err(SyscallError::ENOENT);
            }
            _ => {
                warn!("Unexpect errno catched {:?}", e);
                return Err(SyscallError::EPERM);
            }
        },
    };

    let cwd = current_dir().unwrap();
    if cwd == path.path().to_string() {
        return Err(SyscallError::EPERM);
    }

    // `Unlink` file
    if flags.is_empty() {
        if metadata.is_dir() {
            debug!(
                "Err: EISDIR pathname refers to a directory, and AT_REMOVEDIR was not specified in flags."
            );
            return Err(SyscallError::EISDIR);
        }
        if remove_link(&path).is_none() {
            warn!("unlink file error");
            return Err(SyscallError::EINVAL);
        }
    }
    // Remove `dir`
    else if flags.contains(UnlinkatFlags::AT_REMOVEDIR) {
        if let Err(e) = remove_dir(path.path()) {
            warn!("rmdir error: {:?}", e);
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
