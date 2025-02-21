//! 规定进程控制块内容
extern crate alloc;
use alloc::string::ToString;
use alloc::sync::Arc;
use alloc::{format, vec};
use alloc::vec::Vec;
use alloc::{collections::BTreeMap, string::String};
use axerrno::{AxError, AxResult};
use axlog::{debug, info, trace};
use axmm::{new_kernel_aspace, AddrSpace, Backend};
use axstd::println;
use axsync::Mutex;
use axtask::{current, spawn_task, yield_now, AxTaskRef, TaskId, TaskInner};
use memory_addr::{PhysAddr, VirtAddr, PAGE_SIZE_4K};
use crate::abi::UserContext;
use crate::config::{KERNEL_PROCESS_ID, TASK_STACK_SIZE};
use crate::fs::{FileIO, OpenFlags};
use crate::process::load_user_app;
use crate::process::stdio::{Stderr, Stdin, Stdout};
use crate::process::task_ext::TaskExt;
use crate::{FORK_WAIT, MAIN_WAIT_QUEUE, PARENT_WAIT_QUEUE};
use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};

use crate::process::fd_manager::{FdManager, FdTable};

/// Map from task id to arc pointer of task
pub static TID2TASK: Mutex<BTreeMap<u64, AxTaskRef>> = Mutex::new(BTreeMap::new());

/// Map from process id to arc pointer of process
pub static PID2PC: Mutex<BTreeMap<u64, Arc<Process>>> = Mutex::new(BTreeMap::new());

const FD_LIMIT_ORIGIN: usize = 1025;

#[allow(unused)]
/// The process control block
pub struct Process {
    /// 进程号
    pub pid: u64,
    /// 父进程号 
    pub parent: AtomicU64,
    /// 子进程
    pub children: Mutex<Vec<Arc<Process>>>,
    /// 所管理的线程
    pub tasks: Mutex<Vec<AxTaskRef>>,
    /// 文件描述符管理器
    pub fd_manager: FdManager,
    /// 进程状态
    pub is_zombie: AtomicBool,
    /// 退出状态码
    pub exit_code: AtomicI32,
    /// 地址空间
    pub memory_set: Mutex<Arc<Mutex<AddrSpace>>>,
    /// 可执行文件路径
    pub file_path: Mutex<String>,
    /// the page table token of the process which the task belongs to
    pub page_table_token: AtomicU64,
}

impl Process {
    /// 创建一个新的进程
    #[allow(unused)]
    pub fn new(
        pid: u64,
        parent: u64,
        memory_set: Mutex<Arc<Mutex<AddrSpace>>>,
        cwd: Arc<Mutex<String>>,
        mask: Arc<AtomicI32>,
        fd_table: FdTable,
    ) -> Self {
        let page_table_token = { 
            let ms = memory_set.lock();
            let token = ms.as_ref().lock().page_table_root().as_usize();
            AtomicU64::new(token as u64)
        };

        Self {
            pid,
            parent: AtomicU64::new(parent),
            children: Mutex::new(Vec::new()),
            tasks: Mutex::new(Vec::new()),
            is_zombie: AtomicBool::new(false),
            exit_code: AtomicI32::new(0),
            memory_set,
            fd_manager: FdManager::new(fd_table, cwd, mask, FD_LIMIT_ORIGIN),
            file_path:   Mutex::new(String::new()),
            page_table_token,
        }
    }

    /// 根据给定参数创建一个新的进程
    #[allow(unused)]
    pub fn init(mut path: String, elf_file: &'static [u8]) {
        let mut memory_set = new_kernel_aspace().unwrap();

        let page_table_token = memory_set.page_table_root();

        info!("page_table_token: 0x{:x}", page_table_token);
        
        let (entry, usp) = load_user_app(&mut memory_set, "fork", elf_file).unwrap();
    
        let new_fd_table: FdTable = Arc::new(Mutex::new(vec![
            Some(Arc::new(Stdin { flags: Mutex::new(OpenFlags::empty()) })),
            Some(Arc::new(Stdout { flags: Mutex::new(OpenFlags::empty()) })),
            Some(Arc::new(Stderr { flags: Mutex::new(OpenFlags::empty()) })),
        ]));
    
        let mut new_process = Arc::new(Self::new(
            TaskId::new().as_u64(),
            KERNEL_PROCESS_ID,
            Mutex::new(Arc::new(Mutex::new(memory_set))),
            Arc::new(Mutex::new(String::from("/").into())),
            Arc::new(AtomicI32::new(0o022)),
            new_fd_table,
        ));
    
        if !path.starts_with('/') {
            let cwd = new_process.get_cwd();
            assert!(cwd.ends_with('/'));
            path = format!("{}{}", cwd, path);
        }
    
        new_process.set_file_path(path.clone());
        
        let task_ext = TaskExt::init(new_process.pid(), true);

        let mut task_inner = TaskInner::new(
            move || {
                // 设置用户程序入口点
                unsafe { user_entry(entry.as_usize(), page_table_token, usp); }
            },
            path.to_string(),
            TASK_STACK_SIZE,
        );

        #[cfg(target_arch = "riscv64")]
        task_inner.ctx_mut().set_page_table_root(page_table_token);

        task_inner.init_task_ext(task_ext);

        let new_task = spawn_task(task_inner);

        TID2TASK.lock().insert(new_task.id().as_u64(), Arc::clone(&new_task));
        new_process.tasks.lock().push(Arc::clone(&new_task));
        PID2PC.lock().insert(new_process.pid(), Arc::clone(&new_process));

        MAIN_WAIT_QUEUE.wait();
    }

