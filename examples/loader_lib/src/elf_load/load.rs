use alloc::{
    collections::btree_map::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

use core::{
    cmp::min,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use axerrno::AxResult;
use axhal::paging::MappingFlags;
use axlog::{debug, info, warn};
use axmm::AddrSpace;
use axstd::format;
use elf::{
    ElfBytes,
    abi::{ET_EXEC, PF_R, PF_W, PF_X, PT_LOAD, R_RISCV_64, R_RISCV_JUMP_SLOT, R_RISCV_RELATIVE},
    endian::LittleEndian,
    symbol::Symbol,
};
use kernel_elf_parser::get_app_stack_region;
use memory_addr::{MemoryAddr, VirtAddr};

use crate::{
    config::{APP_START, LIB_START, MAX_APP_SIZE, MAX_LIB_SIZE},
    elf_load::{auxv::get_auxv_vector, load_old, verify::verify_elf_header},
};

/// The segment of the elf file, which is used to map the elf file to the memory space
pub struct ELFSegment {
    /// The start virtual address of the segment
    pub start_vaddr: VirtAddr,
    /// The size of the segment
    pub size: usize,
    /// The flags of the segment which is used to set the page table entry
    pub flags: MappingFlags,
    /// The data of the segment
    pub data: Vec<u8>,
    /// The offset of the segment relative to the start of the page
    pub offset: usize,
}

/// The information of a given ELF file
pub struct ELFInfo {
    /// The entry point of the ELF file
    pub entry: VirtAddr,
    /// The segments of the ELF file
    pub segments: Vec<ELFSegment>,
    /// The auxiliary vectors of the ELF file
    pub auxv: BTreeMap<u8, usize>,
}

/// Load a user app.
///
/// # Returns
/// - The first return value is the entry point of the user app.
/// - The second return value is the top of the user stack.
/// - The third return value is the address space of the user app.
pub fn load_user_app(
    memory_set: &mut AddrSpace,
    app_name: &str,
    app_elf_file: &'static [u8],
    lib_elf_file: Option<&'static [u8]>,
) -> AxResult<(VirtAddr, VirtAddr)> {
    match lib_elf_file {
        Some(lib_elf_file) => {
            info!("Load lib");
            let main_entry = get_func_sym(app_elf_file, "main")
                .expect("Failed to find symbol in APP dynamic symol table")
                .st_value as usize;
            let lib_info = load_lib(
                VirtAddr::from(LIB_START),
                lib_elf_file,
                APP_START
                    + main_entry
                        .ne(&0)
                        .then(|| main_entry)
                        .expect("Bad st_value(main)"),
            );
            for segment in lib_info.segments {
                debug!(
                    "Mapping LIB ELF segment: [{:#x?}, {:#x?}) flags: {:#x?}",
                    segment.start_vaddr,
                    segment.start_vaddr + segment.size,
                    segment.flags
                );
                memory_set.map_alloc(segment.start_vaddr, segment.size, segment.flags, true)?;

                if segment.data.is_empty() {
                    continue;
                }

                memory_set.write(segment.start_vaddr + segment.offset, &segment.data)?;
            }
            info!("Mapping user lib stack {:?}", memory_set);

            // The user stack is divided into two parts:
            // `ustack_start` -> `ustack_pointer`: It is the stack space that users actually read and write.
            // `ustack_pointer` -> `ustack_end`: It is the space that contains the arguments, environment variables and auxv passed to the app.
            //  When the app starts running, the stack pointer points to `ustack_pointer`.

            let lib_ustack_end = VirtAddr::from_usize(LIB_START);
            let lib_ustack_size = MAX_LIB_SIZE;
            let lib_ustack_start = lib_ustack_end - lib_ustack_size;
            debug!(
                "Mapping user lib stack: {:#x?} -> {:#x?}",
                lib_ustack_start, lib_ustack_end
            );

            // `user-heap-base` = `"0x3FA0_0000"`
            // # The base address of the user stack. And the stack bottom is `user-stack-top + max-user-stack-size`.
            // `user-stack-top` = `"0x3FE0_0000"`
            // # The size of the user heap.
            // `max-user-heap-size` = `"0x40_0000"`

            // FIX: Add more arguments and environment variables
            let (lib_stack_data, lib_ustack_pointer) =
                get_app_stack_region(&[], &[], &lib_info.auxv, lib_ustack_start, lib_ustack_size);

            info!("Mapping user lib stack data: {:#x?}", lib_ustack_pointer);

            memory_set.map_alloc(
                lib_ustack_start,
                lib_ustack_size,
                MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
                true,
            )?;

            info!("Writing user lib stack data");

            memory_set.write(
                VirtAddr::from_usize(lib_ustack_pointer),
                lib_stack_data.as_slice(),
            )?;
            // ``` TODO:
            // Ok((lib_info.entry, VirtAddr::from(lib_ustack_pointer)))

            info!("Load app");
            let app_info = load_app(
                VirtAddr::from(APP_START),
                app_elf_file,
                VirtAddr::from(LIB_START),
                lib_elf_file,
            );

            for segment in app_info.segments {
                debug!(
                    "Mapping APP ELF segment: [{:#x?}, {:#x?}) flags: {:#x?}",
                    segment.start_vaddr,
                    segment.start_vaddr + segment.size,
                    segment.flags
                );
                // NOTE: 将之前的那个map先取消
                memory_set.unmap(segment.start_vaddr, segment.size)?;
                memory_set.map_alloc(segment.start_vaddr, segment.size, segment.flags, true)?;

                if segment.data.is_empty() {
                    continue;
                }

                memory_set.write(segment.start_vaddr + segment.offset, &segment.data)?;
            }
            info!("Mapping user app stack {:?}", memory_set);

            let app_ustack_end = VirtAddr::from_usize(APP_START);
            let app_ustack_size = MAX_APP_SIZE;
            let app_ustack_start = app_ustack_end - app_ustack_size;
            debug!(
                "Mapping user app stack: {:#x?} -> {:#x?}",
                app_ustack_start, app_ustack_end
            );

            // FIX: Add more arguments and environment variables
            let (app_stack_data, app_ustack_pointer) = get_app_stack_region(
                &[app_name.to_string()],
                &[],
                &app_info.auxv,
                app_ustack_start,
                app_ustack_size,
            );

            info!("Mapping user app stack data: {:#x?}", app_ustack_pointer);

            // NOTE: 将之前的那个map先取消
            memory_set.unmap(app_ustack_start, app_ustack_size)?;
            memory_set.map_alloc(
                app_ustack_start,
                app_ustack_size,
                MappingFlags::READ | MappingFlags::WRITE | MappingFlags::EXECUTE,
                true,
            )?;

            info!("Writing user app stack data");

            memory_set.write(
                VirtAddr::from_usize(app_ustack_pointer),
                app_stack_data.as_slice(),
            )?;

            Ok((lib_info.entry, VirtAddr::from(app_ustack_pointer)))
        }
        None => {
            todo!();
        }
    }
}

pub fn load_lib(base_addr: VirtAddr, elf_slice: &'static [u8], main_entry: usize) -> ELFInfo {
    debug!("Load lib ...");
    let elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(elf_slice).expect("Failed to parse ELF");
    verify_elf_header(&elf).expect("Failed to verify ELF header");
    let elf_offset = get_elf_base_addr(&elf, base_addr.as_usize()).unwrap();

    let mut segments = Vec::new();
    let phdrs = elf.segments().expect("Failed to parse program headers");
    for phdr in phdrs {
        if phdr.p_type != PT_LOAD {
            continue;
        }

        let st_vaddr = VirtAddr::from(phdr.p_vaddr as usize) + elf_offset;
        let st_vaddr_align = st_vaddr.align_down_4k();
        let ed_vaddr_align =
            VirtAddr::from((phdr.p_vaddr + phdr.p_memsz) as usize).align_up_4k() + elf_offset;

        let mut segment_data = elf
            .segment_data(&phdr)
            .expect("Should be able to get section data")
            .to_vec();

        debug!(
            "Load Segment vaddr 0x{:x} offset 0x{:x} filesz 0x{:x} memsz 0x{:x}",
            phdr.p_vaddr as usize,
            phdr.p_offset as usize,
            phdr.p_filesz as usize,
            phdr.p_memsz as u64,
        );

        modify_lib_segment(
            &elf,
            &mut segment_data,
            elf_offset,
            phdr.p_vaddr as usize,
            main_entry,
        );
        segments.push(ELFSegment {
            start_vaddr: st_vaddr_align,
            size: ed_vaddr_align.as_usize() - st_vaddr_align.as_usize(),
            flags: into_mapflag(phdr.p_flags),
            data: segment_data,
            offset: st_vaddr.align_offset_4k(),
        });
    }

    debug!("Load lib done");
    ELFInfo {
        entry: VirtAddr::from(elf.ehdr.e_entry as usize + elf_offset),
        segments,
        auxv: get_auxv_vector(&elf, elf_offset),
    }
}

pub fn load_app(
    base_addr: VirtAddr,
    elf_slice: &'static [u8],
    lib_base_addr: VirtAddr,
    lib_elf_slice: &'static [u8],
) -> ELFInfo {
    debug!("Load app ...");
    let elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(elf_slice).expect("Failed to parse ELF");
    verify_elf_header(&elf).expect("Failed to verify ELF header");
    let elf_offset = get_elf_base_addr(&elf, base_addr.as_usize()).unwrap();

    let lib_elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(lib_elf_slice).expect("Failed to parse ELF");
    verify_elf_header(&lib_elf).expect("Failed to verify ELF header");
    let lib_elf_offset = get_elf_base_addr(&elf, lib_base_addr.as_usize()).unwrap();

    let mut segments = Vec::new();
    let phdrs = elf.segments().expect("Failed to parse program headers");
    for phdr in phdrs {
        if phdr.p_type != PT_LOAD {
            continue;
        }

        let st_vaddr = VirtAddr::from(phdr.p_vaddr as usize) + elf_offset;
        let st_vaddr_align = st_vaddr.align_down_4k();
        let ed_vaddr_align =
            VirtAddr::from((phdr.p_vaddr + phdr.p_memsz) as usize).align_up_4k() + elf_offset;

        let mut segment_data = elf
            .segment_data(&phdr)
            .expect("Should be able to get section data")
            .to_vec();

        debug!(
            "Load Segment vaddr 0x{:x} offset 0x{:x} filesz 0x{:x} memsz 0x{:x}",
            phdr.p_vaddr as usize,
            phdr.p_offset as usize,
            phdr.p_filesz as usize,
            phdr.p_memsz as u64,
        );

        modify_app_segment(
            &elf,
            &mut segment_data,
            elf_offset,
            phdr.p_vaddr as usize,
            &lib_elf,
            lib_elf_offset,
        );
        segments.push(ELFSegment {
            start_vaddr: st_vaddr_align,
            size: ed_vaddr_align.as_usize() - st_vaddr_align.as_usize(),
            flags: into_mapflag(phdr.p_flags),
            data: segment_data,
            offset: st_vaddr.align_offset_4k(),
        });
    }

    debug!("Load app done");

    ELFInfo {
        entry: VirtAddr::from(elf.ehdr.e_entry as usize + elf_offset),
        segments,
        auxv: get_auxv_vector(&elf, elf_offset),
    }
}

/// Process the relocation segment of lib ELF file.
///
/// # Arguments
/// * `elf` - The ELF file of lib
/// * `segment_data` - The memory that should be loaded
/// * `elf_offset` The offset lib should be loaded
/// * `segment_vaddr` -
/// * `main_entry` - Initialize app main function entry
fn modify_lib_segment(
    elf: &ElfBytes<LittleEndian>,
    segment_data: &mut [u8],
    elf_offset: usize,
    segment_vaddr: usize,
    main_entry: usize,
) {
    let (dynsym_table, dynstr_table) = elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");

    debug!("modify for LIB-RELA.PLT so lib");
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

        let offset = rela_plt.r_offset as usize;
        // 检查重定位是否在当前段内
        if offset >= segment_vaddr && offset < segment_vaddr + segment_data.len() {
            let relative_offset = offset - segment_vaddr;
            match rela_plt.r_type {
                // Indicates the symbol associated with a `PLT` entry: `S`
                R_RISCV_JUMP_SLOT => {
                    if rela_name == run_code_entry_name {
                        // Set main entry in APP
                        let new_value = main_entry as u64;
                        segment_data[relative_offset..relative_offset + 8]
                            .copy_from_slice(&new_value.to_ne_bytes());

                        debug!(
                            "[Lib-plt ENTRY] @0x{:x} value 0x{:x} st_name {}",
                            relative_offset, new_value, rela_name,
                        );
                    } else {
                        let new_value = (elf_offset + sym.st_value as usize) as u64;
                        segment_data[relative_offset..relative_offset + 8]
                            .copy_from_slice(&new_value.to_ne_bytes());
                        debug!(
                            "[Lib-rela.plt R_RISCV_JUMP_SLOT] @0x{:x}=0x{:x} st_name {}",
                            relative_offset, new_value, rela_name,
                        );
                        sym.st_value.eq(&0).then(|| panic!("Bad st_value"));
                    }
                }
                _ => {
                    panic!("Unknown relocation type: {}", rela_plt.r_type);
                }
            }
        }
    }

    debug!("modify for LIB-RELA.DYN so lib");
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

        let offset = rela_dyn.r_offset as usize;
        // 检查重定位是否在当前段内
        if offset >= segment_vaddr && offset < segment_vaddr + segment_data.len() {
            let relative_offset = offset - segment_vaddr;
            match rela_dyn.r_type {
                // Adjust a link address (A) to its load address: `(B + A)`.
                R_RISCV_RELATIVE => {
                    let new_value = (elf_offset + rela_dyn.r_addend as usize) as u64;
                    segment_data[relative_offset..relative_offset + 8]
                        .copy_from_slice(&new_value.to_ne_bytes());
                    debug!(
                        "[Lib-rela.dyn R_RISCV_RELATIVE] @0x{:x}=0x{:x}",
                        relative_offset, new_value,
                    );
                    rela_dyn.r_addend.eq(&0).then(|| panic!("Bad st_value"));
                }
                // 64-bit relocation: `S + A`.
                R_RISCV_64 => {
                    let new_value = (elf_offset + sym.st_value as usize) as u64;
                    segment_data[relative_offset..relative_offset + 8]
                        .copy_from_slice(&new_value.to_ne_bytes());
                    debug!(
                        "[Lib-rela.dyn R_RISCV_64] @0x{:x}=0x{:x} name {}",
                        relative_offset, new_value, rela_name,
                    );
                    sym.st_value.eq(&0).then(|| panic!("Bad st_value"));
                }
                _ => {
                    panic!("Unknown relocation type: {}", rela_dyn.r_type);
                }
            }
        }
    }
}

