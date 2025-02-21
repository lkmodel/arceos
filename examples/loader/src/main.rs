#![no_std]
#![no_main]
#![feature(c_variadic)]
#![feature(alloc_error_handler)]

extern crate alloc;

use abi::abi_entry;
use axlog::info;

mod abi;
mod elf;
mod load;

use linkme::distributed_slice;
// use heap_allocator::init_heap;
use load::load_elf;

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
    let entry = load_elf();

    info!("Execute app ...");
    unsafe { core::arch::asm!("
        la      a2, {abi_entry}
        mv      t2, {run_start}
        jalr    t2",
        abi_entry = sym abi_entry,
        run_start = in(reg) entry,
        clobber_abi("C"),
    )}
}
