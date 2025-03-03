// //! 规定进程控制块内容
// extern crate alloc;
// use alloc::vec;
// use alloc::{
//     collections::BTreeMap,
//     format,
//     string::{String, ToString},
//     sync::Arc,
//     vec::Vec,
// };
// use axerrno::AxResult;
// use axlog::{info, trace};
// use axmm::{AddrSpace, Backend, new_kernel_aspace};
// use axstd::println;
// use axsync::Mutex;
// use axtask::{AxTaskRef, TaskInner, current, spawn_task};
// use core::sync::atomic::{AtomicU64, Ordering};
// use memory_addr::{PAGE_SIZE_4K, PhysAddr, VirtAddr};
//
// use crate::config::TASK_STACK_SIZE;
// use crate::process::load_user_app;
// use crate::process::task_ext::TaskExt;
// use crate::{FORK_WAIT, MAIN_WAIT_QUEUE, PROCESS_COUNT, UserContext};
//
// /// Map from task id to arc pointer of task
// pub static TID2TASK: Mutex<BTreeMap<u64, AxTaskRef>> = Mutex::new(BTreeMap::new());
//
// /// Map from process id to arc pointer of process
// pub static PID2PC: Mutex<BTreeMap<u64, Arc<Process>>> = Mutex::new(BTreeMap::new());
//
// #[allow(unused)]
// /// The process control block
// pub struct Process {
//     /// 进程号
//     pub pid: AtomicU64,
//     /// 父进程号
//     pub parent: AtomicU64,
//     /// 子进程
//     pub children: Mutex<Vec<Arc<Process>>>,
//     /// 所管理的线程
//     pub tasks: Mutex<Vec<AxTaskRef>>,
//     /// 地址空间
//     pub memory_set: Mutex<Arc<Mutex<AddrSpace>>>,
//     /// the page table token of the process which the task belongs to
//     pub page_table_token: AtomicU64,
// }
//
// impl Process {
//     /// 创建一个新的进程
//     #[allow(unused)]
//     pub fn new(parent: u64, memory_set: Mutex<Arc<Mutex<AddrSpace>>>) -> Self {
//         let page_table_token = {
//             let ms = memory_set.lock();
//             let token = ms.as_ref().lock().page_table_root().as_usize();
//             AtomicU64::new(token as u64)
//         };
//
//         Self {
//             pid: AtomicU64::new(0),
//             parent: AtomicU64::new(parent),
//             children: Mutex::new(Vec::new()),
//             tasks: Mutex::new(Vec::new()),
//             memory_set,
//             page_table_token,
//         }
//     }
//
//     /// 根据给定参数创建一个新的进程
//     #[allow(unused)]
//     pub fn init(mut path: String, elf_file: &'static [u8]) {
//         let mut memory_set = new_kernel_aspace().unwrap();
//
//         let page_table_token = memory_set.page_table_root();
//
//         info!("page_table_token: 0x{:x}", page_table_token);
//
//         let (entry, usp) = load_user_app(&mut memory_set, "fork", elf_file).unwrap();
//
//         let mut new_process = Arc::new(Self::new(
//             current().id().as_u64(),
//             Mutex::new(Arc::new(Mutex::new(memory_set))),
//         ));
//
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
//
//         let proc_id = task_inner.id().as_u64();
//
//         new_process.pid.store(proc_id, Ordering::Release);
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
//         new_process.tasks.lock().push(Arc::clone(&new_task));
//         PID2PC.lock().insert(proc_id, Arc::clone(&new_process));
//
//         // 添加到进程计数
//         PROCESS_COUNT.fetch_add(1, Ordering::SeqCst);
//
//         MAIN_WAIT_QUEUE.wait();
//     }
//
//     pub fn fork(
//         &self,
//         stack_data: &'static [u8],
//         #[allow(unused)] user_ctx: UserContext,
//     ) -> AxResult<u64> {
//         // 创建新的地址空间
//         let parent_memory_set = self.memory_set.lock();
//         let parent_ms = parent_memory_set.lock();
//
//         let (start, end) = (parent_ms.base(), parent_ms.end());
//
//         let mut child_memory_set =
//             AddrSpace::new_empty(start, end.as_usize() - start.as_usize()).unwrap();
//
//         // 遍历并复制所有内存区域
//         for area in parent_ms.areas.iter() {
//             let start = area.start();
//             let size = area.size();
//             let flags = area.flags();
//             trace!(
//                 "fork: start = {:#x}, size = {:#x}, flags = {:?}",
//                 start, size, flags
//             );
//
//             match area.backend() {
//                 Backend::Linear { pa_va_offset } => {
//                     let paddr = PhysAddr::from(start.as_usize() - pa_va_offset);
//
//                     child_memory_set.map_linear(start, paddr, size, flags)?;
//                 }
//                 Backend::Alloc { populate: _ } => {
//                     child_memory_set.map_alloc(start, size, flags, true)?;
//
//                     let mut buffer = vec![0u8; PAGE_SIZE_4K];
//                     for offset in (0..size).step_by(PAGE_SIZE_4K) {
//                         let curr_size = PAGE_SIZE_4K.min(size - offset);
//                         let curr_va = VirtAddr::from(start.as_usize() + offset);
//                         parent_ms.read(curr_va, &mut buffer[..curr_size])?;
//                         child_memory_set.write(curr_va, &buffer[..curr_size])?;
//                     }
//                 }
//             }
//         }
//
//         let page_table_root = child_memory_set.page_table_root();
//         info!("page_table_root: 0x{:x}", page_table_root);
//
//         // 创建新进程控制块
//         let child_process = Arc::new(Process::new(
//             self.pid(),
//             Mutex::new(Arc::new(Mutex::new(child_memory_set))),
//         ));
//
//         // 建立父子关系
//         self.children.lock().push(Arc::clone(&child_process));
//
//         // 创建子进程的任务
//         let mut child_inner = TaskInner::new(
//             move || {
//                 info!("Jump to sub_task ...");
//
//                 // 在子进程中
//                 FORK_WAIT.notify_one(true);
//
//                 #[cfg(target_arch = "riscv64")]
//                 {
//                     let current = current();
//                     let sub_sp = current.as_task_ref().inner().ctx().sp;
//                     let sbu_s0 = current.as_task_ref().inner().ctx().s0;
//
//                     unsafe {
//                         let mut pc: usize;
//                         core::arch::asm!(
//                             // 获取当前PC
//                             "auipc {}, 0",  // 将当前PC值加载到寄存器中
//                             out(reg) pc,
//                         );
//
//                         info!("Current PC: 0x{:x}", pc);
//
//                         info!("{:x?}", current.ctx());
//                         fork_entry(sub_sp, sbu_s0, user_ctx.ra);
//                     }
//                 }
//             },
//             format!("child"),
//             TASK_STACK_SIZE,
//         );
//
//         let sub_kstack_top = child_inner.kernel_stack_top().unwrap().as_usize();
//         let sub_sp = sub_kstack_top - stack_data.len();
//         info!(
//             "sub_sp : {:x?}, sub_kstack_top: {:x?}, stack_size : {:x?}",
//             sub_sp,
//             sub_kstack_top,
//             sub_kstack_top - sub_sp
//         );
//
//         // 复制栈内容
//         unsafe {
//             core::ptr::copy_nonoverlapping(
//                 stack_data.as_ptr(),
//                 sub_sp as *mut u8,
//                 stack_data.len(),
//             );
//         }
//
//         // let mut pc: usize;
//
//         // unsafe {
//         //     core::arch::asm!(
//         //         // 获取当前PC
//         //         "auipc {}, 0",  // 将当前PC值加载到寄存器中
//         //         out(reg) pc,
//         //     );
//         // }
//
//         // info!("Current PC: 0x{:x}", pc);
//
//         #[cfg(target_arch = "riscv64")]
//         {
//             child_inner.ctx_mut().set_page_table_root(page_table_root);
//             child_inner.ctx_mut().sp = sub_sp;
//             child_inner.ctx_mut().s0 = sub_sp + 48;
//             child_inner.ctx_mut().s1 = user_ctx.s1;
//             child_inner.ctx_mut().s2 = user_ctx.s2;
//             child_inner.ctx_mut().s3 = user_ctx.s3;
//             child_inner.ctx_mut().s4 = user_ctx.s4;
//             child_inner.ctx_mut().s5 = user_ctx.s5;
//             child_inner.ctx_mut().s6 = user_ctx.s6;
//             child_inner.ctx_mut().s7 = user_ctx.s7;
//             child_inner.ctx_mut().s8 = user_ctx.s8;
//             child_inner.ctx_mut().s9 = user_ctx.s9;
//             child_inner.ctx_mut().s10 = user_ctx.s10;
//             child_inner.ctx_mut().s11 = user_ctx.s11;
//             child_inner.ctx_mut().tp = user_ctx.tp;
//         }
//
//         info!("{:x?}", child_inner.ctx());
//
//         let child_pid = child_inner.id().as_u64();
//
//         // 设置任务扩展信息
//         child_inner.init_task_ext(TaskExt::init(child_pid, true));
//
//         // 创建子进程任务
//         let child_task = spawn_task(child_inner);
//
//         // 添加到全局表
//         child_process.tasks.lock().push(Arc::clone(&child_task));
//         TID2TASK
//             .lock()
//             .insert(child_task.id().as_u64(), Arc::clone(&child_task));
//         PID2PC
//             .lock()
//             .insert(child_process.pid(), Arc::clone(&child_process));
//
//         // 添加到进程计数
//         PROCESS_COUNT.fetch_add(1, Ordering::SeqCst);
//
//         info!("Fork success! Parent={}, Child={}", self.pid(), child_pid);
//
//         // 父进程等待子进程开始执行
//         FORK_WAIT.wait();
//
//         Ok(child_pid)
//     }
// }
//
// impl Process {
//     /// get the process ID
//     pub fn pid(&self) -> u64 {
//         self.pid.load(Ordering::Acquire)
//     }
//
//     /// get the page table token
//     #[allow(unused)]
//     pub fn page_table_token(&self) -> u64 {
//         self.page_table_token.load(Ordering::Acquire)
//     }
//
//     /// set the page table token of the process
//     #[allow(unused)]
//     pub fn set_page_table_token(&self, token: u64) {
//         self.page_table_token.store(token, Ordering::Release);
//     }
// }
//
// pub unsafe extern "C" fn user_entry(entry: usize, _usp: VirtAddr) -> () {
//     info!("entry: 0x{:x}", entry);
//     println!("Jump to task ...");
//
//     unsafe {
//         core::arch::asm!("
//             mv      t2, {run_start}
//             jalr    ra, t2, 0",
//             run_start = in(reg) entry,
//             clobber_abi("C"),
//         )
//     }
// }
//
// #[naked]
// #[allow(unused)]
// pub unsafe extern "C" fn fork_entry(sp: usize, s0: usize, ra: usize) {
//     unsafe {
//         core::arch::naked_asm!(
//             "mv sp, a0", // 第一个参数作为 sp
//             "mv s0, a1", // 第二个参数作为 s0
//             "li a0, 0",  // 子进程返回值为 0
//             "jr a2",     // 跳转到第三个参数（ra）
//         );
//     }
// }