fn modify_app_segment(
    app_elf: &ElfBytes<LittleEndian>,
    segment_data: &mut [u8],
    app_elf_offset: usize,
    segment_vaddr: usize,
    lib_elf: &ElfBytes<LittleEndian>,
    lib_elf_offset: usize,
) {
    let (app_dynsym_table, app_dynstr_table) = app_elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");
    let (lib_dynsym_table, lib_dynstr_table) = lib_elf
        .dynamic_symbol_table()
        .expect("Failed to parse LIB dynamic symbol table")
        .expect("LIB ELF should have a dynamic symbol table");

    debug!("modify for APP-RELA.PLT run code");
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

        let offset = app_rela_plt.r_offset as usize;
        // 检查重定位是否在当前段内
        if offset >= segment_vaddr && offset < segment_vaddr + segment_data.len() {
            let relative_offset = offset - segment_vaddr;
            match app_rela_plt.r_type {
                // Indicates the symbol associated with a `PLT` entry: `S`
                R_RISCV_JUMP_SLOT => {
                    // Find symbol in LIB ELF
                    let lib_sym = lib_dynsym_table
                        .iter()
                        .find(|s| {
                            let name = lib_dynstr_table.get(s.st_name as usize).unwrap_or(&"");
                            name == app_rela_name
                        })
                        .expect("Failed to find symbol in LIB dynamic symbol table");

                    let new_value = (lib_elf_offset + lib_sym.st_value as usize) as u64;
                    segment_data[relative_offset..relative_offset + 8]
                        .copy_from_slice(&new_value.to_ne_bytes());

                    debug!(
                        "[App-rela.plt R_RISCV_JUMP_SLOT] @0x{:x}=0x{:x} st_name {}",
                        relative_offset, new_value, app_rela_name,
                    );
                    lib_sym.st_value.eq(&0).then(|| panic!("Bad st_value"));
                }
                _ => {
                    panic!("Unknown relocation type: {}", app_rela_plt.r_type);
                }
            }
        }
    }

    debug!("modify for APP-RELA.DYN run code");
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

        let offset = app_rela_dyn.r_offset as usize;
        // 检查重定位是否在当前段内
        if offset >= segment_vaddr && offset < segment_vaddr + segment_data.len() {
            let relative_offset = offset - segment_vaddr;
            match app_rela_dyn.r_type {
                // Adjust a link address (A) to its load address: `(B + A)`.
                R_RISCV_RELATIVE => {
                    let new_value = (app_elf_offset + app_rela_dyn.r_addend as usize) as u64;
                    segment_data[relative_offset..relative_offset + 8]
                        .copy_from_slice(&new_value.to_ne_bytes());
                    debug!(
                        "[App-rela.dyn R_RISCV_RELATIVE] @0x{:x}=0x{:x}",
                        relative_offset, new_value,
                    );
                    app_rela_dyn.r_addend.eq(&0).then(|| panic!("Bad st_value"));
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

                    let new_value = (app_elf_offset + lib_sym.st_value as usize) as u64;
                    segment_data[relative_offset..relative_offset + 8]
                        .copy_from_slice(&new_value.to_ne_bytes());

                    debug!(
                        "[App-rela.dyn R_RISCV_64] @0x{:x}=0x{:x} name {}",
                        relative_offset, new_value, app_rela_name,
                    );
                    lib_sym.st_value.eq(&0).then(|| panic!("Bad lib st_value"));
                    app_sym.st_value.ne(&0).then(|| panic!("Bad app st_value"));
                }
                _ => {
                    panic!("Unknown relocation type: {}", app_rela_dyn.r_type);
                }
            }
        }
    }
}

