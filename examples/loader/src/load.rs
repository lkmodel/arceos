use core::{
    cmp::min,
    slice::{from_raw_parts, from_raw_parts_mut},
};

use axlog::debug;

use crate::{abi::lookup_abi_call, elf::{verify_elf_header, LoadError}};
use elf::{
    abi::PT_LOAD,
    endian::LittleEndian,
    ElfBytes,
};

pub const PLASH_START: usize = 0xffff_ffc0_2200_0000;
pub const EXEC_ZONE_START: usize = 0xffff_ffc0_8010_0000;
const MAX_APP_SIZE: usize = 0x100000;

pub fn load_elf() -> u64 {
    debug!("Load payload ...");
    let elf_size = unsafe { *(PLASH_START as *const usize) };
    debug!("ELF size: 0x{:x}", elf_size);
    let elf_slice = unsafe { from_raw_parts((PLASH_START + 0x8) as *const u8, elf_size) };
    let elf: ElfBytes<'_, LittleEndian> =
        ElfBytes::<LittleEndian>::minimal_parse(elf_slice).expect("Failed to parse ELF");
    let elf_hdr = elf.ehdr;

    // 检查 ELF 头
    verify_elf_header(&elf).expect("Failed to verify ELF header");

    let is_need_interp = { 
        if let Some(segments) = elf.segments() {
            let mut is_pie = false;
            for segment in segments {
                debug!("Segment type: {}", segment.p_type);
                if segment.p_type == elf::abi::PT_INTERP {
                    is_pie = true;
                }
            }
            is_pie
        } else {
            false
        }
    };

    debug!("Dynamic interpreter (.interp section) exists: {}", is_need_interp);

    let run_code =
        unsafe { from_raw_parts_mut(EXEC_ZONE_START as *mut u8, MAX_APP_SIZE) };

    let entry: u64 = {
        if is_need_interp == false {
            // static and position independent executable
            let _ = load_exec(&elf, elf_slice, run_code);
            elf_hdr.e_entry
        } else {
            load_dyn(&elf, elf_slice, run_code);
            EXEC_ZONE_START as u64 + elf_hdr.e_entry
        }
    };
    debug!("Entry: 0x{:x}", entry);
    
    return entry;
}

fn load_exec(elf: &ElfBytes<LittleEndian>, elf_slice: &[u8], run_code: &mut [u8]) -> Result<(), LoadError> {
    if let Some(phs) = elf.segments() {
        for ph in phs {
            if ph.p_type != PT_LOAD {
                debug!("skipping segment type: {}", ph.p_type);
                continue;
            }
            
            let offset = ph.p_offset as usize;
            let filesz = ph.p_filesz as usize;
            let memsz = ph.p_memsz as usize;
            
            // 计算在内存中的实际地址
            let vaddr = ph.p_vaddr as usize;
            let dest_addr = vaddr - EXEC_ZONE_START;
            
            debug!("Loading segment: offset=0x{:x}, filesz=0x{:x}, memsz=0x{:x}, vaddr=0x{:x}", 
                offset, filesz, memsz, vaddr);

            debug!("dest_addr: {}", dest_addr);
            
            // 复制段内容
            if filesz > 0 {
                let src: &[u8] = &elf_slice[offset..offset + filesz];
                let dest = &mut run_code[dest_addr..dest_addr + filesz];
                dest.copy_from_slice(src);
            }
            
            // 处理 .bss 等需要零初始化的部分
            if memsz > filesz {
                let dest = &mut run_code[dest_addr + filesz..dest_addr + memsz];
                dest.fill(0);
            }
        }
    }

    let rela_shdr = elf
        .section_header_by_name(".rela.dyn")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.dyn section header");

    let relas = elf.section_data_as_relas(&rela_shdr)
        .expect("Failed to parse .rela.dyn section");

    for rela in relas {
        debug!("Rela offset: 0x{:x}, sym: 0x{:x}, type: 0x{:x}, addend: 0x{:x}", 
            rela.r_offset, 
            rela.r_sym, 
            rela.r_type, 
            rela.r_addend
        );

        match rela.r_type {
            // R_RISCV_RELATIVE
            3 => {
                let new_value = rela.r_addend as usize;
                unsafe {
                    *(rela.r_offset as *mut u64) = new_value as u64;
                }
            },
            _ => {
                debug!("Unsupported relocation type");
            }
        }
    }
    
    Ok(())
}

