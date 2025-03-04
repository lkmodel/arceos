use crate::{
    linux_env::{
        axfs_ext::api::{FileIO, FileIOType, Kstat, OpenFlags, SeekFrom},
        linux_fs::{
            fd_manager::{FDM, alloc_fd},
            link::{FilePath, create_link},
            utils::{UtilsError, deal_path, has_permission},
        },
    },
    syscall::{
        IoVec, O_CLOEXEC, StMode, SyscallError, SyscallResult, normal_file_mode,
        syscall_fs::ctype::{
            dir::{get_dir_desc, new_dir},
            epoll::{EpollCtl, EpollEvent, EpollEventType, EpollFile},
            file::{new_fd, new_inode},
        },
    },
};
use alloc::{string::ToString, sync::Arc, vec};
use axerrno::AxError;
use axfs::api::{Permissions, lookup};
use axlog::{debug, error, info, warn};
use core::slice::{from_raw_parts, from_raw_parts_mut};

/// # Arguments
/// * `signum` - usize
/// * `action` - *const SigAction
/// * `old_action` - *mut SigAction
pub fn syscall_sigaction(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //    let signum = args[0];
    //    let action = args[1] as *const SigAction;
    //    let old_action = args[2] as *mut SigAction;
    //    info!(
    //        "signum: {}, action: {:X}, old_action: {:X}",
    //        signum, action as usize, old_action as usize
    //    );
    //    if signum == SignalNo::SIGKILL as usize || signum == SignalNo::SIGSTOP as usize {
    //        // 特殊参数不能被覆盖
    //        return Err(SyscallError::EPERM);
    //    }
    //
    //    let current_process = current_process();
    //    let mut signal_modules = current_process.signal_modules.lock();
    //    let signal_module = signal_modules
    //        .get_mut(&current_task().id().as_u64())
    //        .unwrap();
    //    let mut signal_handler = signal_module.signal_handler.lock();
    //    let old_address = old_action as usize;
    //
    //    if old_address != 0 {
    //        // old_address非零说明要求写入到这个地址
    //        // 此时要检查old_address是否在某一个段中
    //        if current_process
    //            .manual_alloc_for_lazy(old_address.into())
    //            .is_err()
    //        {
    //            // 无法分配
    //            return Err(SyscallError::EPERM);
    //        }
    //        if let Some(action) = signal_handler.get_action(signum) {
    //            // 将原有的action存储到old_address
    //            unsafe {
    //                *old_action = *action;
    //            }
    //        }
    //    }
    //
    //    let new_address = action as usize;
    //    if new_address != 0 {
    //        if current_process
    //            .manual_alloc_for_lazy(new_address.into())
    //            .is_err()
    //        {
    //            // 无法分配
    //            return Err(SyscallError::EPERM);
    //        }
    //        unsafe { signal_handler.set_action(signum, action) };
    //    }
    //    Ok(0)
}

/// # Arguments
/// * `flag` - SigMaskFlag
/// * `new_mask` - *const usize
/// * `old_mask` - *mut usize
/// * `sigsetsize` - usize, specifies the size in bytes of the signal sets in set and oldset, which is equal to sizeof(kernel_sigset_t)
pub fn syscall_sigprocmask(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //    let flag = SigMaskFlag::from(args[0]);
    //    let new_mask = args[1] as *const usize;
    //    let old_mask = args[2] as *mut usize;
    //    let sigsetsize = args[3];
    //    if sigsetsize != SIGSET_SIZE_IN_BYTE {
    //        // 若sigsetsize不是正确的大小，则返回错误
    //        return Err(SyscallError::EINVAL);
    //    }
    //
    //    let current_process = current_process();
    //    if old_mask as usize != 0
    //        && current_process
    //            .manual_alloc_for_lazy((old_mask as usize).into())
    //            .is_err()
    //    {
    //        return Err(SyscallError::EFAULT);
    //    }
    //    if new_mask as usize != 0
    //        && current_process
    //            .manual_alloc_for_lazy((new_mask as usize).into())
    //            .is_err()
    //    {
    //        return Err(SyscallError::EPERM);
    //    }
    //
    //    let mut signal_modules = current_process.signal_modules.lock();
    //    let signal_module = signal_modules
    //        .get_mut(&current_task().id().as_u64())
    //        .unwrap();
    //    if old_mask as usize != 0 {
    //        unsafe {
    //            *old_mask = signal_module.signal_set.mask;
    //        }
    //    }
    //
    //    if new_mask as usize != 0 {
    //        let now_mask = unsafe { *new_mask };
    //        match flag {
    //            SigMaskFlag::Block => {
    //                signal_module.signal_set.mask |= now_mask;
    //            }
    //            SigMaskFlag::Unblock => {
    //                signal_module.signal_set.mask &= !now_mask;
    //            }
    //            SigMaskFlag::Setmask => {
    //                signal_module.signal_set.mask = now_mask;
    //            }
    //        }
    //    }
    //    Ok(0)
}

/// 向tid指定的线程发送信号
/// # Arguments
/// * `tid` - isize
/// * `signum` - isize
pub fn syscall_tkill(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //    let tid = args[0] as isize;
    //    let signum = args[1] as isize;
    //    debug!(
    //        "cpu: {}, send singal: {} to: {}",
    //        this_cpu_id(),
    //        signum,
    //        tid
    //    );
    //    if tid > 0 && signum > 0 {
    //        let _ = axprocess::signal::send_signal_to_thread(tid, signum);
    //        Ok(0)
    //    } else {
    //        Err(SyscallError::EINVAL)
    //    }
}
