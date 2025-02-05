use alloc::{sync::Arc, vec::Vec};
use axerrno::AxResult;
use axhal::{
    mem::{virt_to_phys, MemoryAddr, PhysAddr, VirtAddr, PAGE_SIZE_4K}, 
    paging::{MappingFlags, PageSize, PageTable}
};
use axlog::{debug, error, trace};
use axsync::Mutex;
use core::ptr::copy_nonoverlapping;

use super::page::PhysPage;

/// A continuous virtual area in user memory.
///
/// NOTE: Cloning a `MapArea` needs allocating new phys pages and modifying a page table. So
/// `Clone` trait won't implemented.
pub struct MapArea {
    /// phys pages of this area
    pub pages: Vec<Option<Arc<Mutex<PhysPage>>>>,
    /// start virtual address
    pub vaddr: VirtAddr,
    /// shared in child process
    shared: bool,
    /// mapping flags of this area
    pub flags: MappingFlags,
}

impl MapArea {
    /// Create a lazy-load area and map it in page table (page fault PTE).
    pub fn new_lazy(
        start: VirtAddr,
        num_pages: usize,
        flags: MappingFlags,
        page_table: &mut PageTable,
    ) -> Self {
        // 1. 创建空页向量
        let mut pages = Vec::with_capacity(num_pages);
        for _ in 0..num_pages {
            pages.push(None);
        }

        // 2. 使用 map_region 方法映射区域
        // 注意：此时使用 PhysAddr::from(0) 作为物理地址
        // 当访问这些页面时会触发缺页中断，因为物理地址为 0
        page_table
            .map_region(
                start,
                |_vaddr| PhysAddr::from(0),
                num_pages * PAGE_SIZE_4K,
                flags,
                false,
                true,
            )
            .unwrap()
            .flush_all();

        Self {
            pages,
            vaddr: start, 
            shared: false,
            flags,
        }
    }

    /// Allocated an area and map it in page table.
    pub fn new_alloc(
        start: VirtAddr,
        num_pages: usize,
        flags: MappingFlags,
        data: Option<&[u8]>,
        page_table: &mut PageTable,
    ) -> AxResult<Self> {
        let pages = PhysPage::alloc_contiguous(num_pages, PAGE_SIZE_4K, data).unwrap()
            .into_iter()
            .map(|page| page.map(|page| Arc::new(Mutex::new(page))))
            .collect::<Vec<_>>();

        debug!(
            "start: {:X?}, size: {:X},  page start: {:X?} flags: {:?}",
            start,
            num_pages * PAGE_SIZE_4K,
            pages[0].as_ref().unwrap().lock().start_vaddr,
            flags
        );

        page_table
            .map_region(
                start,
                |_| { virt_to_phys(pages[0].as_ref().unwrap().lock().start_vaddr) },
                num_pages * PAGE_SIZE_4K,
                flags,
                false,
                true
            )
            .unwrap()
            .flush_all();
        Ok(Self {
            pages,
            vaddr: start,
            shared: false,
            flags
        })
    }

    /// Set the shared flag of the area.
    pub(crate) fn set_shared(&mut self, shared: bool) {
        self.shared = shared;
    }

    /// Return whether the area is shared in child process.
    pub(crate) fn is_shared(&self) -> bool {
        self.shared
    }

    /// Deallocate all phys pages and unmap the area in page table.
    pub fn dealloc(&mut self, page_table: &mut PageTable) {
        let tlb_flush = page_table.unmap_region(self.vaddr, self.size(), true).unwrap();
        tlb_flush.flush_all();
        self.pages.clear();
    }

