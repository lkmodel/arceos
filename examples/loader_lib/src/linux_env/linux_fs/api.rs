extern crate alloc;
use alloc::{collections::BTreeMap, string::String, sync::Arc, vec::Vec};
use axerrno::{AxError, AxResult};
use axtask::AxTaskRef;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::linux_env::{
    axfs_ext::api::FileIO,
    linux_fs::fd_manager::{FD_LIMIT_ORIGIN, FdManager},
};

use axlog::debug;
use axsync::Mutex;
use lazyinit::LazyInit;

/// Map from task id to arc pointer of task
pub static TID2TASK: Mutex<BTreeMap<u64, AxTaskRef>> = Mutex::new(BTreeMap::new());

pub static UNI_API: LazyInit<UniAPI> = LazyInit::new();

/// The Uni_Inter
#[cfg(not(feature = "multiprocess"))]
pub struct UniAPI {
    /// 所管理的线程
    pub tasks: Mutex<Vec<AxTaskRef>>,

    /// 文件描述符管理器
    pub fd_manager: FdManager,

    /// 地址空间
    //    pub memory_set: Mutex<Arc<Mutex<MemorySet>>>,

    /// 用户堆基址，任何时候堆顶都不能比这个值小，理论上讲是一个常量
    pub heap_bottom: AtomicU64,

    /// 当前用户堆的堆顶，不能小于基址，不能大于基址加堆的最大大小
    pub heap_top: AtomicU64,

    /// 是否被`vfork`阻塞
    pub blocked_by_vfork: Mutex<bool>,

    /// 该进程可执行文件所在的路径
    pub file_path: Mutex<String>,
}

impl UniAPI {
    /// Get the heap top of the process
    pub fn get_heap_top(&self) -> u64 {
        self.heap_top.load(Ordering::Acquire)
    }

    /// Set the heap top of the process
    pub fn set_heap_top(&self, top: u64) {
        self.heap_top.store(top, Ordering::Release)
    }

    /// Get the heap bottom of the process
    pub fn get_heap_bottom(&self) -> u64 {
        self.heap_bottom.load(Ordering::Acquire)
    }

    /// Set the heap bottom of the process
    pub fn set_heap_bottom(&self, bottom: u64) {
        self.heap_bottom.store(bottom, Ordering::Release)
    }

    /// Set the process as blocked by `vfork`
    pub fn set_vfork_block(&self, value: bool) {
        *self.blocked_by_vfork.lock() = value;
    }

    /// Set the executable file path of the process
    pub fn set_file_path(&self, path: String) {
        let mut file_path = self.file_path.lock();
        *file_path = path;
    }

    /// Get the executable file path of the process
    pub fn get_file_path(&self) -> String {
        (*self.file_path.lock()).clone()
    }
}

