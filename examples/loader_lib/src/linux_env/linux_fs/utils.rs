use crate::linux_env::{
    axfs_ext::api::FileIOType,
    linux_fs::{
        fd_manager::FDM,
        link::{AT_FDCWD, FilePath, raw_ptr_to_ref_str},
    },
};
use alloc::{format, string::ToString};
use axfs::api::Permissions;
use axlog::debug;

/// The error type used by `utils`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UtilsError {
    /// The pointer is NULL.
    /// 指针为空
    NULL = 1,
    /// The string passed is empty.
    /// 字符串为空
    StrEmpty,
    /// The address is out of accessible space.
    /// 访问非法地址
    CannotAcce,
    /// The path name, filename etc is too long.
    /// 字符串过长
    StrTooLong,
    /// Table index out of the table.
    /// 超出查找表
    OutOfTable,
    /// Find no entry in a table etc.
    /// 表中无对应表项
    NoEntryInTable,
    /// Permission Denied.
    /// 权限错误
    PermDenied,
    /// Something cannot be found.
    /// 无法找到某个东西
    NotFound,
    /// Other invalid parameters not marked by UtilsError
    /// 非UtilsError标出的其他非法参数
    InvalidArg,
    /// Error that never should happened.
    /// 一个不应当出现的结果、错误发生了。
    PanicMe,
}

/// A specialized [`Result`] type with [`UtilsError`] as the error type.
pub type UtilsResult<T = ()> = Result<T, UtilsError>;

const FILE_NAME_LENGTH: usize = 255usize;

/// To handle common file or directory paths and encapsulates them into a `UtilsError`.
/// Under normal circumstances, it converts each provided address into a standardized path in the form of an absolute address.
///
/// * `dir_fd` - The file descriptor of the directory, if it is AT_FDCWD, the call operates on the current working directory
///
/// * `path_addr` - The address of the path, if it is null or an empty string, `AxError::InvalidInput` will be returned
///
/// * `force_dir` - If true, the path will be treated as a directory
///
/// # Returns
///
/// Only return follows:
///
pub fn deal_path(
    dir_fd: usize,
    path_addr: Option<*const u8>,
    force_dir: bool,
) -> UtilsResult<FilePath> {
    let mut path = "".to_string();
    if let Some(path_addr) = path_addr {
        if path_addr.is_null() {
            return Err(UtilsError::NULL);
        }
        const START_ADDR: usize = 0xffffffc080100000;
        const END_ADDR: usize = 0xffffffc0ffffffff;
        // FIX: 检查指针是否被分配，检查是不是在范围内，并返回错误
        if (path_addr as usize).lt(&START_ADDR) || (path_addr as usize).ge(&END_ADDR) {
            return Err(UtilsError::CannotAcce);
        }
        path = unsafe { raw_ptr_to_ref_str(path_addr) }.to_string().clone();
    }

    if path.is_empty() {
        return Err(UtilsError::StrEmpty);
    } else if path.len() >= FILE_NAME_LENGTH {
        return Err(UtilsError::StrTooLong);
    } else if !path.starts_with('/') && dir_fd != AT_FDCWD && dir_fd as u32 != AT_FDCWD as u32 {
        // 如果不是绝对路径, 且dir_fd不是AT_FDCWD, 则需要将dir_fd和path拼接起来
        let fd_table = FDM.fd_table.lock();
        if dir_fd >= fd_table.len() {
            debug!(
                "dir_fd out of the fd_table bound.(pathname is relative but dirfd is neither AT_FDCWD nor a valid file descriptor)return EBADF"
            );
            return Err(UtilsError::OutOfTable);
        }
        match fd_table[dir_fd].as_ref() {
            Some(dir) => {
                if dir.get_type() != FileIOType::DirDesc {
                    debug!("selected fd {} is not a dir, return ENOTDIR", dir_fd);
                    return Err(UtilsError::InvalidArg);
                }
                let dir = dir.clone();
                path = format!("{}{}", dir.get_path(), path);
            }
            None => {
                return Err(UtilsError::NoEntryInTable);
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
        Ok(path) => Ok(path),
        Err(_) => Err(UtilsError::PanicMe),
    }
}

pub fn has_permission(mode: Permissions, perm: Permissions) -> bool {
    if (mode.contains(Permissions::OWNER_WRITE) && !perm.contains(Permissions::OWNER_WRITE))
        || (mode.contains(Permissions::OWNER_READ) && !perm.contains(Permissions::OWNER_READ))
        || (mode.contains(Permissions::OWNER_EXEC) && !perm.contains(Permissions::OWNER_EXEC))
    {
        return false;
    }
    if (mode.contains(Permissions::GROUP_WRITE) && !perm.contains(Permissions::GROUP_WRITE))
        || (mode.contains(Permissions::GROUP_READ) && !perm.contains(Permissions::GROUP_READ))
        || (mode.contains(Permissions::GROUP_EXEC) && !perm.contains(Permissions::GROUP_EXEC))
    {
        return false;
    }
    if (mode.contains(Permissions::OTHER_WRITE) && !perm.contains(Permissions::OTHER_WRITE))
        || (mode.contains(Permissions::OTHER_READ) && !perm.contains(Permissions::OTHER_READ))
        || (mode.contains(Permissions::OTHER_EXEC) && !perm.contains(Permissions::OTHER_EXEC))
    {
        return false;
    }
    true
}
