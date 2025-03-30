use core::cmp::min;

use axstd::format;

use axlog::{debug, info};

use elf::{
    ElfBytes,
    abi::{PT_LOAD, R_RISCV_64, R_RISCV_JUMP_SLOT, R_RISCV_RELATIVE},
    endian::LittleEndian,
};

// pub fn load_elf() -> u64 {
//     debug!("Load payload ...");
//     // Load X out file
//     let app_elf_size = unsafe { *(PLASH_START as *const usize) };
//     debug!("app_elf_size 0x{:x}", app_elf_size);
//     if app_elf_size >= MAX_APP_SIZE {
//         panic!("app elf size > MAP_APP_SIZE");
//     }
//     let app_elf_slice = unsafe { from_raw_parts((PLASH_START + 0x8) as *const u8, app_elf_size) };
//     let app_code = unsafe { from_raw_parts_mut((APP_START) as *mut u8, MAX_APP_SIZE) };
//
//     let app_elf: ElfBytes<'_, LittleEndian> =
//         ElfBytes::<LittleEndian>::minimal_parse(app_elf_slice).expect("Failed to parse ELF");
//
//     let is_need_interp = {
//         if let Some(segments) = app_elf.segments() {
//             let mut is_pie = false;
//             for segment in segments {
//                 debug!("Segment type: {}", segment.p_type);
//                 if segment.p_type == elf::abi::PT_INTERP {
//                     is_pie = true;
//                 }
//             }
//             is_pie
//         } else {
//             false
//         }
//     };
//     debug!(
//         "Dynamic interpreter (.interp section) exists: {}",
//         is_need_interp
//     );
//
//     let entry: u64 = {
//         if is_need_interp == false {
//             // Static and position independent executable
//             debug!("Static and position independent app");
//             let _ = load_exec(&app_elf, app_elf_slice, app_code);
//             app_elf.ehdr.e_entry
//         } else {
//             debug!("Dynamic link app");
//             let lib_elf_size = unsafe { *((PLASH_START + app_elf_size + 0x8) as *const usize) };
//             if lib_elf_size > MAX_LIB_SIZE {
//                 panic!("lib elf size > MAP LIB SIZE");
//             }
//             let lib_elf_slice = unsafe {
//                 from_raw_parts(
//                     (PLASH_START + app_elf_size + 0x10) as *const u8,
//                     lib_elf_size,
//                 )
//             };
//             let lib_code = unsafe { from_raw_parts_mut((LIB_START) as *mut u8, MAX_LIB_SIZE) };
//
//             let lib_elf: ElfBytes<'_, LittleEndian> =
//                 ElfBytes::<LittleEndian>::minimal_parse(lib_elf_slice)
//                     .expect("Failed to parse ELF at LIB file");
//
//             debug!(
//                 "ELF Headers App: 0x{:x}, Lib: 0x{:x}",
//                 app_elf.ehdr.e_ehsize, lib_elf.ehdr.e_ehsize
//             );
//
//             debug!("Load lib to mem space");
//             load_dyn(&lib_elf, lib_elf_slice, lib_code, 0);
//             debug!("Load app to mem space");
//             load_dyn(&app_elf, app_elf_slice, app_code, 0);
//
//             modify_plt_for_app(&app_elf, &lib_elf);
//             modify_plt_for_lib(&app_elf, &lib_elf);
//
//             println!("Lib elf size: 0x{:x}", lib_elf_size);
//             // FIX: 检查一下
//             // NOTE:
//             // 正常情况下，应该是由APP内的start函数开始执行，但是因为我们是unikernel,直接执行也OK？
//             LIB_START as u64 + lib_elf.ehdr.e_entry
//             // APP_START as u64 + app_elf.ehdr.e_entry
//         }
//     };
//
//     println!("App elf size: 0x{:x}", app_elf_size);
//     return entry;
// }
//
// fn load_exec(
//     app_elf: &ElfBytes<LittleEndian>,
//     app_elf_slice: &[u8],
//     app_code: &mut [u8],
// ) -> Result<(), LoadError> {
//     // 检查 ELF 头
//     verify_elf_header(app_elf).expect("Failed to verify ELF header");
//
//     if let Some(phs) = app_elf.segments() {
//         for ph in phs {
//             if ph.p_type != PT_LOAD {
//                 debug!("skipping segment type: {}", ph.p_type);
//                 continue;
//             }
//
//             let offset = ph.p_offset as usize;
//             let filesz = ph.p_filesz as usize;
//             let memsz = ph.p_memsz as usize;
//
//             // 计算在内存中的实际地址
//             let vaddr = ph.p_vaddr as usize;
//             let dest_addr = vaddr - APP_START;
//
//             debug!(
//                 "Loading segment: offset=0x{:x}, filesz=0x{:x}, memsz=0x{:x}, vaddr=0x{:x}",
//                 offset, filesz, memsz, vaddr
//             );
//
//             debug!(
//                 "dest_addr: {:x} = vaddr({:x}) - APP_START({:x})",
//                 dest_addr, vaddr, APP_START
//             );
//
//             // 复制段内容
//             if filesz > 0 {
//                 let src: &[u8] = &app_elf_slice[offset..offset + filesz];
//                 let dest = &mut app_code[dest_addr..dest_addr + filesz];
//                 dest.copy_from_slice(src);
//             }
//
//             // 处理`.bss`等需要零初始化的部分
//             if memsz > filesz {
//                 let dest = &mut app_code[dest_addr + filesz..dest_addr + memsz];
//                 dest.fill(0);
//             }
//         }
//     }
//     /* ```
//         let text_shdr = app_elf
//             .section_header_by_name(".text")
//             .expect("section table should be parseable")
//             .expect("elf should have a .text section");
//         let text_slice = app_elf_slice
//             .get(text_shdr.sh_offset as usize..)
//             .expect("text section should be in bounds");
//         let copy_size = min(app_code.len(), text_slice.len());
//         app_code[..copy_size].copy_from_slice(&text_slice[..copy_size]);
//     ```*/
//     Ok(())
// }

