use super::{
    syscall_fs::{FsSyscallId, fs_syscall},
    syscall_mem::{MemSyscallId, mem_syscall},
    syscall_task::{TaskSyscallId, task_syscall},
};
use axlog::info;

use super::{SyscallResult, deal_result};

#[unsafe(no_mangle)]
pub fn syscall(syscall_id: usize, args: [usize; 6]) -> isize {
    #[allow(unused_mut, unused_assignments)]
    let mut ans: Option<SyscallResult> = None;

    if let Ok(mem_syscall_id) = MemSyscallId::try_from(syscall_id) {
        info!(
            "[syscall] id = {:#?}, args = {:?}, entry",
            mem_syscall_id, args
        );
        #[allow(unused_assignments)]
        ans = Some(mem_syscall(mem_syscall_id, args));
    }

    if let Ok(fs_syscall_id) = FsSyscallId::try_from(syscall_id) {
        info!(
            "[syscall] id = {:#?}, args = {:?}, entry",
            fs_syscall_id, args
        );
        #[allow(unused_assignments)]
        ans = Some(fs_syscall(fs_syscall_id, args));
    }

    if let Ok(task_syscall_id) = TaskSyscallId::try_from(syscall_id) {
        info!(
            "[syscall] id = {:#?}, args = {:?}, entry",
            task_syscall_id, args
        );
        #[allow(unused_assignments)]
        ans = Some(task_syscall(task_syscall_id, args));
    }

    if ans.is_none() {
        panic!("unknown syscall id: {}", syscall_id);
    }
    let ans = deal_result(ans.unwrap());
    if syscall_id != 96 && syscall_id != 98 {
        info!(
            "[syscall] id = {}, args = {:?}, return {}",
            syscall_id, args, ans
        );
    }
    ans
}
