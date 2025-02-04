use crate::{
    linux_env::{
        axfs_ext::api::OpenFlags,
        linux_fs::{
            fd_manager::{FDM, alloc_fd},
            link::{create_link, deal_with_path},
        },
    },
    syscall::{
        SyscallError, SyscallResult,
        syscall_fs::ctype::{
            dir::new_dir,
            epoll::{EpollCtl, EpollEvent, EpollEventType, EpollFile},
            file::{new_fd, new_inode},
        },
    },
};
use alloc::{string::ToString, sync::Arc};
use axlog::{debug, info};

/// 功能:打开或创建一个文件；
/// # Arguments
/// * `fd: usize`, 文件所在目录的文件描述符。
/// * `path: *const u8`, 要打开或创建的文件名。如为绝对路径,则忽略`fd`。如为相对路径,且`fd`是`AT_FDCWD`,则`filename`是相对于当前工作目录来说的。如为相对路径,且`fd`是一个文件描述符,则`filename`是相对于`fd`所指向的目录来说的。
/// * `flags: usize`, 必须包含如下访问模式的其中一种:`O_RDONLY`,`O_WRONLY`,`O_RDWR`。还可以包含文件创建标志和文件状态标志。
/// * `mode: u8`, 文件的所有权描述。详见`man 7 inode `。
/// 返回值:成功执行,返回新的文件描述符。失败,返回-1。
///
/// 说明:如果打开的是一个目录,那么返回的文件描述符指向的是该目录的描述符。(后面会用到针对目录的文件描述符)
/// `flags: O_RDONLY: 0, O_WRONLY: 1, O_RDWR: 2, O_CREAT: 64, O_DIRECTORY: 65536`
pub fn syscall_openat(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let path = args[1] as *const u8;
    let flags = args[2];
    let _mode = args[3] as u8;
    let force_dir = OpenFlags::from(flags).is_dir();
    let path = if let Some(path) = deal_with_path(fd, Some(path), force_dir) {
        path
    } else {
        return Err(SyscallError::EINVAL);
    };
    let mut fd_table = FDM.fd_table.lock();
    let fd_num: usize = if let Ok(fd) = alloc_fd(&mut fd_table) {
        fd
    } else {
        return Err(SyscallError::EMFILE);
    };
    debug!("allocated fd_num: {}", fd_num);
    // 分配`inode`
    new_inode(path.path().to_string()).unwrap();
    // 如果是`DIR`
    info!("path: {:?}", path.path());
    if path.is_dir() {
        debug!("open dir");
        if let Ok(dir) = new_dir(path.path().to_string(), flags.into()) {
            debug!("new dir_desc successfully allocated: {}", path.path());
            fd_table[fd_num] = Some(Arc::new(dir));
            Ok(fd_num as isize)
        } else {
            debug!("open dir failed");
            Err(SyscallError::ENOENT)
        }
    }
    // 如果是FILE,注意若创建了新文件,需要添加链接
    else {
        debug!("open file");
        if let Ok(file) = new_fd(path.path().to_string(), flags.into()) {
            debug!("new file_desc successfully allocated");
            fd_table[fd_num] = Some(Arc::new(file));
            let _ = create_link(&path, &path); // 不需要检查是否成功,因为如果成功,说明是新建的文件,如果失败,说明已经存在了
            Ok(fd_num as isize)
        } else {
            debug!("open file failed");
            Err(SyscallError::ENOENT)
        }
    }
}

/// 功能:关闭一个文件描述符；
/// # Arguments
/// * `fd: usize`, 要关闭的文件描述符。
/// 返回值:成功执行,返回0。失败,返回-1。
pub fn syscall_close(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    info!("Into syscall_close. fd: {}", fd);

    let mut fd_table = FDM.fd_table.lock();
    if fd >= fd_table.len() {
        debug!("fd {} is out of range", fd);
        return Err(SyscallError::EPERM);
    }
    // ```
    // if fd == 3 {
    //     debug!("fd {} is reserved for cwd", fd);
    //     return -1;
    // }
    if fd_table[fd].is_none() {
        debug!("fd {} is none", fd);
        return Err(SyscallError::EPERM);
    }
    // ```
    // let file = process_inner.fd_manager.fd_table[fd].unwrap();
    for i in 0..fd_table.len() {
        if let Some(file) = fd_table[i].as_ref() {
            if let Some(epoll_file) = file.as_any().downcast_ref::<EpollFile>() {
                if epoll_file.contains(fd as i32) {
                    let ev = EpollEvent {
                        event_type: EpollEventType::EPOLLMSG,
                        data: 0,
                    };
                    epoll_file.epoll_ctl(EpollCtl::DEL, fd as i32, ev)?;
                }
            }
        }
    }

    fd_table[fd] = None;
    // ```
    // for i in 0..process_inner.fd_table.len() {
    //     if let Some(file) = process_inner.fd_table[i].as_ref() {
    //         debug!("fd: {} has file", i);
    //     }
    // }

    Ok(0)
}