// 往后的内容是新的

pub fn load_dyn(
    elf: &ElfBytes<LittleEndian>,
    elf_slice: &[u8],
    run_code: &mut [u8],
    address_bios: usize,
) {
    let phdrs = elf.segments().expect("Failed to parse program headers");
    for phdr in phdrs {
        if phdr.p_type != PT_LOAD {
            continue;
        }
        debug!(
            "Load Segment vaddr 0x{:x} offset 0x{:x} filesz 0x{:x} memsz 0x{:x} address_bios 0x{:x}",
            phdr.p_vaddr as usize,
            phdr.p_offset as usize,
            phdr.p_offset as usize,
            phdr.p_memsz as u64,
            address_bios
        );
        load_segment(
            run_code,
            elf_slice,
            phdr.p_vaddr as usize,
            phdr.p_offset as usize,
            phdr.p_filesz as usize,
            phdr.p_memsz as usize,
            address_bios,
        );
    }
}

pub fn load_segment(
    run_code: &mut [u8],
    elf_slice: &[u8],
    p_vaddr: usize,
    p_offset: usize,
    p_filesz: usize,
    p_memsz: usize,
    address_bios: usize,
) {
    // Copy the segment into the executable zone
    // If `memsz` is larger than `filesz`, zero out the rest
    let run_code_offset = p_vaddr - address_bios;
    run_code[run_code_offset..run_code_offset + p_filesz]
        .copy_from_slice(&elf_slice[p_offset..p_offset + p_filesz]);
    if p_memsz > p_filesz {
        let zero_size = min(run_code.len() - p_filesz, p_memsz - p_filesz);
        run_code[run_code_offset + p_filesz..run_code_offset + p_filesz + zero_size].fill(0);
    }
}

/// 加载 `APP` 库
///
/// # 参数
/// * `app_elf_slice` - `APP` 基于在 `PLASH` 中的入口地址与在 `PLASH` 中的大小，
/// 获取其在 `PLASH` 中的切片。
/// * `app_code` - `APP` 基于在 `MEM` 中的入口地址与在 `MEM` 中的大小，
/// 获取其在 `MEM` 中的切片。
/// * `app_size` - `APP` 在 `PLASH` 中的大小
/// * `app_start` - `APP` 在 `MEM` 中的入口地址
/// * `lib_elf` - `LIB` 基于在 `PLASH` 的入口地址和大小，获得其切片，转化为 `ElfBytes`
/// * `lib_start` - `LIB` 在 `MEM` 中的入口地址
///
/// # 返回值
/// 返回 `main` 函数入口地址
pub fn load_app_dyn(
    app_elf_slice: &[u8],
    app_elf: &ElfBytes<LittleEndian>,
    app_code: &mut [u8],
    app_size: usize,
    app_start: usize,
    lib_elf: &ElfBytes<LittleEndian>,
    lib_start: usize,
    entry_name: &str,
) -> usize {
    debug!("Load app to mem space");
    load_dyn(&app_elf, app_elf_slice, app_code, 0);

    debug!("Modify lib to mem space");
    modify_app(&app_elf, app_start, &lib_elf, lib_start);

    info!("App elf size: 0x{:x}", app_size);
    find_app_main_entry(&app_elf, app_start, entry_name)
}