    pub fn fork(&self, stack_data: &'static [u8], user_ctx: UserContext) -> AxResult<u64> {
        // 1. 创建新的地址空间
        let parent_memory_set = self.memory_set.lock();
        let parent_ms = parent_memory_set.lock();

        let (start, end) = (parent_ms.base(), parent_ms.end());

        let mut child_memory_set = AddrSpace::new_empty(start, end.as_usize() - start.as_usize()).unwrap();

        // 遍历并复制所有内存区域
        for area in parent_ms.areas.iter() {
            let start = area.start();
            let size = area.size();
            let flags = area.flags();
            trace!("fork: start = {:#x}, size = {:#x}, flags = {:?}", start, size, flags);
            
            match area.backend() {
                Backend::Linear { pa_va_offset } => {
                    let paddr = PhysAddr::from(start.as_usize() - pa_va_offset);

                    child_memory_set.map_linear(
                        start,
                        paddr,
                        size,
                        flags
                    )?;
                },
                Backend::Alloc { populate: _ } => {
                    child_memory_set.map_alloc(
                        start,
                        size,
                        flags,
                        true
                    )?;
                    
                    let mut buffer = vec![0u8; PAGE_SIZE_4K];
                    for offset in (0..size).step_by(PAGE_SIZE_4K) {
                        let curr_size = PAGE_SIZE_4K.min(size - offset);
                        let curr_va = VirtAddr::from(start.as_usize() + offset);
                        parent_ms.read(curr_va, &mut buffer[..curr_size])?;
                        child_memory_set.write(curr_va, &buffer[..curr_size])?;
                    }
                }
            }
        }

        let page_table_root = child_memory_set.page_table_root();
        info!("page_table_root: 0x{:x}", page_table_root);

        // 创建新进程控制块
        let new_pid = TaskId::new().as_u64();
        let child_process = Arc::new(Process::new(
            new_pid,
            self.pid(),
            Mutex::new(Arc::new(Mutex::new(child_memory_set))),
            Arc::clone(&self.fd_manager.cwd),
            Arc::clone(&self.fd_manager.umask),
            Arc::clone(&self.fd_manager.fd_table),
        ));

        // 继承父进程属性
        child_process.set_file_path(self.get_file_path());

        // 建立父子关系
        self.children.lock().push(Arc::clone(&child_process));

        // 创建子进程的任务
        let mut child_inner = TaskInner::new(
            move || {
                // 通知父进程可以继续执行
                FORK_WAIT.notify_all(true);

                let current = current();
                let sub_kstack_top = current.kernel_stack_top().unwrap().as_usize();
                let sub_sp = sub_kstack_top - stack_data.len();

                info!("sub_sp : {:x?}, sub_kstack_top: {:x?}, stack_size : {:x?}", sub_sp, sub_kstack_top, sub_kstack_top - sub_sp);

                // let offset = sub_sp - user_ctx.sp;

                // info!("offset : {:x?}", offset);
            
                // // 复制栈内容
                // unsafe {
                //     core::ptr::copy_nonoverlapping(
                //         stack_data.as_ptr(),
                //         sub_sp as *mut u8,
                //         stack_data.len()
                //     );
                // }

                // unsafe {
                //     core::arch::asm!(
                //         "mv sp, {}",
                //         "mv s0, {}",
                //         // 设置子进程的返回值
                //         "li a0, 0",
                //         // 设置返回地址并跳转
                //         "mv t0, {}",          // 保存跳转地址到临时寄存器
                //         "jr t0",   // 跳转执行
                //         in(reg) sub_sp,
                //         in(reg) (user_ctx.s0 + offset + 48),
                //         in(reg) user_ctx.ra
                //     );
                // }
            },
            format!("task_{}", new_pid),
            TASK_STACK_SIZE,
        );

        #[cfg(target_arch = "riscv64")]
        {
            child_inner.ctx_mut().set_page_table_root(page_table_root);
            // 复制 user_ctx 中的字段到 child_inner 的上下文中
            // child_inner.ctx_mut().sp = user_ctx.sp;
            child_inner.ctx_mut().tp = user_ctx.tp;

            // 对于其他字段，也需要一一对应赋值，如下：
            child_inner.ctx_mut().s0 = user_ctx.s0;
            child_inner.ctx_mut().s1 = user_ctx.s1;
            child_inner.ctx_mut().s2 = user_ctx.s2;
            child_inner.ctx_mut().s3 = user_ctx.s3;
            child_inner.ctx_mut().s4 = user_ctx.s4;
            child_inner.ctx_mut().s5 = user_ctx.s5;
            child_inner.ctx_mut().s6 = user_ctx.s6;
            child_inner.ctx_mut().s7 = user_ctx.s7;
            child_inner.ctx_mut().s8 = user_ctx.s8;
            child_inner.ctx_mut().s9 = user_ctx.s9;
            child_inner.ctx_mut().s10 = user_ctx.s10;
            child_inner.ctx_mut().s11 = user_ctx.s11;
        }

        info!("child_inner.ctx() : {:x?}", child_inner.ctx());

        // 设置任务扩展信息
        child_inner.init_task_ext(TaskExt::init(child_process.pid(), true));
        
        // 创建子进程任务
        let child_task = spawn_task(child_inner);
    
        // 添加到全局表
        child_process.tasks.lock().push(Arc::clone(&child_task));
        TID2TASK.lock().insert(child_task.id().as_u64(), Arc::clone(&child_task));
        PID2PC.lock().insert(child_process.pid(), Arc::clone(&child_process));
        
        // 父进程等待子进程开始执行
        FORK_WAIT.wait();

        // 在最后添加这个 - 通知主进程父进程已完成
        PARENT_WAIT_QUEUE.notify_one(false);

        info!("Fork success! Parent={}, Child={}", self.pid(), new_pid);

        yield_now();

        Ok(new_pid)
    }
}

