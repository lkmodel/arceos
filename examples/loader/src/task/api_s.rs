//! Task APIs for single-task configuration.

use crate::config::TASK_STACK_SIZE;

/// For single-task situation, we just relax the CPU and wait for incoming
/// interrupts.
pub fn yield_now() {
    if cfg!(feature = "irq") {
        axhal::arch::wait_for_irqs();
    } else {
        core::hint::spin_loop();
    }
}

/// For single-task situation, we just busy wait for the given duration.
pub fn sleep(dur: core::time::Duration) {
    axhal::time::busy_wait(dur);
}

/// For single-task situation, we just busy wait until reaching the given
/// deadline.
pub fn sleep_until(deadline: axhal::time::TimeValue) {
    axhal::time::busy_wait_until(deadline);
}

// arch_boot
unsafe extern "C" {
    fn current_boot_stack() -> *mut u8;
}

pub fn global_unique_ts() -> (usize, usize) {
    let boot_stack = unsafe {
        current_boot_stack() as usize
    };
    (boot_stack, boot_stack + TASK_STACK_SIZE)
}