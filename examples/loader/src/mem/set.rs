use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use axhal::{mem::{memory_regions, phys_to_virt, MemoryAddr, PhysAddr, VirtAddr, PAGE_SIZE_4K}, paging::{MappingFlags, PageTable}};
use axlog::{debug, info};

use super::area::MapArea;

/// PageTable + MemoryArea for a process (task)
pub struct MemorySet {
    page_table: PageTable,
    owned_mem: BTreeMap<usize, MapArea>,
}

impl MemorySet {
    /// Get the root page table token.
    pub fn page_table_token(&self) -> usize {
        self.page_table.root_paddr().as_usize()
    }

    /// Create a new empty MemorySet.
    pub fn new_empty() -> Self {
        Self {
            page_table: PageTable::try_new().expect("Error allocating page table."),
            owned_mem: BTreeMap::new(),
        }
    }

    /// Create a new MemorySet
    pub fn new_memory_set() -> Self {
        Self::new_with_kernel_mapped()
    }

    /// Create a new MemorySet with kernel mapped regions.
    fn new_with_kernel_mapped() -> Self {
        let mut page_table = PageTable::try_new().expect("Error allocating page table.");

        for r in memory_regions() {
            debug!(
                "mapping kernel region [0x{:x}, 0x{:x})",
                usize::from(phys_to_virt(r.paddr)),
                usize::from(phys_to_virt(r.paddr)) + r.size,
            );
            let _flush = page_table
                .map_region(phys_to_virt(r.paddr), |_| r.paddr, r.size, r.flags.into(), true, true)
                .expect("Error mapping kernel memory");
        }

        Self {
            page_table,
            owned_mem: BTreeMap::new(),
        }
    }

    /// The root page table physical address.
    pub fn page_table_root_ppn(&self) -> PhysAddr {
        self.page_table.root_paddr()
    }

    /// The max virtual address of the areas in this memory set.
    pub fn max_va(&self) -> VirtAddr {
        self.owned_mem
            .last_key_value()
            .map(|(_, area)| area.end_va())
            .unwrap_or_default()
    }

    /// Allocate contiguous region. If no data, it will create a lazy load region.
    pub fn new_region(
        &mut self,
        vaddr: VirtAddr,
        size: usize,
        flags: MappingFlags,
        data: Option<&[u8]>,
    ) {
        let num_pages = (size + PAGE_SIZE_4K - 1) / PAGE_SIZE_4K;

        let area = match data {
            Some(data) => MapArea::new_alloc(
                vaddr,
                num_pages,
                flags,
                Some(data),
                &mut self.page_table,
            )
            .unwrap(),
            None => MapArea::new_lazy(vaddr, num_pages, flags, &mut self.page_table),
        };

        debug!(
            "allocating [0x{:x}, 0x{:x}) to [0x{:x}, 0x{:x}) flag: {:?}",
            usize::from(vaddr),
            usize::from(vaddr) + size,
            usize::from(area.vaddr),
            usize::from(area.vaddr) + area.size(),
            flags
        );

        // self.owned_mem.insert(area.vaddr.into(), area);
        assert!(self.owned_mem.insert(area.vaddr.into(), area).is_none());
    }

    /// Make [start, end) unmapped and dealloced. You need to flush TLB after this.
    ///
    /// NOTE: modified map area will have the same PhysAddr.
    pub fn split_for_area(&mut self, start: VirtAddr, size: usize) {
        let end = start + size;
        assert!(end.is_aligned_4k());

        // Note: Some areas will have to shrink its left part, so its key in BTree (start vaddr) have to change.
        // We get all the overlapped areas out first.

        // UPDATE: draif_filter is an unstable feature, so we implement it manually.
        let mut overlapped_area: Vec<(usize, MapArea)> = Vec::new();

        let mut prev_area: BTreeMap<usize, MapArea> = BTreeMap::new();

        for _ in 0..self.owned_mem.len() {
            let (idx, area) = self.owned_mem.pop_first().unwrap();
            if area.overlap_with(start, end) {
                overlapped_area.push((idx, area));
            } else {
                prev_area.insert(idx, area);
            }
        }

        self.owned_mem = prev_area;

        info!("splitting for [{:?}, {:?})", start, end);

        // Modify areas and insert it back to BTree.
        for (_, mut area) in overlapped_area {
            if area.contained_in(start, end) {
                info!("  drop [{:?}, {:?})", area.vaddr, area.end_va());
                area.dealloc(&mut self.page_table);
                // drop area
                drop(area);
            } else if area.strict_contain(start, end) {
                info!(
                    "  split [{:?}, {:?}) into 2 areas",
                    area.vaddr,
                    area.end_va()
                );
                let new_area = area.remove_mid(start, end, &mut self.page_table);

                assert!(self
                    .owned_mem
                    .insert(new_area.vaddr.into(), new_area)
                    .is_none());
                assert!(self.owned_mem.insert(area.vaddr.into(), area).is_none());
            } else if start <= area.vaddr && area.vaddr < end {
                info!(
                    "  shrink_left [{:?}, {:?}) to [{:?}, {:?})",
                    area.vaddr,
                    area.end_va(),
                    end,
                    area.end_va()
                );
                area.shrink_left(end, &mut self.page_table);

                assert!(self.owned_mem.insert(area.vaddr.into(), area).is_none());
            } else {
                info!(
                    "  shrink_right [{:?}, {:?}) to [{:?}, {:?})",
                    area.vaddr,
                    area.end_va(),
                    area.vaddr,
                    start
                );
                area.shrink_right(start, &mut self.page_table);

                assert!(self.owned_mem.insert(area.vaddr.into(), area).is_none());
            }
        }
    }
}