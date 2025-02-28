#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(c_size_t)]
extern crate alloc;

mod abi;
mod config;
mod elf;
mod process;

use core::{fmt::Display, slice::from_raw_parts, sync::atomic::{AtomicUsize, Ordering}};

use alloc::string::ToString;
use alloc::vec::Vec;
use axlog::info;
use axstd::println;
use axtask::{current, exit, WaitQueue};
use elf::PLASH_START;
use linkme::distributed_slice;
use process::Process;

// 全局等待队列
pub static MAIN_WAIT_QUEUE: WaitQueue = WaitQueue::new();    // main线程等待所有进程结束
pub static FORK_WAIT: WaitQueue = WaitQueue::new();          // 父进程等待子进程开始执行 

// 进程计数
pub static PROCESS_COUNT: AtomicUsize = AtomicUsize::new(0);

// 保存原始内核的 GP 和应用的 GP
pub static APP_GP: AtomicUsize = AtomicUsize::new(0);
pub static KERNEL_GP: AtomicUsize = AtomicUsize::new(0);

#[distributed_slice]
pub static ABI_TABLE: [AbiEntry] = [..];

#[repr(C)]
pub struct AbiEntry {
    pub name: &'static str,
    addr: *const (),
}
unsafe impl Sync for AbiEntry {}

#[unsafe(no_mangle)]
fn main() {
    let fns = ABI_TABLE.iter().map(|table| table.name).collect::<Vec<&str>>();
    info!("Existed Abi functions:{}", fns.join(","));
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

#[repr(C)]
#[derive(Debug, Clone)]
pub struct UserContext {
    pub ra: usize,
    pub sp: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub tp: usize,
}

impl UserContext {
    pub const fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }
}

impl Display for UserContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UserContext {{ ra: 0x{:x}, sp: 0x{:x}, s0: 0x{:x}, s1: 0x{:x}, s2: 0x{:x}, s3: 0x{:x}, s4: 0x{:x}, s5: 0x{:x}, s6: 0x{:x}, s7: 0x{:x}, s8: 0x{:x}, s9: 0x{:x}, s10: 0x{:x}, s11: 0x{:x}, tp: 0x{:x} }}",
            self.ra, self.sp, self.s0, self.s1, self.s2, self.s3, self.s4, self.s5, self.s6, self.s7, self.s8, self.s9, self.s10, self.s11, self.tp
        )
    }
}