/// 功能:从一个文件描述符中读取；
/// # Arguments
/// * `fd: usize`, 要读取文件的文件描述符。
/// * `buf: *mut u8`, 一个缓存区,用于存放读取的内容。
/// * `count: usize`, 要读取的字节数。
/// 返回值:成功执行,返回读取的字节数。如为0,表示文件结束。错误,则返回-1。
pub fn syscall_read(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:从一个文件描述符中写入；
/// # Arguments:
/// * `fd: usize`, 要写入文件的文件描述符。
/// * `buf: *const u8`, 一个缓存区,用于存放要写入的内容。
/// * `count: usize`, 要写入的字节数。
/// 返回值:成功执行,返回写入的字节数。错误,则返回-1。
pub fn syscall_write(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:创建管道；
/// # Arguments
/// * `fd[2]: *mut u32`, 用于保存2个文件描述符。其中,`fd[0]`为管道的读出端,`fd[1]`为管道的写入端。
/// * `flags: usize`, 用于指定管道的属性。
/// 返回值:成功执行,返回0。失败,返回-1。
///
/// 注意:`fd[2]`是32位数组,所以这里的`fd`是 u32 类型的指针,而不是`usize`类型的指针。
pub fn syscall_pipe2(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:复制文件描述符；
/// # Arguments
/// * `fd: usize`, 被复制的文件描述符。
/// 返回值:成功执行,返回新的文件描述符。失败,返回-1。
pub fn syscall_dup(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 功能:复制文件描述符,并指定了新的文件描述符；
/// # Arguments
/// * `fd: usize`, 原文件所在的文件描述符
/// * `new_fd: usize`, 新的文件描述符
/// * `flags: usize`, 文件描述符标志
/// 返回值:成功执行,返回新的文件描述符。失败,返回-1。
pub fn syscall_dup3(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 从同一个文件描述符读取多个字符串
/// # Arguments
/// * `fd: usize`, 要读取文件的文件描述符。
/// * `iov: *mut IoVec`, 一个缓存区,用于存放读取的内容。
/// * `iov_cnt: usize`, 要读取的字节数。
pub fn syscall_readv(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 从同一个文件描述符写入多个字符串
/// # Arguments
/// * `fd: usize`, 要写入文件的文件描述符。
/// * `iov: *mut IoVec`, 一个缓存区,用于存放要写入的内容。
/// * `iov_cnt: usize`, 要写入的字节数。
pub fn syscall_writev(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 62
/// 移动文件描述符的读写指针
/// # Arguments
/// * `fd: usize`
/// * `offset: isize`
/// * `whence: usize`
pub fn syscall_lseek(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 67
/// `pread64`
/// 从文件的指定位置读取数据,并且不改变文件的读写指针
/// # Arguments
/// * `fd: usize`
/// * `buf: *mut u8`
/// * `count: usize`
/// * `offset: usize`
pub fn syscall_pread64(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 78
/// `readlinkat`
/// 读取符号链接文件的内容
/// * 如果`buf`为`NULL`,则返回符号链接文件的长度
/// * 如果`buf`不为`NULL`,则将符号链接文件的内容写入`buf`中
/// 如果写入的内容超出了`buf_size`则直接截断
/// # Arguments
/// * `dir_fd: usize`
/// * `path: *const u8`
/// * `buf: *mut u8`
/// * `bufsiz: usize`
pub fn syscall_readlinkat(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 68
/// `pwrite64`
/// 向文件的指定位置写入数据,并且不改变文件的读写指针
/// # Arguments
/// * `fd: usize`
/// * `buf: *const u8`
/// * `count: usize`
/// * `offset: usize`
pub fn syscall_pwrite64(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 71
/// `sendfile64`
/// 将一个文件的内容发送到另一个文件中
/// 如果`offset`为`NULL`,则从当前读写指针开始读取,读取完毕后会更新读写指针
/// 如果`offset`不为`NULL`,则从`offset`指定的位置开始读取,读取完毕后不会更新读写指针,但是会更新`offset`的值
/// # Arguments
/// * `out_fd: usize`
/// * `in_fd: usize`
/// * `offset: *mut usize`
/// * `count: usize`
pub fn syscall_sendfile64(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// # Arguments
/// * `fd: usize`
/// * `len: usize`
pub fn syscall_ftruncate64(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/**
该系统调用应复制文件描述符`fd_in`中的至多`len`个字节到文件描述符`fd_out`中。
若`off_in`为`NULL`,则复制时应从文件描述符`fd_in`本身的文件偏移处开始读取,并将其文件偏移增加成功复制的字节数；否则,从`*off_in`指定的文件偏移处开始读取,不改变`fd_in`的文件偏移,而是将`*off_in`增加成功复制的字节数。
参数`off_out`的行为类似:若`off_ou` 为`NULL`,则复制时从文件描述符`fd_out`本身的文件偏移处开始写入,并将其文件偏移增加成功复制的字节数；否则,从`*off_out`指定的文件偏移处开始写入,不改变`fd_out`的文件偏移,而是将`*off_out`增加成功复制的字节数。
该系统调用的返回值为成功复制的字节数,出现错误时返回负值。若读取`fd_in`时的文件偏移超过其大小,则直接返回`0`,不进行复制。
本题中,`fd_in`和`fd_out`总指向文件系统中两个不同的普通文件；`flags`总为`0`,没有实际作用。
 */
/// # Arguments
/// * `fd_in: usize`
/// * `off_in: *mut usize`
/// * `fd_out: usize`
/// * `off_out: *mut usize`
/// * `len: usize`
/// * `flags: usize`
pub fn syscall_copyfilerange(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