impl UniAPI {
    pub fn new(fd_table: Vec<Option<Arc<dyn FileIO>>>, heap_bottom: u64) -> Self {
        Self {
            heap_bottom: AtomicU64::new(heap_bottom),
            heap_top: AtomicU64::new(heap_bottom),
            tasks: Mutex::new(Vec::new()),
            fd_manager: FdManager::new(fd_table, FD_LIMIT_ORIGIN),
            blocked_by_vfork: Mutex::new(false),
            file_path: Mutex::new(String::new()),
        }
    }
    pub fn init(args: Vec<String>, envs: &Vec<String>) -> AxResult<AxTaskRef> {
        unimplemented!();
        // let memory_set = MemorySet::new_memory_set();
        // let page_table_token = memory_set.page_table_token();

        //        if page_table_token != 0 {
        //            unsafe {
        //                write_page_table_root0(page_table_token.into());
        //                #[cfg(target_arch = "riscv64")]
        //                riscv::register::sstatus::set_sum();
        //            };
        //        }

        //        let (entry, user_stack_bottom, heap_bottom) =
        //            if let Ok(ans) = load_app(path.clone(), args, envs, &mut memory_set) {
        //                ans
        //            } else {
        //                error!("Failed to load app {}", path);
        //                return Err(AxError::NotFound);
        //            };
        //        UNI_API.init_once(UniAPI::new(
        //            heap_bottom.as_usize() as u64,
        //            vec![
        //                // 标准输入
        //                Some(Arc::new(Stdin {
        //                    flags: Mutex::new(OpenFlags::empty()),
        //                })),
        //                // 标准输出
        //                Some(Arc::new(Stdout {
        //                    flags: Mutex::new(OpenFlags::empty()),
        //                })),
        //                // 标准错误
        //                Some(Arc::new(Stderr {
        //                    flags: Mutex::new(OpenFlags::empty()),
        //                })),
        //            ],
        //            Mutex::new(Arc::new(Mutex::new(memory_set))),
        //        ));
        //
        //        let new_task = TaskInner::new(
        //            || {},
        //            path,
        //            axconfig::TASK_STACK_SIZE,
        //            new_process.pid(),
        //            page_table_token,
        //            #[cfg(feature = "signal")]
        //            false,
        //        );
        //        TID2TASK
        //            .lock()
        //            .insert(new_task.id().as_u64(), Arc::clone(&new_task));
        //        new_task.set_leader(true);
        //        let new_trap_frame =
        //            TrapFrame::app_init_context(entry.as_usize(), user_stack_bottom.as_usize());
        //        new_task.set_trap_context(new_trap_frame);
        //        // 需要将完整内容写入到内核栈上，first_into_user并不会复制到内核栈上
        //        new_task.set_trap_in_kernel_stack();
        //        //        new_process.tasks.lock().push(Arc::clone(&new_task));
        //        //        new_process
        //        //            .robust_list
        //        //            .lock()
        //        //            .insert(new_task.id().as_u64(), FutexRobustList::default());
        //        //        PID2PC
        //        //            .lock()
        //        //            .insert(new_process.pid(), Arc::clone(&new_process));
        //        //        // 将其作为内核进程的子进程
        //        //        match PID2PC.lock().get(&KERNEL_PROCESS_ID) {
        //        //            Some(kernel_process) => {
        //        //                kernel_process.children.lock().push(new_process);
        //        //            }
        //        //            None => {
        //        //                return Err(AxError::NotFound);
        //        //            }
        //        //        }
        //        RUN_QUEUE.lock().add_task(Arc::clone(&new_task));
        //        Ok(new_task)
    }
}

// /// 与地址空间相关的进程方法
// impl UniAPI {
//     /// Alloc physical memory for lazy allocation manually
//     pub fn manual_alloc_for_lazy(&self, addr: VirtAddr) -> AxResult<()> {
//         self.memory_set.lock().lock().manual_alloc_for_lazy(addr)
//     }
//
//     /// Alloc range physical memory for lazy allocation manually
//     pub fn manual_alloc_range_for_lazy(&self, start: VirtAddr, end: VirtAddr) -> AxResult<()> {
//         self.memory_set
//             .lock()
//             .lock()
//             .manual_alloc_range_for_lazy(start, end)
//     }
//
//     /// Alloc physical memory with the given type size for lazy allocation manually
//     pub fn manual_alloc_type_for_lazy<T: Sized>(&self, obj: *const T) -> AxResult<()> {
//         self.memory_set
//             .lock()
//             .lock()
//             .manual_alloc_type_for_lazy(obj)
//     }
// }

/// 与文件相关的进程方法
impl UniAPI {
    /// 为进程分配一个文件描述符
    pub fn alloc_fd(&self, fd_table: &mut Vec<Option<Arc<dyn FileIO>>>) -> AxResult<usize> {
        for (i, fd) in fd_table.iter().enumerate() {
            if fd.is_none() {
                return Ok(i);
            }
        }
        if fd_table.len() >= self.fd_manager.get_limit() as usize {
            debug!("fd table is full");
            return Err(AxError::StorageFull);
        }
        fd_table.push(None);
        Ok(fd_table.len() - 1)
    }

    /// 获取当前进程的工作目录
    pub fn get_cwd(&self) -> String {
        self.fd_manager.cwd.lock().clone()
    }
}

// pub fn time_stat_output() -> (usize, usize, usize, usize) {
//     let curr_task = current();
//     curr_task.time_stat_output()
// }
