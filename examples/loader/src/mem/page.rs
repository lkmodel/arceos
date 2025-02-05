use alloc::vec::Vec;
use axhal::mem::{VirtAddr, PAGE_SIZE_4K};
use axalloc::global_allocator;

/// A safe wrapper of a single 4K page.
/// It holds the page's VirtAddr (PhysAddr + offset)
#[derive(Debug)]
pub struct PhysPage {
    /// The start virtual address of this page.
    pub start_vaddr: VirtAddr,
}

impl PhysPage {
    /// Allocate one 4K-sized page.
    pub fn alloc() -> Result<Self, &'static str> {
        global_allocator()
            .alloc_pages(1, PAGE_SIZE_4K)
            .map(|vaddr| Self {
                start_vaddr: vaddr.into(),
            })
            .map_err(|_| "allocate page failed")
    }

    /// Allocate some 4K-sized pages and fill with zero.
    pub fn alloc_contiguous(
        num_pages: usize,
        align_pow2: usize,
        data: Option<&[u8]>,
    ) -> Result<Vec<Option<Self>>, &'static str> {
        global_allocator()
            .alloc_pages(num_pages, align_pow2)
            .map(|vaddr| {
                let pages = unsafe {
                    core::slice::from_raw_parts_mut(vaddr as *mut u8, num_pages * PAGE_SIZE_4K)
                };
                pages.fill(0);
                if let Some(data) = data {
                    pages[..data.len()].copy_from_slice(data);
                }

                (0..num_pages)
                    .map(|page_idx| {
                        Some(PhysPage {
                            start_vaddr: (vaddr + page_idx * PAGE_SIZE_4K).into(),
                        })
                    })
                    .collect()
            })
            .map_err(|_| "allocate page failed")
    }

    /// Convert to a raw pointer.
    pub fn as_ptr(&self) -> *const u8 {
        self.start_vaddr.as_ptr()
    }

    /// Convert to a mutable raw pointer.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.start_vaddr.as_mut_ptr()
    }

    /// Forms a slice that can read data.
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), PAGE_SIZE_4K) }
    }

    /// Forms a mutable slice that can write data.
    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.as_mut_ptr(), PAGE_SIZE_4K) }
    }

    /// Fill `self` with `byte`.
    pub fn fill(&mut self, byte: u8) {
        unsafe { core::ptr::write_bytes(self.as_mut_ptr(), byte, PAGE_SIZE_4K) }
    }
}

impl Drop for PhysPage {
    fn drop(&mut self) {
        global_allocator().dealloc_pages(self.start_vaddr.into(), 1);
    }
}
