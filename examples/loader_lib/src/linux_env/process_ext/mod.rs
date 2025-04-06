pub mod api;
pub mod context;
pub mod process;

use axhal::arch::TrapFrame;
use axtask::WaitQueue;
use core::sync::atomic::AtomicUsize;

// 全局等待队列
pub static MAIN_WAIT_QUEUE: WaitQueue = WaitQueue::new(); // Main线程等待所有进程结束
pub static FORK_WAIT: WaitQueue = WaitQueue::new(); // 父进程等待子进程开始执行 

// 进程计数
pub static PROCESS_COUNT: AtomicUsize = AtomicUsize::new(0);

// 保存原始内核的 GP 和应用的 GP
pub static APP_GP: AtomicUsize = AtomicUsize::new(0);
pub static KERNEL_GP: AtomicUsize = AtomicUsize::new(0);

#[cfg(target_arch = "riscv64")]
/// set the return code
pub fn set_ret_code(tp: &mut TrapFrame, ret_value: usize) {
    use axlog::warn;

    warn!("test");
    tp.regs.a0 = ret_value;
}

#[cfg(target_arch = "x86_64")]
/// set the return code
pub fn set_ret_code(tp: &mut TrapFrame, ret_value: usize) {
    tp.rax = ret_value as _;
}

#[cfg(target_arch = "aarch64")]
pub fn set_ret_code(tp: &mut TrapFrame, ret: usize) {
    tp.r[0] = ret;
}
