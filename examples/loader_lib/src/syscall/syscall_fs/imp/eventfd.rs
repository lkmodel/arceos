use crate::{
    linux_env::linux_fs::api::UNI_API,
    syscall::{SyscallError, SyscallResult, syscall_fs::ctype::eventfd::create_eventfd},
};

pub fn syscall_eventfd(args: [usize; 6]) -> SyscallResult {
    let initval = args[0] as u64;
    let flags = args[1] as u32;

    let mut fd_table = UNI_API.fd_manager.fd_table.lock();
    let fd_num = if let Ok(fd) = UNI_API.alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EPERM);
    };

    fd_table[fd_num] = Some(create_eventfd(initval, flags));
    Ok(fd_num as isize)
}
