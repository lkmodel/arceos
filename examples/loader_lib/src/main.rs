#![feature(c_variadic)]
#![feature(stmt_expr_attributes)]
#![no_std]
#![no_main]
#![feature(f128)]

extern crate arceos_posix_api;
extern crate axstd;
extern crate axstd as std;
extern crate cty;
use std::println;
extern crate alloc;

mod abi;
use abi::{ABI_TABLE, ABI_TERMINATE, init_abis};

mod load;
use init::init_all;
use load::load_elf;

mod elf;
mod init;

mod linux_env;
mod syscall;

mod runtime_func;

mod config;

#[unsafe(no_mangle)]
fn main() {
    init_all();
    init_abis();
    let run_entry = load_elf();
    println!("Entry: 0x{:x} and RUN", run_entry);
    unsafe {
        core::arch::asm!("
            // 保存更多上下文信息
            addi    sp, sp, -144 // 增加栈空间以存储额外的寄存器
            // 保存CPU相关的寄存器
            mv      t0, tp          // 保存CPU_ID
            sd      t0, 0(sp)
            csrr    t0, sstatus     // 保存系统状态
            sd      t0, 8(sp)

            // 保存通用寄存器
            sd      ra, 16(sp)
            sd      a7, 24(sp)
            sd      a6, 32(sp)
            sd      a5, 40(sp)
            sd      a4, 48(sp)
            sd      a3, 56(sp)
            sd      a2, 64(sp)
            sd      a1, 72(sp)
            sd      a0, 80(sp)
            sd      t6, 88(sp)
            sd      t5, 96(sp)
            sd      t4, 104(sp)
            sd      t3, 112(sp)
            sd      t2, 120(sp)
            sd      t1, 128(sp)
            sd      t0, 136(sp)

            la      a7, {abi_table}
            mv      t2, {entry}
            jalr    t2

            // 恢复所有寄存器
            ld      t0, 0(sp)       // 恢复CPU ID
            mv      tp, t0
            ld      t0, 8(sp)       // 恢复系统状态
            csrw    sstatus, t0

            ld      ra, 16(sp)
            ld      a7, 24(sp)
            ld      a6, 32(sp)
            ld      a5, 40(sp)
            ld      a4, 48(sp)
            ld      a3, 56(sp)
            ld      a2, 64(sp)
            ld      a1, 72(sp)
            ld      a0, 80(sp)
            ld      t6, 88(sp)
            ld      t5, 96(sp)
            ld      t4, 104(sp)
            ld      t3, 112(sp)
            ld      t2, 120(sp)
            ld      t1, 128(sp)
            ld      t0, 136(sp)

            addi    sp, sp, 144
            ",
            abi_table = sym ABI_TABLE,
            entry = in(reg) run_entry,
            options(nostack)
        )
    }
    bye();
}

fn bye() -> () {
    unsafe {
        core::arch::asm!("
            li      t0, {abi_exit}
            slli    t0, t0, 3
            la      t1, {abi_table}
            add     t1, t1, t0
            ld      t1, (t1)
            jalr    t1
            j       .",
            abi_exit = const ABI_TERMINATE,
            abi_table = sym ABI_TABLE,
        )
    }
}
