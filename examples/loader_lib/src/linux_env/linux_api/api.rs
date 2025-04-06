use core::time::Duration;

use crate::abi::thread::abi_pthread_exit;
use alloc::{string::String, sync::Arc};
use axhal::time::{TimeValue, monotonic_time_nanos};
use axstd::process::exit;
use axtask::current;
use lazyinit::LazyInit;

use crate::elf_load::batch::reentry_label;
#[cfg(feature = "pseudo_multi_process")]
use crate::linux_env::process_ext::api::current_process;
use crate::linux_env::process_ext::process::Process;

#[cfg(any(feature = "unikernel", feature = "batch"))]
pub static UNI_API: LazyInit<Arc<Process>> = LazyInit::new();

#[cfg(any(feature = "unikernel", feature = "batch"))]
pub fn process_api() -> Arc<Process> {
    UNI_API.get().unwrap().clone()
}

#[cfg(feature = "unikernel")]
pub fn exit_current_task(exit_code: i32) -> ! {
    exit(exit_code);
}

#[cfg(feature = "batch")]
pub fn exit_current_task(exit_code: i32) -> ! {
    reentry_label(); // 这里回到OS
}

#[cfg(feature = "pseudo_multi_process")]
pub fn process_api() -> Arc<Process> {
    current_process()
}

#[cfg(feature = "pseudo_multi_process")]
pub fn exit_current_task(exit_code: i32) -> ! {
    unimplemented!();
}

