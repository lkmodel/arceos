//! Loader for loading apps.
//!
//! It will read and parse ELF files.
//!
//! Now these apps are loaded into memory as a part of the kernel image.
use alloc::{collections::btree_map::BTreeMap, string::{String, ToString}, vec::Vec};

use axhal::paging::MappingFlags;
use axlog::{debug, info, trace};
use axstd::println;
use memory_addr::{MemoryAddr, VirtAddr};
use xmas_elf::{sections::SectionData, symbol_table::Entry, ElfFile};

use crate::{abi::lookup_abi_call, elf::auxv::get_auxv_vector};

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

/// Load the ELF files by the given app name and return
/// the segments of the ELF file
///
/// # Arguments
/// * `name` - The name of the app
/// * `base_addr` - The minimal address of user space
///
/// # Returns
/// Entry and information about segments of the given ELF file
pub(crate) fn load_elf(base_addr: VirtAddr, elf_slice: &'static [u8]) -> ELFInfo {
    use xmas_elf::program::{Flags, SegmentData};
    use xmas_elf::{header, ElfFile};

    let elf = ElfFile::new(&elf_slice).expect("Failed to parse ELF");

    let elf_header = elf.header;
    assert_eq!(elf_header.pt1.magic, *b"\x7fELF", "invalid elf!");

    let expect_arch = if cfg!(target_arch = "x86_64") {
        header::Machine::X86_64
    } else if cfg!(target_arch = "aarch64") {
        header::Machine::AArch64
    } else if cfg!(target_arch = "riscv64") {
        header::Machine::RISC_V
    } else {
        panic!("Unsupported architecture!");
    };

    assert_eq!(
        elf.header.pt2.machine().as_machine(),
        expect_arch,
        "invalid ELF arch"
    );

    fn into_mapflag(f: Flags) -> MappingFlags {
        let mut ret = MappingFlags::WRITE;
        if f.is_read() {
            ret |= MappingFlags::READ;
        }
        // if f.is_write() {
        //     ret |= MappingFlags::WRITE;
        // }
        if f.is_execute() {
            ret |= MappingFlags::EXECUTE;
        }
        ret
    }

    let mut segments = Vec::new();
    let elf_offset = get_elf_base_addr(&elf, base_addr.as_usize()).unwrap();

    // 加载所有LOAD段
    for ph in elf.program_iter()
        .filter(|ph| ph.get_type() == Ok(xmas_elf::program::Type::Load)) {
        
        let st_vaddr = VirtAddr::from(ph.virtual_addr() as usize) + elf_offset;
        let st_vaddr_align = st_vaddr.align_down_4k();
        let ed_vaddr_align = VirtAddr::from((ph.virtual_addr() + ph.mem_size()) as usize)
            .align_up_4k() + elf_offset;

        let mut segment_data = match ph.get_data(&elf).unwrap() {
            SegmentData::Undefined(data) => data.to_vec(),
            _ => panic!("failed to get ELF segment data"),
        };

        // 处理段内的重定位信息
        process_relocations(&elf, &mut segment_data, elf_offset, ph.virtual_addr() as usize);

        segments.push(ELFSegment {
            start_vaddr: st_vaddr_align,
            size: ed_vaddr_align.as_usize() - st_vaddr_align.as_usize(),
            flags: into_mapflag(ph.flags()),
            data: segment_data,
            offset: st_vaddr.align_offset_4k(),
        });
    }

    ELFInfo {
        entry: VirtAddr::from(elf.header.pt2.entry_point() as usize + elf_offset),
        segments,
        auxv: get_auxv_vector(&elf, elf_offset),
    }
}