/// Calculate the base address of the ELF file loaded into the memory.
///
/// - When the ELF file is a position-independent executable,
/// the base address will be decided by the kernel.
///
/// - Otherwise, the base address is determined by the file, and this field `given_base` will be ignored.
///
/// # Arguments
///
/// * `elf` - The ELF file
///
/// * `given_base` - The base address of the ELF file given by the kernel
///
/// # Return
///
/// The real base address for ELF file loaded into the memory.
pub fn get_elf_base_addr(elf: &ElfBytes<LittleEndian>, given_base: usize) -> Result<usize, String> {
    // Some elf will load ELF Header (offset == 0) to `vaddr` 0. In that case, base_addr will be added to all the LOAD.
    if elf.ehdr.e_type == ET_EXEC {
        if let Some(phdr) = elf
            .segments()
            .expect("Failed to parse ELF")
            .iter()
            .find(|phdr| phdr.p_type == PT_LOAD)
        {
            if phdr.p_vaddr == 0 {
                Err(
                    "The ELF file is an executable, but some segements may be loaded to vaddr 0"
                        .to_string(),
                )
            } else {
                Ok(0)
            }
        } else {
            Err("The ELF file is an executable, but no LOAD segment found".to_string())
        }
    } else {
        Ok(given_base)
    }
}