// /// `main`线程等待所有进程结束
// pub static MAIN_WAIT_QUEUE: WaitQueue = WaitQueue::new();
// /// Map from task id to arc pointer of task
// pub static TID2TASK: Mutex<BTreeMap<u64, AxTaskRef>> = Mutex::new(BTreeMap::new());
//
// pub static UNI_API: LazyInit<UniAPI> = LazyInit::new();
//
// /// The Uni_Inter
// #[cfg(not(feature = "multiprocess"))]
// pub struct UniAPI {
//     /// 所管理的线程
//     pub tasks: Mutex<Vec<AxTaskRef>>,
//
//     /// 文件描述符管理器
//     pub fd_manager: FdManager,
//
//     /// 地址空间
//     //    pub memory_set: Mutex<Arc<Mutex<MemorySet>>>,
//
//     /// 用户堆基址，任何时候堆顶都不能比这个值小，理论上讲是一个常量
//     pub heap_bottom: AtomicU64,
//
//     /// 当前用户堆的堆顶，不能小于基址，不能大于基址加堆的最大大小
//     pub heap_top: AtomicU64,
//
//     /// 是否被`vfork`阻塞
//     pub blocked_by_vfork: Mutex<bool>,
//
//     /// 地址空间
//     pub memory_set: Mutex<Arc<Mutex<AddrSpace>>>,
//
//     /// 该进程可执行文件所在的路径
//     pub file_path: Mutex<String>,
// }
//
// impl UniAPI {
//     /// Get the heap top of the process
//     pub fn get_heap_top(&self) -> u64 {
//         self.heap_top.load(Ordering::Acquire)
//     }
//
//     /// Set the heap top of the process
//     pub fn set_heap_top(&self, top: u64) {
//         self.heap_top.store(top, Ordering::Release)
//     }
//
//     /// Get the heap bottom of the process
//     pub fn get_heap_bottom(&self) -> u64 {
//         self.heap_bottom.load(Ordering::Acquire)
//     }
//
//     /// Set the heap bottom of the process
//     pub fn set_heap_bottom(&self, bottom: u64) {
//         self.heap_bottom.store(bottom, Ordering::Release)
//     }
//
//     /// Set the process as blocked by `vfork`
//     pub fn set_vfork_block(&self, value: bool) {
//         *self.blocked_by_vfork.lock() = value;
//     }
//
//     /// Set the executable file path of the process
//     pub fn set_file_path(&self, path: String) {
//         let mut file_path = self.file_path.lock();
//         *file_path = path;
//     }
//
//     /// Get the executable file path of the process
//     pub fn get_file_path(&self) -> String {
//         (*self.file_path.lock()).clone()
//     }
// }
//
// impl UniAPI {
//     pub fn new(
//         fd_table: Vec<Option<Arc<dyn FileIO>>>,
//         heap_bottom: u64,
//         memory_set: Mutex<Arc<Mutex<AddrSpace>>>,
//     ) -> Self {
//         Self {
//             heap_bottom: AtomicU64::new(heap_bottom),
//             heap_top: AtomicU64::new(heap_bottom),
//             tasks: Mutex::new(Vec::new()),
//             fd_manager: FdManager::new(fd_table, FD_LIMIT_ORIGIN),
//             blocked_by_vfork: Mutex::new(false),
//             memory_set,
//             file_path: Mutex::new(String::new()),
//         }
//     }
//     pub fn init(
//         mut path: String,
//         app_elf_file: &'static [u8],
//         lib_elf_file: Option<&'static [u8]>,
//     ) {
//         let mut memory_set = new_kernel_aspace().unwrap();
//         info!("new kernel aspace: memory_set stack {:?}", memory_set);
//         let page_table_token = memory_set.page_table_root();
//
//         info!("page_table_token: 0x{:x}", page_table_token);
//
//         let (entry, usp) =
//             match load_user_app(&mut memory_set, "sqlite", app_elf_file, lib_elf_file) {
//                 Ok(t) => t,
//                 Err(e) => {
//                     panic!("{:?}", e);
//                 }
//             };
//
//         UNI_API.init_once(UniAPI::new(
//             vec![
//                 // 标准输入
//                 Some(Arc::new(Stdin {
//                     flags: Mutex::new(OpenFlags::empty()),
//                 })),
//                 // 标准输出
//                 Some(Arc::new(Stdout {
//                     flags: Mutex::new(OpenFlags::empty()),
//                 })),
//                 // 标准错误
//                 Some(Arc::new(Stderr {
//                     flags: Mutex::new(OpenFlags::empty()),
//                 })),
//             ],
//             // FIXME:
//             0,
//             Mutex::new(Arc::new(Mutex::new(memory_set))),
//         ));
//
//         info!("ENTRY: {:x}", entry.as_usize());
//         let mut task_inner = TaskInner::new(
//             move || {
//                 // 设置用户程序入口点
//                 unsafe {
//                     user_entry(entry.as_usize(), usp);
//                 }
//             },
//             path.to_string(),
//             TASK_STACK_SIZE,
//         );
//
//         #[cfg(target_arch = "riscv64")]
//         task_inner.ctx_mut().set_page_table_root(page_table_token);
//         let proc_id = task_inner.id().as_u64();
//
//         // ```
//         // new_process.pid.store(proc_id, Ordering::Release);
//
//         let task_ext = TaskExt::init(proc_id, true);
//
//         task_inner.init_task_ext(task_ext);
//
//         let new_task = spawn_task(task_inner);
//
//         TID2TASK
//             .lock()
//             .insert(new_task.id().as_u64(), Arc::clone(&new_task));
//         UNI_API.tasks.lock().push(Arc::clone(&new_task));
//         // PID2PC.lock().insert(proc_id, Arc::clone(&new_process));
//         //
//         // 添加到进程计数
//         // PROCESS_COUNT.fetch_add(1, Ordering::SeqCst);
//
//         MAIN_WAIT_QUEUE.wait();
//     }
// }
//
// // /// 与地址空间相关的进程方法
// // impl UniAPI {
// //     /// Alloc physical memory for lazy allocation manually
// //     pub fn manual_alloc_for_lazy(&self, addr: VirtAddr) -> AxResult<()> {
// //         self.memory_set.lock().lock().manual_alloc_for_lazy(addr)
// //     }
// //
// //     /// Alloc range physical memory for lazy allocation manually
// //     pub fn manual_alloc_range_for_lazy(&self, start: VirtAddr, end: VirtAddr) -> AxResult<()> {
// //         self.memory_set
// //             .lock()
// //             .lock()
// //             .manual_alloc_range_for_lazy(start, end)
// //     }
// //
// //     /// Alloc physical memory with the given type size for lazy allocation manually
// //     pub fn manual_alloc_type_for_lazy<T: Sized>(&self, obj: *const T) -> AxResult<()> {
// //         self.memory_set
// //             .lock()
// //             .lock()
// //             .manual_alloc_type_for_lazy(obj)
// //     }
// // }
//
// /// 与文件相关的进程方法
// impl UniAPI {
//     /// 为进程分配一个文件描述符
//     pub fn alloc_fd(&self, fd_table: &mut Vec<Option<Arc<dyn FileIO>>>) -> AxResult<usize> {
//         for (i, fd) in fd_table.iter().enumerate() {
//             if fd.is_none() {
//                 return Ok(i);
//             }
//         }
//         if fd_table.len() >= self.fd_manager.get_limit() as usize {
//             debug!("fd table is full");
//             return Err(AxError::StorageFull);
//         }
//         fd_table.push(None);
//         Ok(fd_table.len() - 1)
//     }
//
//     /// 获取当前进程的工作目录
//     pub fn get_cwd(&self) -> String {
//         self.fd_manager.cwd.lock().clone()
//     }
// }
//
// pub unsafe extern "C" fn user_entry(entry: usize, usp: VirtAddr) -> () {
//     info!("entry: 0x{:x}", entry);
//     info!("Jump to task ...");
//
//     unsafe {
//         core::arch::asm!("
//             la      a7, {abi_table}
//             mv      t2, {run_start}
//             jalr    ra, t2, 0",
//             run_start = in(reg) entry,
//             abi_table = sym ABI_TABLE,
//             clobber_abi("C"),
//         )
//     }
// }

/// 统计时间输出
/// (用户态秒，用户态微妙，内核态秒，内核态微妙)
pub fn time_stat_output() -> (usize, usize, usize, usize) {
    let curr_task = current();
    curr_task.time_stat_output()
}

/// Returns the current clock time in nanoseconds.
pub fn current_time_nanos() -> u64 {
    monotonic_time_nanos()
}

/// Returns the current clock time in [`TimeValue`].
pub fn current_time() -> TimeValue {
    TimeValue::from_nanos(current_time_nanos())
}

/// Busy waiting for the given duration.
pub fn busy_wait(dur: Duration) {
    busy_wait_until(current_time() + dur);
}

/// Busy waiting until reaching the given deadline.
pub fn busy_wait_until(deadline: TimeValue) {
    while current_time() < deadline {
        core::hint::spin_loop();
    }
}
