#![feature(c_variadic)]
#![no_std]
#![no_main]
extern crate arceos_posix_api;
extern crate axstd;
extern crate axstd as std;
extern crate cty;
extern crate printf_compat;
use std::println;

mod abi;
use abi::{ABI_TABLE, SYS_TERMINATE, init_abis};

mod load;
use init::init_loader_lib;
use load::{APP_START, load_elf};

mod elf;
mod init;

#[unsafe(no_mangle)]
fn main() {
    // init_loader_lib();
    init_abis();
    let entry = load_elf();
    println!("Entry: 0x{:x} and RUN", entry);

    unsafe {
        core::arch::asm!("
                la      a7, {abi_table}

                addi    sp, sp, -128
                sd      ra, 0(sp)
                sd      a7, 8(sp)
                sd      a6, 16(sp)
                sd      a5, 24(sp)
                sd      a4, 32(sp)
                sd      a3, 40(sp)
                sd      a2, 48(sp)
                sd      a1, 56(sp)
                sd      a0, 64(sp)
                sd      t6, 72(sp)
                sd      t5, 80(sp)
                sd      t4, 98(sp)
                sd      t3, 104(sp)
                sd      t2, 112(sp)
                sd      t1, 120(s0)
                sd      t0, 128(sp)

                mv      t2, {entry}
                jalr    t2

                ld      ra, 0(sp)
                ld      a7, 8(sp)
                ld      a6, 16(sp)
                ld      a5, 24(sp)
                ld      a4, 32(sp)
                ld      a3, 40(sp)
                ld      a2, 48(sp)
                ld      a1, 56(sp)
                ld      a0, 64(sp)
                ld      t6, 72(sp)
                ld      t5, 80(sp)
                ld      t4, 98(sp)
                ld      t3, 104(sp)
                ld      t2, 112(sp)
                ld      t1, 120(s0)
                ld      t0, 128(sp)
                addi    sp, sp, 128
                ",
            abi_table = sym ABI_TABLE,
            entry = in(reg) entry,
        )
    }
    bye();
}

fn bye() -> () {
    unsafe {
        core::arch::asm!("
          li      t0, {abi_num}
          slli    t0, t0, 3
          la      t1, {abi_table}
          add     t1, t1, t0
          ld      t1, (t1)
          jalr    t1
          li      t2, {run_start}
          jalr    t2
          j       .",
            run_start = const APP_START,
            abi_table = sym ABI_TABLE,
            abi_num = const SYS_TERMINATE,
        )
    }
}