/// 加载 `LIB` 库
///
/// # 参数
/// * `lib_size` - `LIB` 在 `PLASH` 中的大小
/// * `lib_entry` - `LIB` 在 `PLASH` 中的入口地址
/// * `lib_start` - `LIB` 在 `MEM` 中的入口地址
/// * `max_lib_size` - `LIB` 在 `MEM` 中的最大大小
///
/// # 返回值
/// `Lib` 库的入口函数
pub fn load_lib(
    lib_elf_slice: &[u8],
    lib_code: &mut [u8],
    lib_elf: &ElfBytes<LittleEndian>,
    lib_start: usize,
) -> usize {
    debug!("Load lib to mem space");
    load_dyn(&lib_elf, lib_elf_slice, lib_code, 0);

    debug!("Modify lib to mem space");
    modify_lib(&lib_elf, lib_start);

    // 正常情况下，应该是由 `APP` 内的 `start` 函数开始执行，
    // 但是我们在 `Batch Mode` 下需要重新写 `start` 函数，因此从这里开始
    return lib_start + lib_elf.ehdr.e_entry as usize;
}

/// 传递一个 `APP` 的 `ElfBytes`，查询某一个函数的入口地址，返回
/// # 参数
/// * `elf` - `APP` 基于在 `PLASH` 的入口地址和大小，获得其切片，转化为 `ElfBytes`
/// * `app_start` - `APP` 在 `MEM` 中的入口地址
/// * `entry_name` - 待查询的入口地址，通常为 `main`
fn find_app_main_entry(elf: &ElfBytes<LittleEndian>, app_start: usize, entry_name: &str) -> usize {
    let (dynsym_table, dynstr_table) = elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");

    debug!("Finding main entry");

    let sym = dynsym_table
        .iter()
        .find(|sym| {
            let name = dynstr_table
                .get(sym.st_name as usize)
                .expect("Failed to get name in dynstr_table");
            name == entry_name
        })
        .expect("Failed to find symbol in APP dynamic symbol table");
    sym.st_value.eq(&0).then(|| panic!("Bad main entry"));
    app_start + sym.st_value as usize
}

/// 这个是用来修改 `LIB` 库的 `PLT` 表格的，但是不会修改 `main` 函数。
///
/// # 参数
/// * `elf` -
/// * `lib_start` - `LIB` 在内存中地址
fn modify_lib(elf: &ElfBytes<LittleEndian>, lib_start: usize) {
    let (dynsym_table, dynstr_table) = elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");

    info!("modify for LIB-RELA.PLT so lib");
    let rela_plt_shdr = elf
        .section_header_by_name(".rela.plt")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.plt section");
    let rela_plts = elf
        .section_data_as_relas(&rela_plt_shdr)
        .expect("Failed to parse .rela.plt section");

    let run_code_entry_name = "main";

    for rela_plt in rela_plts {
        let sym = dynsym_table.get(rela_plt.r_sym as usize).expect(&format!(
            "Failed to get symbol for index: {}",
            rela_plt.r_sym
        ));
        let rela_name = dynstr_table.get(sym.st_name as usize).expect(&format!(
            "Failed to get symbol name for index: {}",
            sym.st_name
        ));

        match rela_plt.r_type {
            // Indicates the symbol associated with a `PLT` entry: `S`
            R_RISCV_JUMP_SLOT => {
                // 在遇到 `main` 函数的时候，跳过。
                if rela_name != run_code_entry_name {
                    let relative_offset = lib_start + rela_plt.r_offset as usize;
                    let new_value = lib_start + sym.st_value as usize;
                    debug!(
                        "[Lib-rela.plt R_RISCV_JUMP_SLOT] @0x{:x}=0x{:x} st_name {}",
                        relative_offset, new_value, rela_name,
                    );
                    sym.st_value.eq(&0).then(|| panic!("Bad st_value"));
                    unsafe { *(relative_offset as *mut usize) = new_value };
                }
            }
            _ => {
                panic!("Unknown relocation type: {}", rela_plt.r_type);
            }
        }
    }

    info!("modify for LIB-RELA.DYN so lib");
    let rela_dyn_shdr = elf
        .section_header_by_name(".rela.dyn")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.dyn section");
    let rela_dyns = elf
        .section_data_as_relas(&rela_dyn_shdr)
        .expect("Failed to parse .rela.dyn section");
    for rela_dyn in rela_dyns {
        let sym = dynsym_table.get(rela_dyn.r_sym as usize).expect(&format!(
            "Failed to get symbol for index: {}",
            rela_dyn.r_sym
        ));
        let rela_name = dynstr_table.get(sym.st_name as usize).expect(&format!(
            "Failed to get symbol name for index: {}",
            sym.st_name
        ));

        match rela_dyn.r_type {
            // Adjust a link address (A) to its load address: `(B + A)`.
            R_RISCV_RELATIVE => {
                let relative_offset = lib_start + rela_dyn.r_offset as usize;
                let new_value = lib_start + rela_dyn.r_addend as usize;
                debug!(
                    "[Lib-rela.dyn R_RISCV_RELATIVE] @0x{:x}=0x{:x}",
                    relative_offset, new_value,
                );
                rela_dyn.r_addend.eq(&0).then(|| panic!("Bad st_value"));
                unsafe { *(relative_offset as *mut usize) = new_value };
            }
            // 64-bit relocation: `S + A`.
            R_RISCV_64 => {
                let relative_offset = lib_start + rela_dyn.r_offset as usize;
                let new_value = lib_start + sym.st_value as usize;
                debug!(
                    "[Lib-rela.dyn R_RISCV_64] @0x{:x}=0x{:x} name {}",
                    relative_offset, new_value, rela_name,
                );
                sym.st_value.eq(&0).then(|| panic!("Bad st_value"));
                unsafe { *(relative_offset as *mut usize) = new_value };
            }
            _ => {
                panic!("Unknown relocation type: {}", rela_dyn.r_type);
            }
        }
    }
}

