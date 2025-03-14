#![no_std]
#![no_main]
#![feature(f128)]
#![feature(c_variadic)]
#![feature(stmt_expr_attributes)]
#![feature(naked_functions)]
#![feature(alloc_error_handler)]

extern crate alloc;
extern crate arceos_posix_api;
extern crate axstd;
extern crate cty;

mod abi;
mod config;
mod elf_load;
mod init;
mod linux_env;
mod load;
mod runtime_func;
mod syscall;

use alloc::string::ToString;
use axlog::info;
use axstd::{println, process::exit};
use axtask::current;
use core::{
    slice::{from_raw_parts, from_raw_parts_mut},
    sync::atomic::{AtomicUsize, Ordering},
};
use elf::{
    ElfBytes,
    abi::{ET_DYN, ET_EXEC},
    endian::LittleEndian,
};
use linux_env::process_ext::{KERNEL_GP, context::save_gp, process::Process};

use crate::{
    abi::{ABI_TABLE, ABI_TERMINATE, init_abis},
    config::{MAX_APP_SIZE, MAX_LIB_SIZE, PLASH_START},
    elf_load::load::load_user_app,
    init::init_all,
    // linux_env::linux_fs::api::UniAPI,
    load::load_elf,
};

#[unsafe(no_mangle)]
fn main() {
    #[cfg(not(any(feature = "unikernel", feature = "pseudo_multi_process")))]
    compile_error!("You must enable exactly one of `unikernel` or `multi_process_unchecked`.");

    #[cfg(all(feature = "unikernel", feature = "pseudo_multi_process"))]
    compile_error!(
        "You cannot enable both `unikernel` and `multi_process_unchecked` at the same time."
    );

    info!("Load payload ...");
    init_abis();
    // Load X out file
    let app_elf_size = unsafe { *(PLASH_START as *const usize) };
    info!("app_elf_size 0x{:x}", app_elf_size);
    app_elf_size
        .ge(&MAX_APP_SIZE)
        .then(|| panic!("app elf size > MAP_APP_SIZE"));
    let app_elf_slice = unsafe { from_raw_parts((PLASH_START + 0x8) as *const u8, app_elf_size) };

    let app_elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(app_elf_slice).expect("Failed to parse ELF");

    match app_elf.ehdr.e_type {
        ET_DYN => {
            let lib_elf_size = unsafe { *((PLASH_START + app_elf_size + 0x8) as *const usize) };

            info!("lib_elf_size 0x{:x}", lib_elf_size);
            lib_elf_size
                .ge(&MAX_LIB_SIZE)
                .then(|| panic!("lib elf size > MAX LIB SIZE"));

            let lib_elf_slice = unsafe {
                from_raw_parts(
                    (PLASH_START + app_elf_size + 0x10) as *const u8,
                    lib_elf_size,
                )
            };

            unsafe {
                save_gp(&KERNEL_GP);
            }

            info!("Kernel GP: 0x{:x}", KERNEL_GP.load(Ordering::SeqCst));

            info!("Execute payload {:?}", current().id());

            Process::init("sqlite".to_string(), app_elf_slice, Some(lib_elf_slice));
        }
        ET_EXEC => {
            unimplemented!();
        }
        _ => {
            panic!("Unexpected e_type {}", app_elf.ehdr.e_type);
        }
    }

    println!("Execute payload done!");

    exit(0);

    //======

    // init_all();
    // init_abis();
    // let run_entry = load_elf();
    // println!("Entry: 0x{:x} and RUN", run_entry);
    // unsafe {
    //     core::arch::asm!("
    //         // 保存更多上下文信息
    //         addi    sp, sp, -144 // 增加栈空间以存储额外的寄存器
    //         // 保存CPU相关的寄存器
    //         mv      t0, tp          // 保存CPU_ID
    //         sd      t0, 0(sp)
    //         csrr    t0, sstatus     // 保存系统状态
    //         sd      t0, 8(sp)

    //         // 保存通用寄存器
    //         sd      ra, 16(sp)
    //         sd      a7, 24(sp)
    //         sd      a6, 32(sp)
    //         sd      a5, 40(sp)
    //         sd      a4, 48(sp)
    //         sd      a3, 56(sp)
    //         sd      a2, 64(sp)
    //         sd      a1, 72(sp)
    //         sd      a0, 80(sp)
    //         sd      t6, 88(sp)
    //         sd      t5, 96(sp)
    //         sd      t4, 104(sp)
    //         sd      t3, 112(sp)
    //         sd      t2, 120(sp)
    //         sd      t1, 128(sp)
    //         sd      t0, 136(sp)

    //         la      a7, {abi_table}
    //         mv      t2, {entry}
    //         jalr    t2

    //         // 恢复所有寄存器
    //         ld      t0, 0(sp)       // 恢复CPU ID
    //         mv      tp, t0
    //         ld      t0, 8(sp)       // 恢复系统状态
    //         csrw    sstatus, t0

    //         ld      ra, 16(sp)
    //         ld      a7, 24(sp)
    //         ld      a6, 32(sp)
    //         ld      a5, 40(sp)
    //         ld      a4, 48(sp)
    //         ld      a3, 56(sp)
    //         ld      a2, 64(sp)
    //         ld      a1, 72(sp)
    //         ld      a0, 80(sp)
    //         ld      t6, 88(sp)
    //         ld      t5, 96(sp)
    //         ld      t4, 104(sp)
    //         ld      t3, 112(sp)
    //         ld      t2, 120(sp)
    //         ld      t1, 128(sp)
    //         ld      t0, 136(sp)

    //         addi    sp, sp, 144
    //         ",
    //         abi_table = sym ABI_TABLE,
    //         entry = in(reg) run_entry,
    //         options(nostack)
    //     )
    // }
    // bye();
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