/// 封装了获取ELF Symbol的函数
///
/// # Arguments
/// * `elf_file` - ELF文件
/// * `func_name` - 需要查询的函数名
///
/// # Return
/// 查询结果被包装到了Option之中
pub fn get_func_sym(elf_file: &'static [u8], func_name: &str) -> Option<Symbol> {
    let app_elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(elf_file).expect("Failed to parse ELF");
    let (app_dynsym_table, app_dynstr_table) = app_elf
        .dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");
    app_dynsym_table.iter().find(|s| {
        let name = app_dynstr_table.get(s.st_name as usize).unwrap_or(&"");
        name == func_name
    })
}

/// 将flags转换为MappingFlags
pub fn into_mapflag(f: u32) -> MappingFlags {
    let mut ret = MappingFlags::WRITE;
    if f == PF_R {
        ret |= MappingFlags::READ;
    }
    // FIX: 这里可能需要注释
    if f == PF_W {
        ret |= MappingFlags::WRITE;
    }
    if f == PF_X {
        ret |= MappingFlags::EXECUTE;
    }
    ret
}

/// 用于给`Unikernel`去加载段
/// Copy the segment into the executable zone
/// TODO:
/// If `memsz` is larger than `filesz`, zero out the rest
fn load_segment(
    run_code: &mut [u8],
    data: &Vec<u8>,
    start_addr: usize,
    p_offset: usize,
    size: usize,
    address_bios: usize,
) {
    let start_code_offset = start_addr - address_bios;
    debug!(
        "run_code size 0x{:x} data size 0x{:x} Addr[0x{:x}, 0x{:x}) = Data[0x{:x}, 0x{:x})",
        run_code.len(),
        data.len(),
        start_code_offset,
        start_code_offset + size,
        p_offset,
        p_offset + size
    );
    // run_code.copy_from_slice(&data);
    run_code[start_code_offset..start_code_offset + data.len()]
        .copy_from_slice(&data[0..data.len()]);
}
