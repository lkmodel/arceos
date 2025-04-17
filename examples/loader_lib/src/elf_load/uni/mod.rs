pub mod head_decoder;
pub mod script_decoder;

use core::slice::{from_raw_parts, from_raw_parts_mut};

use axlog::{debug, info};

use elf::{
    ElfBytes,
    abi::{ET_DYN, ET_EXEC},
    endian::LittleEndian,
};
use head_decoder::head_decoded;
use script_decoder::script_decoded;

use crate::{
    abi::ABI_TABLE,
    config::{APP_START, LIB_START, MAX_APP_SIZE, MAX_LIB_SIZE, PLASH_SIZE, PLASH_START},
    elf_load::{
        load::{load_app_dyn, load_exec, load_lib, modify_lib_main},
        verify::verify_elf_header,
    },
};

pub fn run() {
    info!("Load payload ...");
    debug!("Decode head and script...");
    let head_decoded = head_decoded(PLASH_START, PLASH_SIZE);
    info!("HeadDecoded {:?}", head_decoded);
    let script_decoded = script_decoded(
        PLASH_START + head_decoded.script.1 as usize,
        head_decoded.script.0 as usize,
    );

    info!("Load APP");
    let app = head_decoded.app;

    if app.0 as usize > MAX_APP_SIZE {
        panic!("App size > max app size");
    }
    let app_elf_slice = unsafe {
        from_raw_parts(
            (PLASH_START + (app.2 as usize)) as *const u8,
            app.0 as usize,
        )
    };
    let app_code = unsafe { from_raw_parts_mut((APP_START) as *mut u8, MAX_APP_SIZE) };
    let app_elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(app_elf_slice)
            .expect("Failed to parse ELF at APP file");
    verify_elf_header(&app_elf)
        .is_err()
        .then(|| panic!("Failed to verify_elf_header for App ELF"));

    let arg_entry = script_decoded.argc_ptr();

    // 判断是否需要动态链接器
    let entry = match app_elf.ehdr.e_type {
        ET_EXEC => {
            if head_decoded.lib.0.ne(&0) || head_decoded.lib.1.ne(&0) {
                panic!("Error in Lib part");
            }
            // 在需要动态链接器的时候（静态加载），就可以直接使用App的Entry
            load_exec(&app_elf, app_elf_slice, app_code, APP_START);

            app_elf.ehdr.e_entry as usize
        }
        ET_DYN => {
            if head_decoded.lib.0.eq(&0) || head_decoded.lib.1.eq(&0) {
                panic!("Error in Lib part");
            }
            if head_decoded.lib.0 as usize > MAX_LIB_SIZE {
                panic!("Lib size > max lib size");
            }
            let lib_elf_slice = unsafe {
                from_raw_parts(
                    (PLASH_START + (head_decoded.lib.1 as usize)) as *const u8,
                    head_decoded.lib.0 as usize,
                )
            };
            let lib_code = unsafe { from_raw_parts_mut((LIB_START) as *mut u8, MAX_LIB_SIZE) };
            let lib_elf: ElfBytes<'_, LittleEndian> =
                ElfBytes::<LittleEndian>::minimal_parse(lib_elf_slice)
                    .expect("Failed to parse ELF at LIB file");
            verify_elf_header(&lib_elf)
                .is_err()
                .then(|| panic!("Failed to verify_elf_header for Lib ELF"));

            let lib_entry = load_lib(lib_elf_slice, lib_code, &lib_elf, LIB_START);

            info!(
                "Load lib done, entry 0x{:x} size 0x{:x}",
                lib_entry, head_decoded.lib.0
            );

            // 动态连接加载
            let main_entry = load_app_dyn(
                app_elf_slice,
                &app_elf,
                app_code,
                app.0 as usize,
                APP_START,
                &lib_elf, // ✅ 这里用 &lib.elf 避免 Move
                LIB_START,
                "main",
            );
            modify_lib_main(&lib_elf, LIB_START, main_entry);

            lib_entry // ✅ 这里还是可用
        }
        _ => panic!("Unsupported ehdr type {:?}", app_elf.ehdr.e_type),
    };

    info!(
        "Entry @0x{:x} arg @0x{:x}",
        entry as usize, arg_entry as usize
    );

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
            mv      a0, {param}             // 将参数p的地址加载到a0
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
            param = in(reg) arg_entry,
            entry = in(reg) entry,
            options(nostack, nomem)
        )
    }

    info!("Done app");
}
