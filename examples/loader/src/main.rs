#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]

extern crate alloc;

mod abi;
mod config;
mod elf;
mod fs;
mod process;

use core::{slice::from_raw_parts, sync::atomic::{AtomicUsize, Ordering}};

use alloc::string::ToString;
use axlog::info;
use axstd::println;
use axtask::{current, exit, WaitQueue};
use elf::PLASH_START;
use process::Process;

// 在全局添加一个 WaitQueue
pub static MAIN_WAIT_QUEUE: WaitQueue = WaitQueue::new();
pub static PARENT_WAIT_QUEUE: WaitQueue = WaitQueue::new();
pub static FORK_WAIT: WaitQueue = WaitQueue::new();
// 保存原始内核的 GP 和应用的 GP
pub static APP_GP: AtomicUsize = AtomicUsize::new(0);
pub static KERNEL_GP: AtomicUsize = AtomicUsize::new(0);

#[unsafe(no_mangle)]
fn main() {
    println!("Load payload ...");
    let elf_size = unsafe { *(PLASH_START as *const usize) };

    println!("ELF size: 0x{:x}", elf_size);

    let elf_slice = unsafe { from_raw_parts((PLASH_START + 0x8) as *const u8, elf_size) };

    unsafe {
        save_gp(&KERNEL_GP);
    }

    info!("Kernel GP: 0x{:x}", KERNEL_GP.load(Ordering::SeqCst));

    info!("Execute payload {:?}", current().id());
    
    Process::init("fork".to_string(), elf_slice);

    println!("Execute payload done!");

    exit(0);
}

#[inline(never)]
pub unsafe fn save_gp(gp: &AtomicUsize) {
    let current_gp: usize;
    unsafe {
        core::arch::asm!(
            "mv {}, gp",
            out(reg) current_gp,
            options(nomem, nostack)
        );
    }
    info!("Saving GP: 0x{:x}", current_gp);
    gp.store(current_gp, Ordering::SeqCst);
}

#[inline(never)]
pub unsafe fn switch_to_gp(gp: &AtomicUsize) {
    let saved_gp = gp.load(Ordering::SeqCst);
    info!("Switching to other GP: 0x{:x}", saved_gp);
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option norelax",
            "mv gp, {0}",
            ".option pop",
            in(reg) saved_gp,
            options(nomem, nostack)
        );
    }
}

pub unsafe fn switch_to_usize(gp: usize) {
    info!("Switching to other GP: 0x{:x}", gp);
    unsafe {
        core::arch::asm!(
            ".option push",
            ".option norelax",
            "mv gp, {0}",
            ".option pop",
            in(reg) gp,
            options(nomem, nostack)
        );
    }
}

pub unsafe fn current_gp() -> usize {
    let current_gp: usize;
    unsafe {
        core::arch::asm!(
            "mv {}, gp",
            out(reg) current_gp,
            options(nomem, nostack)
        );
    }
    current_gp
}

pub unsafe fn get_saved_gp(gp: &AtomicUsize) -> usize {
    gp.load(Ordering::SeqCst)
}
