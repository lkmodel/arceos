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

/// 获取对应任务的CPU适配集
///
/// 若pid是进程ID，则获取对应的进程的主线程的信息
///
/// 若pid是线程ID，则获取对应线程信息
///
/// 若pid为0，则获取当前运行任务的信息
///
/// mask为即将写入的cpu set的地址指针
/// # Arguments
/// * `pid` - usize
/// * `cpu_set_size` - usize
/// * `mask` - *mut usize
pub fn syscall_sched_getaffinity(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //    let pid = args[0];
    //    let cpu_set_size = args[1];
    //    let mask = args[2] as *mut usize;
    //    // let task: LazyInit<AxTaskRef> = LazyInit::new();
    //    let tid2task = TID2TASK.lock();
    //    let pid2task = PID2PC.lock();
    //    let pid = pid as u64;
    //    let task = if tid2task.contains_key(&pid) {
    //        Arc::clone(tid2task.get(&pid).unwrap())
    //    } else if pid2task.contains_key(&pid) {
    //        let process = pid2task.get(&pid).unwrap();
    //
    //        process
    //            .tasks
    //            .lock()
    //            .iter()
    //            .find(|task| task.is_leader())
    //            .map(Arc::clone)
    //            .unwrap()
    //    } else if pid == 0 {
    //        Arc::clone(current_task().as_task_ref())
    //    } else {
    //        // 找不到对应任务
    //        return Err(SyscallError::ESRCH);
    //    };
    //
    //    drop(pid2task);
    //    drop(tid2task);
    //
    //    let process = current_process();
    //    if process
    //        .manual_alloc_for_lazy(VirtAddr::from(mask as usize))
    //        .is_err()
    //    {
    //        return Err(SyscallError::EFAULT);
    //    }
    //    let cpu_set = task.get_cpu_set();
    //    let mut prev_mask = unsafe { *mask };
    //    let len = SMP.min(cpu_set_size * 4);
    //    prev_mask &= !((1 << len) - 1);
    //    prev_mask &= cpu_set & ((1 << len) - 1);
    //    unsafe {
    //        *mask = prev_mask;
    //    }
    //    // 返回成功填充的缓冲区的长度
    //    Ok(SMP as isize)
}
