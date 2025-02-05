pub mod riscv64;

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