/// 修改 `Lib` 库中的 `main` 的入口地址
///
/// # 参数
/// * `elf` - `LIB` 的 `ElfBytes`
/// * `lib_start` - `LIB` 在内存中地址
/// * `main_entry` - 对应的 `APP` 库的 `main` 函数入口地址
pub fn modify_lib_main(elf: &ElfBytes<LittleEndian>, lib_start: usize, main_entry: usize) {
    let (dynsym_table, dynstr_table) = elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");

    info!("Modify `main` function entry in lib");
    // 获取 `.rela.plt` section
    let rela_plt_shdr = elf
        .section_header_by_name(".rela.plt")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.plt section");
    // 解析 `.rela.plt` 中的重定位条目
    let rela_plts = elf
        .section_data_as_relas(&rela_plt_shdr)
        .expect("Failed to parse .rela.plt section");

    let run_code_entry_name = "main";

    let rela_plt = rela_plts
        .into_iter()
        .find(|rela_plt| {
            let sym = dynsym_table.get(rela_plt.r_sym as usize).expect(&format!(
                "Failed to get symbol for index: {}",
                rela_plt.r_sym
            ));
            let rela_name = dynstr_table.get(sym.st_name as usize).expect(&format!(
                "Failed to get symbol name for index: {}",
                sym.st_name
            ));

            run_code_entry_name == rela_name
        })
        .expect("Failed to find `main` in LIB dynamic symbol table");

    match rela_plt.r_type {
        // Indicates the symbol associated with a `PLT` entry: `S`
        R_RISCV_JUMP_SLOT => {
            let relative_offset = lib_start + rela_plt.r_offset as usize;
            info!(
                "[Lib-rela.plt ENTRY] @0x{:x}=0x{:x}",
                relative_offset, main_entry
            );
            unsafe { *(relative_offset as *mut usize) = main_entry };
        }
        _ => {
            panic!("Unknown relocation type: {}", rela_plt.r_type);
        }
    }
}

