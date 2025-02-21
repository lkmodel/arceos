use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use axtask::def_task_ext;

pub struct TaskExt {
    process_id: AtomicU64,
    /// 是否是所属进程下的主线程
    is_leader: AtomicBool,
    // /// the page table token of the process which the task belongs to
    // pub page_table_token: UnsafeCell<usize>,
    // set_child_tid: AtomicU64,
    // clear_child_tid: AtomicU64,
    
}

impl TaskExt {
    pub const fn init(process_id: u64, is_leader: bool) -> Self {
        Self {
            process_id: AtomicU64::new(process_id),
            is_leader: AtomicBool::new(is_leader),
            // page_table_token: UnsafeCell::new(0),
            // set_child_tid: AtomicU64::new(0),
            // clear_child_tid: AtomicU64::new(0),
        }
    }

    /// get the process ID of the task
    pub fn get_process_id(&self) -> u64 {
        self.process_id.load(Ordering::Acquire)
    }

    /// set the process ID of the task
    pub fn set_process_id(&self, process_id: u64) {
        self.process_id.store(process_id, Ordering::Release);
    }

    /// set the flag whether the task is the main thread of the process
    pub fn set_leader(&self, is_lead: bool) {
        self.is_leader.store(is_lead, Ordering::Release);
    }

    /// whether the task is the main thread of the process
    pub fn is_leader(&self) -> bool {
        self.is_leader.load(Ordering::Acquire)
    }

    // /// store the child thread ID at the location pointed to by child_tid in clone args
    // pub fn set_child_tid(&self, tid: usize) {
    //     self.set_child_tid.store(tid as u64, Ordering::Release)
    // }

    // /// clear (zero) the child thread ID at the location pointed to by child_tid in clone args
    // pub fn set_clear_child_tid(&self, tid: usize) {
    //     self.clear_child_tid.store(tid as u64, Ordering::Release)
    // }

    // /// get the pointer to the child thread ID
    // pub fn get_clear_child_tid(&self) -> usize {
    //     self.clear_child_tid.load(Ordering::Acquire) as usize
    // }
}

// 先注册扩展数据类型
def_task_ext!(TaskExt);