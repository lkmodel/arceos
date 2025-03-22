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
mod runtime_func;
mod syscall;

use alloc::{ffi::CString, string::ToString, vec::Vec};
use axlog::info;
use axstd::{println, process::exit};
use axtask::{current, init_scheduler};
use core::{
    ptr,
    slice::{from_raw_parts, from_raw_parts_mut},
    sync::atomic::{AtomicUsize, Ordering},
};
use cty::{c_int, c_long, uintptr_t};
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
    elf_load::uni_load::load_elf,
    init::init_all,
    // linux_env::linux_fs::api::UniAPI,
};

// 准备参数 p，这里我们直接在代码中指定参数
static mut PARAMS: [u64; 10] = [0; 10];

// fn call_c_start(args: &[&str], entry: usize) {
//     let argc = args.len() as c_int;
//     let mut argv_pointers = Vec::new();
//     let mut arg_strings = Vec::new();
//
//     for arg in args {
//         match CString::new(*arg) {
//             Ok(c_str) => {
//                 argv_pointers.push(c_str.as_ptr() as uintptr_t);
//                 arg_strings.push(c_str);
//             }
//             Err(e) => {
//                 panic!("Error creating CString: {}", e);
//             }
//         }
//     }
//
//     let mut params: Vec<c_long> = Vec::new();
//     params.push(argc as c_long);
//     for ptr in argv_pointers {
//         params.push(ptr as c_long);
//     }
//     params.push(ptr::null() as c_long); // Optional null terminator for `argv`
//
//     unsafe {
//         // 这里entry就是执行函数的地址
//         let func: extern "C" fn(*const c_long) = axstd::mem::transmute(entry);
//         func(params.as_ptr());
//     }
//
//     // 'arg_strings' Vec 在此处被 drop，这将释放 CString 实例。
//     // 只要 C 代码不需要保留这些字符串，这就是安全的。
// }

#[unsafe(no_mangle)]
fn main() {
    #[cfg(not(any(feature = "unikernel", feature = "pseudo_multi_process")))]
    compile_error!("You must enable exactly one of `unikernel` or `multi_process_unchecked`.");

    #[cfg(all(feature = "unikernel", feature = "pseudo_multi_process"))]
    compile_error!(
        "You cannot enable both `unikernel` and `multi_process_unchecked` at the same time."
    );

    #[cfg(feature = "pseudo_multi_process")]
    {
        info!("Load payload ...");
        init_abis();
        // Load X out file
        let app_elf_size = unsafe { *(PLASH_START as *const usize) };
        info!("app_elf_size 0x{:x}", app_elf_size);
        app_elf_size
            .ge(&MAX_APP_SIZE)
            .then(|| panic!("app elf size > MAP_APP_SIZE"));
        let app_elf_slice =
            unsafe { from_raw_parts((PLASH_START + 0x8) as *const u8, app_elf_size) };

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
    }

    #[cfg(feature = "unikernel")]
    {
        init_all();
        init_abis();
        let run_entry = load_elf();

        unsafe {
            PARAMS[0] = 0;
            PARAMS[1] = "arg1\0".as_ptr() as u64;
        }

        println!("Entry: 0x{:x} and RUN", run_entry);
        unsafe {
            core::arch::asm!("
            // 保存更多上下文信息
            addi    sp, sp, -128

            // 保存通用寄存器
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
            sd      t4, 88(sp)
            sd      t3, 96(sp)
            sd      t2, 104(sp)
            sd      t1, 112(sp)
            sd      t0, 120(sp)

            mv      t2, {entry}
            la      a0, {param}     // 将参数p的地址加载到a0
            la      a7, {abi_table}
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
            ld      t4, 88(sp)
            ld      t3, 96(sp)
            ld      t2, 104(sp)
            ld      t1, 112(sp)
            ld      t0, 120(sp)

            addi    sp, sp, 128
            ",
                abi_table = sym ABI_TABLE,
                param = sym PARAMS,
                entry = in(reg) run_entry,
                options(nostack)
            )
        }
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