/// 这个是用来修改 `APP` 库的 `PLT` 表格的
///
/// # 参数
/// * `app_elf` - `APP` 文件本身
/// * `app_start` - 内存中 `APP` 的起始地址
/// * `lib_elf` - `LIB` 文件本身
/// * `lib_start` - 内存中 `LIB` 的起始地址
fn modify_app(
    app_elf: &ElfBytes<LittleEndian>,
    app_start: usize,
    lib_elf: &ElfBytes<LittleEndian>,
    lib_start: usize,
) {
    let (app_dynsym_table, app_dynstr_table) = app_elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");
    let (lib_dynsym_table, lib_dynstr_table) = lib_elf
        .dynamic_symbol_table()
        .expect("Failed to parse LIB dynamic symbol table")
        .expect("LIB ELF should have a dynamic symbol table");

    info!("modify for APP-RELA.PLT run code");
    let app_rela_plt_shdr = app_elf
        .section_header_by_name(".rela.plt")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.plt section");
    let app_rela_plts = app_elf
        .section_data_as_relas(&app_rela_plt_shdr)
        .expect("Failed to parse .rela.dyn section");

    for app_rela_plt in app_rela_plts {
        // Get the `r_sym'th` symbol from the dynamic symbol table
        let app_sym = app_dynsym_table
            .get(app_rela_plt.r_sym as usize)
            .expect("Failed to get symbol");
        let app_rela_name = app_dynstr_table
            .get(app_sym.st_name as usize)
            .expect("Failed to get symbol name");

        match app_rela_plt.r_type {
            // Indicates the symbol associated with a `PLT` entry: `S`
            R_RISCV_JUMP_SLOT => {
                debug!("Finding symbol name {}", app_rela_name);
                // Find symbol in LIB ELF
                let lib_sym = lib_dynsym_table
                    .iter()
                    .find(|s| {
                        let name = lib_dynstr_table.get(s.st_name as usize).unwrap_or(&"");
                        // 如果开始寻找`__libc_start_main`，就替换成我们的实现
                        name == app_rela_name
                    })
                    .expect("Failed to find symbol in LIB dynamic symbol table");

                let relative_offset = app_start + app_rela_plt.r_offset as usize;
                let new_value = lib_start + lib_sym.st_value as usize;
                debug!(
                    "[App-rela.plt R_RISCV_JUMP_SLOT] @0x{:x}=0x{:x} st_name {}",
                    relative_offset, new_value, app_rela_name,
                );
                lib_sym.st_value.eq(&0).then(|| panic!("Bad st_value"));
                unsafe { *(relative_offset as *mut usize) = new_value };
            }
            _ => {
                panic!("Unknown relocation type: {}", app_rela_plt.r_type);
            }
        }
    }

    info!("modify for APP-RELA.DYN run code");
    let app_rela_dyn_shdr = app_elf
        .section_header_by_name(".rela.dyn")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.dyn section");
    let app_rela_dyns = app_elf
        .section_data_as_relas(&app_rela_dyn_shdr)
        .expect("Failed to parse .rela.dyn section");

    for app_rela_dyn in app_rela_dyns {
        let app_sym = app_dynsym_table
            .get(app_rela_dyn.r_sym as usize)
            .expect(&format!(
                "Failed to get symbol for index: {}",
                app_rela_dyn.r_sym
            ));
        let app_rela_name = app_dynstr_table
            .get(app_sym.st_name as usize)
            .expect(&format!(
                "Failed to get symbol name for index: {}",
                app_sym.st_name
            ));

        match app_rela_dyn.r_type {
            // Adjust a link address (A) to its load address: `(B + A)`.
            R_RISCV_RELATIVE => {
                let relative_offset = app_start + app_rela_dyn.r_offset as usize;
                let new_value = app_start + app_rela_dyn.r_addend as usize;
                debug!(
                    "[App-rela.dyn R_RISCV_RELATIVE] @0x{:x}=0x{:x}",
                    relative_offset, new_value,
                );
                app_rela_dyn.r_addend.eq(&0).then(|| panic!("Bad st_value"));
                unsafe { *(relative_offset as *mut usize) = new_value };
            }
            // 64-bit relocation: `S + A`.
            R_RISCV_64 => {
                // Find symbol in LIB ELF
                let lib_sym = lib_dynsym_table
                    .iter()
                    .find(|s| {
                        let name = lib_dynstr_table.get(s.st_name as usize).unwrap_or(&"");
                        name == app_rela_name
                    })
                    .expect("Failed to find symbol in LIB dynamic symbol table");

                let relative_offset = app_start + app_rela_dyn.r_offset as usize;
                let new_value = lib_start + lib_sym.st_value as usize;
                debug!(
                    "[App-rela.dyn R_RISCV_64] @0x{:x}=0x{:x} name {}",
                    relative_offset, new_value, app_rela_name,
                );
                lib_sym.st_value.eq(&0).then(|| panic!("Bad lib st_value"));
                app_sym.st_value.ne(&0).then(|| panic!("Bad app st_value"));
                unsafe { *(relative_offset as *mut usize) = new_value };
            }
            _ => {
                panic!("Unknown relocation type: {}", app_rela_dyn.r_type);
            }
        }
    }
}