    /// 如果处理失败，返回false，此时直接退出当前程序
    pub fn handle_page_fault(
        &mut self,
        addr: VirtAddr,
        flags: MappingFlags,
        page_table: &mut PageTable,
    ) -> bool {
        // 打印调试信息
        trace!(
            "handling {:?} page fault in area [{:?}, {:?})",
            addr,
            self.vaddr,
            self.end_va()
        );
    
        // 检查地址是否在有效范围内
        assert!(
            self.vaddr <= addr && addr < self.end_va(),
            "Try to handle page fault address out of bound"
        );
    
        // 检查访问权限是否满足要求
        if !self.flags.contains(flags) {
            error!(
                "Try to access {:?} memory addr: {:?} with {:?} flag",
                self.flags, addr, flags
            );
            return false;
        }
    
        // 计算页索引
        let page_index = (usize::from(addr) - usize::from(self.vaddr)) / PAGE_SIZE_4K;
        if page_index >= self.pages.len() {
            error!("Phys page index out of bound");
            return false;
        }
    
        // 检查页面是否已加载
        if self.pages[page_index].is_some() {
            debug!("Page fault in page already loaded");
            return true;
        }
    
        debug!("page index {}", page_index);
    
        // 分配新的物理页面
        let page = PhysPage::alloc().expect("Error allocating new phys page for page fault");
    
        debug!(
            "new phys page virtual (offset) address {:?}",
            page.start_vaddr
        );

        // 重新映射页面
        let result = page_table.remap(
            addr.align_down_4k(),              // 对齐到页边界的虚拟地址
            virt_to_phys(page.start_vaddr),    // 物理地址
            self.flags,                        // 映射标志
        );
    
        match result {
            Ok(_) => {
                // 更新页面向量
                self.pages[page_index] = Some(Arc::new(Mutex::new(page)));
                true
            }
            Err(_) => {
                error!("Failed to remap page in page fault handler");
                false
            }
        }
    }

    /// Deallocate some pages from the start of the area.
    /// This function will unmap them in a page table. You need to flush TLB after this function.
    pub fn shrink_left(&mut self, new_start: VirtAddr, page_table: &mut PageTable) {
        assert!(new_start.is_aligned_4k());

        let delete_size = new_start.as_usize() - self.vaddr.as_usize();
        let delete_pages = delete_size / PAGE_SIZE_4K;

        // remove (dealloc) phys pages
        drop(self.pages.drain(0..delete_pages));

        // unmap deleted pages
        let tlb_flush = page_table.unmap_region(self.vaddr, delete_size, true).unwrap();
        tlb_flush.flush_all();

        self.vaddr = new_start;
    }

    /// Deallocate some pages from the end of the area.
    /// This function will unmap them in a page table. You need to flush TLB after this function.
    pub fn shrink_right(&mut self, new_end: VirtAddr, page_table: &mut PageTable) {
        assert!(new_end.is_aligned_4k());

        let delete_size = self.end_va().as_usize() - new_end.as_usize();
        let delete_pages = delete_size / PAGE_SIZE_4K;

        // remove (dealloc) phys pages
        drop(
            self.pages
                .drain((self.pages.len() - delete_pages)..self.pages.len()),
        );

        // unmap deleted pages
        let tlb_flush = page_table.unmap_region(new_end, delete_size, true).unwrap();
        tlb_flush.flush_all();
    }

    /// Split this area into 2.
    pub fn split(&mut self, addr: VirtAddr) -> Self {
        assert!(addr.is_aligned_4k());

        let right_page_count = (self.end_va() - addr.as_usize()).as_usize() / PAGE_SIZE_4K;
        let right_page_range = self.pages.len() - right_page_count..self.pages.len();

        let right_pages = self.pages.drain(right_page_range).collect();

        Self {
            pages: right_pages,
            vaddr: addr,
            flags: self.flags,
            shared: self.shared
        }
    }

    /// Split this area into 3.
    pub fn split3(&mut self, start: VirtAddr, end: VirtAddr) -> (Self, Self) {
        assert!(start.is_aligned_4k());
        assert!(end.is_aligned_4k());
        assert!(start < end);
        assert!(self.vaddr < start);
        assert!(end < self.end_va());

        let right_pages = self
            .pages
            .drain(
                self.pages.len() - (self.end_va().as_usize() - end.as_usize()) / PAGE_SIZE_4K
                    ..self.pages.len(),
            )
            .collect();

        let mid_pages = self
            .pages
            .drain(
                self.pages.len() - (self.end_va().as_usize() - start.as_usize()) / PAGE_SIZE_4K
                    ..self.pages.len(),
            )
            .collect();

        let mid = Self {
            pages: mid_pages,
            vaddr: start,
            flags: self.flags,
            shared: self.shared
        };

        let right = Self {
            pages: right_pages,
            vaddr: end,
            flags: self.flags,
            shared: self.shared,
        };

        (mid, right)
    }