fn load_dyn(elf: &ElfBytes<LittleEndian>, elf_slice: &[u8], run_code: &mut [u8]) {
    let phdrs = elf.segments().expect("Failed to parse program headers");
    for phdr in phdrs {
        if phdr.p_type != elf::abi::PT_LOAD {
            continue;
        }
        debug!("Loading segment: offset=0x{:x}, filesz=0x{:x}, memsz=0x{:x}, vaddr=0x{:x}, paddr=0x{:x}", 
        phdr.p_offset, phdr.p_filesz, phdr.p_memsz, phdr.p_vaddr, phdr.p_paddr);
        load_segment(run_code, elf_slice, phdr.p_vaddr as usize, phdr.p_offset as usize, phdr.p_filesz as usize, phdr.p_memsz as usize);
    }

    if let Ok((shdrs_opt, strtab_opt)) = elf.section_headers_with_strtab() {
        let shdrs = shdrs_opt.expect("shdrs should be Some");
        let strtab = strtab_opt.expect("strtab should be Some");
        
        // 遍历所有节头，查找 .rela.dyn 和 .rela.plt
        for shdr in shdrs {
            // 获取节的名称
            if let Ok(name) = strtab.get(shdr.sh_name as usize) {
                match name {
                    // 找到 .rela.dyn 节
                    ".rela.dyn" => {
                        debug!("Found .rela.dyn section at offset 0x{:x}", shdr.sh_offset);
                        modify_rela_dyn(elf);
                    },
                    // 找到 .rela.plt 节
                    ".rela.plt" => {
                        debug!("Found .rela.plt section at offset 0x{:x}", shdr.sh_offset);
                        modify_rela_plt(elf);
                    },
                    // 其他节可以忽略
                    _ => continue,
                }
            }
        }
    }
}

fn load_segment(run_code: &mut [u8], elf_slice: &[u8], p_vaddr: usize, p_offset: usize, p_filesz: usize, p_memsz: usize) {
    // copy the segment into the executable zone
    // if memz is larger than filesz, zero out the rest
    let run_code_offset = p_vaddr;
    run_code[run_code_offset..run_code_offset + p_filesz]
        .copy_from_slice(&elf_slice[p_offset..p_offset + p_filesz]);
    if p_memsz > p_filesz {
        let zero_size = min(
            run_code.len() - p_filesz,
            p_memsz - p_filesz,
        );
        run_code[run_code_offset + p_filesz..run_code_offset + p_filesz + zero_size].fill(0);
    }
}

fn modify_rela_plt(elf: &ElfBytes<LittleEndian>) {
    let (dynsym_table, dynstr_table) = elf.dynamic_symbol_table()
        .expect("Failed to parse dynamic symbol table")
        .expect("ELF should have a dynamic symbol table");
    let rela_shdr = elf
        .section_header_by_name(".rela.plt")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.plt section header");

    let relas = elf.section_data_as_relas(&rela_shdr)
        .expect("Failed to parse .rela.plt section");

    for rela in relas {
        // get the r_sym'th symbol from the dynamic symbol table
        let sym = dynsym_table.get(rela.r_sym as usize).expect("Failed to get symbol");
        let rela_name = dynstr_table.get(sym.st_name as usize).expect("Failed to get symbol name");
        debug!("Rela sym: {}", rela_name);
        let func_addr = lookup_abi_call(rela_name).expect("Failed to find abi function");
        debug!("func_addr 0x{:x}", func_addr);
        unsafe {
            *((EXEC_ZONE_START as u64 + rela.r_offset) as *mut usize) = func_addr;
        }
    }
}

fn modify_rela_dyn(elf: &ElfBytes<LittleEndian>) {
    let rela_shdr = elf
        .section_header_by_name(".rela.dyn")
        .expect("section table should be parseable")
        .expect("elf should have a .rela.dyn section header");

    let relas = elf.section_data_as_relas(&rela_shdr)
        .expect("Failed to parse .rela.dyn section");

    for rela in relas {
        debug!("Rela offset: 0x{:x}, sym: 0x{:x}, type: 0x{:x}, addend: 0x{:x}", 
            rela.r_offset, 
            rela.r_sym, 
            rela.r_type, 
            rela.r_addend
        );

        match rela.r_type {
            // R_RISCV_RELATIVE
            3 => {
                let reloc_addr = EXEC_ZONE_START + rela.r_offset as usize;
                let new_value = EXEC_ZONE_START + rela.r_addend as usize;
                unsafe {
                    *(reloc_addr as *mut u64) = new_value as u64;
                }
            },
            _ => {
                debug!("Unsupported relocation type");
            }
        }
    }
}
