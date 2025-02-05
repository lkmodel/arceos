use axhal::{arch::read_page_table_root, mem::PhysAddr};
use axlog::trace;
use riscv::asm;
use riscv::register::satp;

/// Writes the register to update the current page table root.
///
/// # Safety
///
/// This function is unsafe as it changes the virtual memory address space.
pub unsafe fn write_page_table_root(root_paddr: PhysAddr) {
    let old_root = read_page_table_root();
    trace!("set page table root: {:#x} => {:#x}", old_root, root_paddr);
    if old_root != root_paddr {
        unsafe { satp::set(satp::Mode::Sv39, 0, root_paddr.as_usize() >> 12) };
        asm::sfence_vma_all();
    }
}

pub use self::write_page_table_root as write_page_table_root0;