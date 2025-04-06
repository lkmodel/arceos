//! Some constant in the elf file
extern crate alloc;
use alloc::collections::BTreeMap;
use elf::{ElfBytes, abi::PT_LOAD, endian::LittleEndian};
use memory_addr::PAGE_SIZE_4K;

use crate::elf_load::load::get_elf_base_addr;

const AT_PHDR: u8 = 3;
const AT_PHENT: u8 = 4;
const AT_PHNUM: u8 = 5;
const AT_PAGESZ: u8 = 6;
#[allow(unused)]
const AT_BASE: u8 = 7;
#[allow(unused)]
const AT_ENTRY: u8 = 9;
const AT_RANDOM: u8 = 25;

/// Read auxiliary vectors from the ELF file.
///
/// # Arguments
///
/// * `elf` - The elf file
/// * `elf_base_addr` - The base address of the elf file if the file will be loaded to the memory
///
/// # Return
/// It will return a `BTreeMap<u8, usize>` which contains the auxiliary vectors. The key is the entry type, and the value is the value of the auxiliary vector.
///
/// Details about auxiliary vectors are described in <https://articles.manugarg.com/aboutelfauxiliaryvectors.html>
pub fn get_auxv_vector(elf: &ElfBytes<LittleEndian>, elf_base_addr: usize) -> BTreeMap<u8, usize> {
    // Some elf will load ELF Header (offset == 0) to `vaddr` 0. In that case, `base_addr` will be added to all the LOAD.
    let kernel_offset = get_elf_base_addr(elf, elf_base_addr).unwrap();
    let mut map = BTreeMap::new();

    if let Some(phdr) = elf
        .segments()
        .expect("Failed to parse program headers")
        .iter()
        .find(|phdr| phdr.p_type == PT_LOAD)
    {
        map.insert(
            AT_PHDR,
            kernel_offset + (phdr.p_vaddr + elf.ehdr.e_phoff) as usize,
        );
    } else {
        map.insert(AT_PHDR, 0);
    }

    map.insert(AT_PHENT, elf.ehdr.e_phentsize as usize);
    map.insert(AT_PHNUM, elf.ehdr.e_phnum as usize);
    map.insert(AT_RANDOM, 0);
    map.insert(AT_PAGESZ, PAGE_SIZE_4K);
    map
}