impl Process {
    /// get the process id
    #[allow(unused)]
    pub fn pid(&self) -> u64 {
        self.pid
    }
    
    /// get the page table token
    #[allow(unused)]
    pub fn page_table_token(&self) -> u64 {
        self.page_table_token.load(Ordering::Acquire) 
    }
    /// get the parent process id
    #[allow(unused)]
    pub fn get_parent(&self) -> u64 {
        self.parent.load(Ordering::Acquire)
    }

    /// set the parent process id
    #[allow(unused)]
    pub fn set_parent(&self, parent: u64) {
        self.parent.store(parent, Ordering::Release)
    }

    /// get the exit code of the process
    #[allow(unused)]
    pub fn get_exit_code(&self) -> i32 {
        self.exit_code.load(Ordering::Acquire)
    }
    
    /// set the exit code of the process
    #[allow(unused)]
    pub fn set_exit_code(&self, exit_code: i32) {
        self.exit_code.store(exit_code, Ordering::Release)
    }
    
    /// whether the process is a zombie process
    #[allow(unused)]
    pub fn get_zombie(&self) -> bool {
        self.is_zombie.load(Ordering::Acquire)
    }

    /// set the process as a zombie process
    #[allow(unused)]
    pub fn set_zombie(&self, status: bool) {
        self.is_zombie.store(status, Ordering::Release)
    }
    
    /// set the executable file path of the process
    #[allow(unused)]
    pub fn set_file_path(&self, path: String) {
        let mut file_path = self.file_path.lock();
        *file_path = path;
    }

    /// set the page table token of the process
    #[allow(unused)]
    pub fn set_page_table_token(&self, token: u64) {
        self.page_table_token.store(token, Ordering::Release);
    }
    
    /// get the executable file path of the process
    #[allow(unused)]
    pub fn get_file_path(&self) -> String {
        (*self.file_path.lock()).clone()
    }

    /// 若进程运行完成，则获取其返回码
    /// 若正在运行（可能上锁或没有上锁），则返回None
    #[allow(unused)]
    pub fn get_code_if_exit(&self) -> Option<i32> {
        if self.get_zombie() {
            return Some(self.get_exit_code());
        }
        None
    }
}

/// 与文件相关的进程方法
impl Process {
    /// 为进程分配一个文件描述符
    #[allow(unused)]
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
    #[allow(unused)]
    pub fn get_cwd(&self) -> String {
        self.fd_manager.cwd.lock().clone().to_string()
    }

    /// Set the current working directory of the process
    #[allow(unused)]
    pub fn set_cwd(&self, cwd: String) {
        *self.fd_manager.cwd.lock() = cwd.into();
    }
}

pub unsafe extern "C" fn user_entry(entry: usize, _page_table_token: PhysAddr, _usp: VirtAddr) -> () {
    info!("entry: 0x{:x}", entry);
    println!("Jump to task ...");

    unsafe {
        core::arch::asm!("
            mv      t2, {run_start}
            jalr    ra, t2, 0",
            run_start = in(reg) entry,
            clobber_abi("C"),
        )
    }
}