    /// Create a second area in the right part of the area, [self.vaddr, left_end) and
    /// [right_start, self.end_va()).
    /// This function will unmap deleted pages in a page table. You need to flush TLB after calling
    /// this.
    pub fn remove_mid(
        &mut self,
        left_end: VirtAddr,
        right_start: VirtAddr,
        page_table: &mut PageTable,
    ) -> Self {
        assert!(left_end.is_aligned_4k());
        assert!(right_start.is_aligned_4k());
        // We can have left_end == right_start, although it doesn't do anything other than create
        // two areas.
        assert!(left_end <= right_start);

        let delete_size = right_start.as_usize() - left_end.as_usize();
        let delete_range = ((left_end.as_usize() - self.vaddr.as_usize()) / PAGE_SIZE_4K)
            ..((right_start.as_usize() - self.vaddr.as_usize()) / PAGE_SIZE_4K);

        // create a right area
        let pages = self
            .pages
            .drain(((right_start.as_usize() - self.vaddr.as_usize()) / PAGE_SIZE_4K)..)
            .collect();

        let right_area = Self {
            pages,
            vaddr: right_start,
            flags: self.flags,
            shared: self.shared,
        };

        // remove pages
        let _ = self.pages.drain(delete_range);

        let tlb_flush = page_table.unmap_region(left_end, delete_size, true).unwrap();
        tlb_flush.flush_all();

        right_area
    }
}

impl MapArea {
    /// return the size of the area, which thinks the page size is default 4K.
    pub fn size(&self) -> usize {
        self.pages.len() * PAGE_SIZE_4K
    }

    /// return the end virtual address of the area.
    pub fn end_va(&self) -> VirtAddr {
        self.vaddr + self.size()
    }

