use axerrno::AxError;
use axfs::api::metadata;
use axlog::{debug, error, info};

use crate::{
    linux_env::{
        axfs_ext::api::{FileIOType, Kstat},
        linux_api::{
            link::{AT_FDCWD, FilePath, raw_ptr_to_ref_str},
            utils::{UtilsError, deal_path},
        },
        process_ext::api::current_process,
    },
    syscall::{
        FSTATATFlags, FsStat, SyscallError, SyscallResult, get_fs_stat,
        syscall_fs::ctype::mount::get_stat_in_fs,
    },
};

// FIX: 未测试
/// 实现 stat 系列系统调用
/// # Arguments
/// * `fd - usize`
/// * `kst - *mut Kstat`
pub fn syscall_fstat(args: [usize; 6]) -> SyscallResult {
    let fd = args[0];
    let kst = args[1] as *mut Kstat;
    let process = current_process();
    let fd_table = process.fd_manager.fd_table.lock();

    if fd >= fd_table.len() || fd < 3 {
        debug!("fd {} is out of range", fd);
        return Err(SyscallError::EPERM);
    }
    if fd_table[fd].is_none() {
        debug!("fd {} is none", fd);
        return Err(SyscallError::EPERM);
    }
    let file = fd_table[fd].clone().unwrap();
    if file.get_type() != FileIOType::FileDesc {
        debug!("fd {} is not a file", fd);
        return Err(SyscallError::EPERM);
    }

    match file.get_stat() {
        Ok(stat) => {
            unsafe {
                *kst = stat;
            }
            Ok(0)
        }
        Err(e) => {
            debug!("get stat error: {:?}", e);
            Err(SyscallError::EPERM)
        }
    }
}

// FIX: 未测试
/// 获取文件状态信息，但是给出的是目录`fd`和相对路径。
/// # Arguments
/// * `dir_fd - usize`
/// * `path - *const u8`
/// * `kst - *mut Kstat`
pub fn syscall_fstatat(args: [usize; 6]) -> SyscallResult {
    let dir_fd = args[0];
    let path = args[1] as *const u8;
    let kst = args[2] as *mut Kstat;
    let flags = args[3];

    let flags = if let Some(ans) = FSTATATFlags::from_bits(flags as u32) {
        ans
    } else {
        debug!("EINVAL An invalid flag was specified in flags.");
        return Err(SyscallError::EINVAL);
    };

    let file_path = match deal_path(dir_fd, Some(path), false) {
        Ok(path) => path,
        Err(e) => match e {
            UtilsError::CannotAcce | UtilsError::NULL => return Err(SyscallError::EFAULT),
            UtilsError::StrTooLong => return Err(SyscallError::ENAMETOOLONG),
            UtilsError::OutOfTable | UtilsError::NoEntryInTable => return Err(SyscallError::EBADF),
            UtilsError::StrEmpty => {
                if !flags.contains(FSTATATFlags::FSTATAT_EMPTY_PATH) {
                    return Err(SyscallError::ENOENT);
                } else {
                    // ```
                    // x86 下应用会调用 newfstatat(1, "", {st_mode=S_IFCHR|0620, st_rdev=makedev(0x88, 0xe), ...}, AT_EMPTY_PATH) = 0
                    // 去尝试检查 STDOUT 的属性。这里暂时先特判，以后再改成真正的 stdout 的属性
                    let path = unsafe { raw_ptr_to_ref_str(path) };
                    if path.is_empty() && dir_fd == 1 {
                        unsafe {
                            (*kst).st_mode = 0o20000 | 0o220u32;
                            (*kst).st_ino = 1;
                            (*kst).st_nlink = 1;
                        }
                        return Ok(0);
                    }
                    panic!("Wrong path at syscall_fstatat: {}(dir_fd={})", path, dir_fd);
                }
            }
            UtilsError::PanicMe => {
                panic!("{:?}", e);
            }
            _ => {
                panic!("{:?}", e);
            }
        },
    };

    info!("path : {}", file_path.path());

    match metadata(file_path.path()) {
        Ok(_) => {}
        Err(e) => match e {
            AxError::NotFound => {
                debug!("link src file not exists");
                return Err(SyscallError::ENOENT);
            }
            _ => {
                panic!("Unexpected error {:?}", e);
            }
        },
    }

    match get_stat_in_fs(&file_path) {
        Ok(stat) => unsafe {
            *kst = stat;
            Ok(0)
        },
        Err(error_no) => {
            debug!("get stat error: {:?}", error_no);
            Err(error_no)
        }
    }
}

// FIX: 测试
/// 获取文件系统的信息
/// # Arguments
/// * `path - *const u8`
/// * `stat - *mut FsStat`
pub fn syscall_statfs(args: [usize; 6]) -> SyscallResult {
    let path = args[0] as *const u8;
    let stat = args[1] as *mut FsStat;
    let file_path = match deal_path(AT_FDCWD, Some(path), false) {
        Ok(path) => path,
        Err(e) => match e {
            UtilsError::CannotAcce | UtilsError::NULL => return Err(SyscallError::EFAULT),
            UtilsError::StrTooLong => return Err(SyscallError::ENAMETOOLONG),
            UtilsError::OutOfTable | UtilsError::NoEntryInTable => return Err(SyscallError::EBADF),
            UtilsError::StrEmpty => return Err(SyscallError::ENOENT),
            UtilsError::PanicMe => {
                panic!("{:?}", e);
            }
            _ => {
                panic!("{:?}", e);
            }
        },
    };
    if file_path.equal_to(&FilePath::new("/").unwrap()) {
        // 目前只支持访问根目录文件系统的信息
        unsafe {
            *stat = get_fs_stat();
        }

        Ok(0)
    } else {
        error!("Only support fs_stat for root");
        Err(SyscallError::EINVAL)
    }
}
