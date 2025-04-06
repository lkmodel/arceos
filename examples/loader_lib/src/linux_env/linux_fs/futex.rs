//! 实现与`futex`相关的系统调用
use alloc::collections::{BTreeMap, VecDeque};
use axhal::mem::{MemoryAddr, VirtAddr};
use axsync::Mutex;
use axtask::{AxTaskRef, WaitQueue};

extern crate alloc;

/// `vec`中的元素分别是任务指针,对应存储时的`futex`变量的值
pub static FUTEX_WAIT_TASK: Mutex<BTreeMap<FutexKey, VecDeque<(AxTaskRef, u32)>>> =
    Mutex::new(BTreeMap::new());

/// Waiting queue which stores tasks waiting for `futex` variable
pub static WAIT_FOR_FUTEX: Mutex<WaitQueue> = Mutex::new(WaitQueue::new());

/// `Futexes` are matched on equal values of this key.
///
/// The key type depends on whether it's a shared or private mapping.
/// Use `pid` to replace the mm_struct pointer
#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub struct FutexKey {
    ptr: u64,
    word: usize,
    offset: u32,
}

impl FutexKey {
    fn new(ptr: u64, word: usize, offset: u32) -> Self {
        Self { ptr, word, offset }
    }
}

/// 获取`futex`变量的key
/// TODO: shared `futex` and error handling
pub fn get_futex_key(uaddr: VirtAddr, _flags: i32) -> FutexKey {
    let ptr = 0;
    let offset = uaddr.align_offset_4k() as u32;
    let word = uaddr.align_down_4k().as_usize();
    FutexKey::new(ptr, word, offset)
}

#[derive(Default)]
/// 用于存储 robust list 的结构
pub struct FutexRobustList {
    /// The location of the head of the robust list in user space
    pub head: usize,
    /// The length of the robust list
    pub len: usize,
}

impl FutexRobustList {
    /// Create a new robust list
    pub fn new(head: usize, len: usize) -> Self {
        Self { head, len }
    }
}

//```
// /// 退出的时候清空指针
// ///
// /// 若当前线程是主线程，代表进程退出，此时传入的id是进程id，要清除所有进程下的线程
// ///
// /// 否则传入的id是线程id
// pub fn clear_wait(id: u64, leader: bool) {
//     let mut futex_wait_task = FUTEX_WAIT_TASK.lock();
//
//     if leader {
//         // 清空所有所属进程为指定进程的线程
//         futex_wait_task.iter_mut().for_each(|(_, tasks)| {
//             // tasks.drain_filter(|task| task.get_process_id() == id);
//             tasks.retain(|(task, _)| task.get_process_id() != id);
//         });
//     } else {
//         futex_wait_task.iter_mut().for_each(|(_, tasks)| {
//             // tasks.drain_filter(|task| task.id().as_u64() == id);
//             tasks.retain(|(task, _)| task.id().as_u64() != id)
//         });
//     }
//
//     // 如果一个共享变量不会被线程所使用了，那么直接把他移除
//     // info!("clean pre keys: {:?}", futex_wait_task.keys());
//     futex_wait_task.retain(|_, tasks| !tasks.is_empty());
// }