    /// return whether all the pages have been allocated.
    pub fn allocated(&self) -> bool {
        self.pages.iter().all(|page| page.is_some())
    }
    /// # Safety
    /// This function is unsafe because it dereferences a raw pointer.
    /// It will return a slice of the area's memory, whose len is the same as the area's size.
    pub unsafe fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.vaddr.as_ptr(), self.size()) }
    }

    /// Fill `self` with `byte`.
    pub fn fill(&mut self, byte: u8) {
        self.pages.iter_mut().for_each(|page| {
            if let Some(page) = page {
                page.lock().fill(byte);
            }
        });
    }

    /// If [start, end) overlaps with self.
    pub fn overlap_with(&self, start: VirtAddr, end: VirtAddr) -> bool {
        self.vaddr <= start && start < self.end_va() || start <= self.vaddr && self.vaddr < end
    }

    /// If [start, end] contains self.
    pub fn contained_in(&self, start: VirtAddr, end: VirtAddr) -> bool {
        start <= self.vaddr && self.end_va() <= end
    }

    /// If self contains [start, end].
    pub fn contains(&self, start: VirtAddr, end: VirtAddr) -> bool {
        self.vaddr <= start && end <= self.end_va()
    }

    /// If self strictly contains [start, end], which stands for the start and end are not equal to self's.
    pub fn strict_contain(&self, start: VirtAddr, end: VirtAddr) -> bool {
        self.vaddr < start && end < self.end_va()
    }

    /// Update area's mapping flags and write it to page table. You need to flush TLB after calling
    /// this function.
    pub fn update_flags(&mut self, flags: MappingFlags, page_table: &mut PageTable) {
        // 更新内部标志
        self.flags = flags;

        // 计算结束地址
        let mut current_vaddr = self.vaddr;
        let end_vaddr = self.vaddr + self.size();

        // 遍历区域内的所有页面更新权限
        while current_vaddr < end_vaddr {
            // 使用 protect 方法更新权限（因为 update 也是私有的）
            if let Ok((page_size, tlb)) = page_table.protect(current_vaddr, flags) {
                // 刷新这一页的 TLB
                tlb.flush();
                // 移动到下一页
                current_vaddr += page_size as usize;
            } else {
                panic!("Failed to update flags");
            }
        }
    }

    /// # Clone the area.
    ///
    /// If the area is shared, we don't need to allocate new phys pages.
    ///
    /// If the area is not shared and all the pages have been allocated,
    /// we can allocate a contiguous area in phys memory.
    ///
    /// This function will modify the page table as well.
    ///
    /// # Arguments
    ///
    /// * `page_table` - The page table of the new child process.
    ///
    /// * `parent_page_table` - The page table of the current process.
    pub fn clone_alloc(
        &mut self,
        page_table: &mut PageTable,
        parent_page_table: &mut PageTable,
    ) -> AxResult<Self> {
        // 1. 如果是共享区域
        if self.is_shared() {
            // 1.1 在父页表中分配所有缺页的物理页
            let fault_pages: Vec<_> = self
                .pages
                .iter()
                .enumerate()
                .filter_map(|(idx, slot)| {
                    if slot.is_none() {
                        Some(self.vaddr + (idx * PAGE_SIZE_4K))
                    } else {
                        None
                    }
                })
                .collect();
            
            // 处理所有缺页
            for vaddr in fault_pages {
                self.handle_page_fault(vaddr, MappingFlags::empty(), parent_page_table);
            }
    
            // 1.2 在子页表中映射整个区域
            let pages: Vec<_> = self
                .pages
                .iter()
                .enumerate()
                .map(|(idx, slot)| {
                    let vaddr = self.vaddr + (idx * PAGE_SIZE_4K);
                    assert!(slot.is_some());
                    let page = slot.as_ref().unwrap().lock();
                    
                    // 建立映射
                    let _ = page_table
                        .map(
                            vaddr,
                            virt_to_phys(page.start_vaddr),
                            PageSize::Size4K,
                            self.flags,
                        )
                        .unwrap();
                        
                    drop(page);
                    Some(Arc::clone(slot.as_ref().unwrap()))
                })
                .collect();
    
            return Ok(Self {
                pages,
                vaddr: self.vaddr,
                flags: self.flags,
                shared: self.shared,
            });
        }
    
        // 2. 如果所有页面都已分配
        if self.allocated() {
            MapArea::new_alloc(
                self.vaddr,
                self.pages.len(),
                self.flags,
                Some(unsafe { self.as_slice() }),
                page_table,
            )
        } else {
            // 3. 部分页面已分配的情况
            let pages: Vec<_> = self
                .pages
                .iter()
                .enumerate()
                .map(|(idx, slot)| {
                    let vaddr = self.vaddr + (idx * PAGE_SIZE_4K);
                    match slot.as_ref() {
                        // 已分配的页面：复制内容并建立映射
                        Some(page) => {
                            let mut new_page = PhysPage::alloc().unwrap();
                            unsafe {
                                copy_nonoverlapping(
                                    page.lock().as_ptr(),
                                    new_page.as_mut_ptr(),
                                    PAGE_SIZE_4K,
                                );
                            }
    
                            // 建立映射
                            let _ = page_table
                                .map(
                                    vaddr,
                                    virt_to_phys(new_page.start_vaddr),
                                    PageSize::Size4K,
                                    self.flags,
                                )
                                .unwrap();
    
                            Some(Arc::new(Mutex::new(new_page)))
                        }
                        // 未分配的页面：使用map方法并设置空物理地址来触发缺页
                        None => {
                            let _ = page_table
                                .map(
                                    vaddr,
                                    PhysAddr::from(0),  // 使用0地址触发缺页
                                    PageSize::Size4K,
                                    self.flags,
                                )
                                .unwrap();
                            None
                        }
                    }
                })
                .collect();
    
            Ok(Self {
                pages,
                vaddr: self.vaddr,
                flags: self.flags,
                shared: self.shared,
            })
        }
    }
}