fn process_relocations(elf: &ElfFile, segment_data: &mut [u8], elf_offset: usize, segment_vaddr: usize) {
    // 处理 .rela.dyn
    if let Some(rela_dyn) = elf.find_section_by_name(".rela.dyn") {
        if let Ok(SectionData::Rela64(rela_data)) = rela_dyn.get_data(elf) {
            for rela in rela_data {
                let offset = rela.get_offset() as usize;
                // 检查重定位是否在当前段内
                if offset >= segment_vaddr && offset < segment_vaddr + segment_data.len() {
                    let relative_offset = offset - segment_vaddr;
                    
                    match rela.get_type() {
                        3 => { // R_RISCV_RELATIVE
                            let new_value = (elf_offset + rela.get_addend() as usize) as u64;
                            segment_data[relative_offset..relative_offset + 8]
                                .copy_from_slice(&new_value.to_ne_bytes());
                        },
                        _ => debug!("Unsupported relocation type: {}", rela.get_type()),
                    }
                }
            }
        }
    }
    let mut unimpl = Vec::new();
    // 处理 .rela.plt
    if let Some(rela_plt) = elf.find_section_by_name(".rela.plt") {
        if let Ok(SectionData::Rela64(rela_data)) = rela_plt.get_data(elf) {
            if let Some(dynsym) = elf.find_section_by_name(".dynsym") {
                if let Ok(SectionData::DynSymbolTable64(dynsym_data)) = dynsym.get_data(elf) {
                    for rela in rela_data {
                        let offset = rela.get_offset() as usize;
                        // 检查重定位是否在当前段内
                        if offset >= segment_vaddr && offset < segment_vaddr + segment_data.len() {
                            let sym = &dynsym_data[rela.get_symbol_table_index() as usize];
                            if let Ok(name) = sym.get_name(elf) {
                                trace!("Relocation: {}", name);
                                if let Some(func_addr) = lookup_abi_call(name) {
                                    println!("Found function: 0x{:x}", func_addr);
                                    let relative_offset = offset - segment_vaddr;
                                    // 在段数据中修改重定位位置
                                    
                                    segment_data[relative_offset..relative_offset + 8]
                                        .copy_from_slice(&(func_addr as u64).to_ne_bytes());
                                }else {
                                    unimpl.push(name);
                                }
                            }
                        }
                    }
                    info!("Not be implemented functions:{}",unimpl.join(","));
                }
            }
        }
    }
}

// fn analyze_load_segments_sections(elf: &ElfFile) {
//     trace!("Analyzing LOAD segments and their sections:");
    
//     // 遍历所有LOAD段
//     let load_segments: Vec<_> = elf.program_iter()
//         .filter(|ph| ph.get_type() == Ok(program::Type::Load))
//         .collect();
        
//     for (i, load_seg) in load_segments.iter().enumerate() {
//         trace!("\nLOAD Segment #{}", i);
//         trace!("Virtual Address Range: {:#x} - {:#x}", 
//             load_seg.virtual_addr(), 
//             load_seg.virtual_addr() + load_seg.mem_size());
//             trace!("Contained sections:");
        
//         // 遍历所有节，检查哪些节在这个LOAD段内
//         for section in elf.section_iter() {
//             let sect_start = section.address();
//             let sect_end = sect_start + section.size();
//             let seg_start = load_seg.virtual_addr();
//             let seg_end = seg_start + load_seg.mem_size();
            
//             // 检查节是否在LOAD段的地址范围内
//             if sect_start >= seg_start && sect_end <= seg_end {
//                 trace!("  - {} (addr: {:#x}, size: {:#x}, type: {:?})", 
//                     section.get_name(elf).unwrap_or("unnamed"),
//                     section.address(),
//                     section.size(),
//                     section.get_type());
//             }
//         }
        
//         // 显示段的权限
//         let flags = load_seg.flags();
//         trace!("Segment flags: R:{} W:{} X:{}", 
//             flags.is_read(), 
//             flags.is_write(), 
//             flags.is_execute());
//     }
// }

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
pub fn get_elf_base_addr(elf: &xmas_elf::ElfFile, given_base: usize) -> Result<usize, String> {
    // Some elf will load ELF Header (offset == 0) to vaddr 0. In that case, base_addr will be added to all the LOAD.
    if elf.header.pt2.type_().as_type() == xmas_elf::header::Type::Executable {
        if let Some(ph) = elf
            .program_iter()
            .find(|ph| ph.get_type() == Ok(xmas_elf::program::Type::Load))
        {
            // The LOAD segements are sorted by the virtual address, so the first one is the lowest one.
            if ph.virtual_addr() == 0 {
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
