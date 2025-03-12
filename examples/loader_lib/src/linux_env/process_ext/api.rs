use alloc::sync::Arc;
use axtask::{AxTaskRef, CurrentTask, TaskExtRef, current};

use crate::linux_env::process_ext::process::{PID2PC, Process, TID2TASK};

/// Return the `Arc<Process>` of the current process
#[allow(unused)]
pub fn current_process() -> Arc<Process> {
    let current_task = current();
    let current_process = Arc::clone(
        PID2PC
            .lock()
            .get(&current_task.task_ext().get_process_id())
            .unwrap(),
    );

    current_process
}

/// 以进程作为中转调用 task 的 yield
#[allow(unused)]
pub fn yield_now_task() {
    axtask::yield_now();
}

/// 以进程作为中转调用 task 的 sleep
#[allow(unused)]
pub fn sleep_now_task(dur: core::time::Duration) {
    axtask::sleep(dur);
}

/// Current running task
#[allow(unused)]
pub fn current_task() -> CurrentTask {
    axtask::current()
}

/// 设置当前任务的 clear_child_tid
#[allow(unused)]
pub fn set_child_tid(tid: usize) {
    todo!()
}

/// Get the task reference by tid
#[allow(unused)]
pub fn get_task_ref(tid: u64) -> Option<AxTaskRef> {
    TID2TASK.lock().get(&tid).cloned()
}
