use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use axtask::def_task_ext;

pub struct TaskExt {
    process_id: AtomicU64,
    /// 是否是所属进程下的主线程
    is_leader: AtomicBool,
}

impl TaskExt {
    /// Returns the expected size of the task extended structure.
    pub fn size() -> usize {
        // 使用 size_of 来获取 TaskExt 类型的大小
        size_of::<TaskExt>()
    }

    /// Returns the expected alignment of the task extended structure.
    pub fn align() -> usize {
        // 使用 align_of 来获取 TaskExt 类型的对齐要求
        align_of::<TaskExt>()
    }

    pub const fn init(process_id: u64, is_leader: bool) -> Self {
        Self {
            process_id: AtomicU64::new(process_id),
            is_leader: AtomicBool::new(is_leader),
        }
    }

    /// Get the process ID of the task
    pub fn get_process_id(&self) -> u64 {
        self.process_id.load(Ordering::Acquire)
    }

    /// Set the process ID of the task
    pub fn set_process_id(&self, process_id: u64) {
        self.process_id.store(process_id, Ordering::Release);
    }

    /// Set the flag whether the task is the main thread of the process
    pub fn set_leader(&self, is_lead: bool) {
        self.is_leader.store(is_lead, Ordering::Release);
    }

    /// Whether the task is the main thread of the process
    pub fn is_leader(&self) -> bool {
        self.is_leader.load(Ordering::Acquire)
    }
}

// 先注册扩展数据类型
def_task_ext!(TaskExt);
