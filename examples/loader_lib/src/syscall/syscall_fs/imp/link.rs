use crate::syscall::SyscallResult;

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
pub fn syscall_unlinkat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
