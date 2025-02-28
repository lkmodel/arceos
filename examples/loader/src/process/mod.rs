//! This module provides the process management API for the operating system.

mod api;
mod process;
mod task_ext;
pub mod flags;

pub use api::*;
pub use process::{Process, PID2PC, TID2TASK};

use axhal::arch::TrapFrame;

/// To write the trap frame into the kernel stack
///
/// # Safety
///
/// It should be guaranteed that the kstack address is valid and writable.
pub fn write_trapframe_to_kstack(kstack_top: usize, trap_frame: &TrapFrame) {
    let trap_frame_size = core::mem::size_of::<TrapFrame>();
    let trap_frame_ptr = (kstack_top - trap_frame_size) as *mut TrapFrame;
    unsafe {
        *trap_frame_ptr = trap_frame.clone();
    }
}

/// To read the trap frame from the kernel stack
///
/// # Safety
///
/// It should be guaranteed that the kstack address is valid and readable.
pub fn read_trapframe_from_kstack(kstack_top: usize) -> TrapFrame {
    let trap_frame_size = core::mem::size_of::<TrapFrame>();
    let trap_frame_ptr = (kstack_top - trap_frame_size) as *mut TrapFrame;
    unsafe { (*trap_frame_ptr).clone() }
}

#[cfg(target_arch = "riscv64")]
/// set the return code
pub fn set_ret_code(tp: &mut TrapFrame, ret_value: usize) {
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