use core::{ffi::c_void, hint::black_box, ptr};

use arceos_posix_api::ctypes;

use axlog::info;

use alloc::alloc::{alloc, dealloc};
use core::alloc::Layout;

struct MemoryControlBlock {
    size: usize,
}

const CTRL_BLK_SIZE: usize = core::mem::size_of::<MemoryControlBlock>();

#[derive(Debug)]
struct AllocatedBlock {
    ptr: *mut c_void,
    next: *mut AllocatedBlock,
}

static mut ALLOCATED_WARMUP: *mut u8 = ptr::null_mut();
static mut ALLOCATED_BLOCKS: *mut AllocatedBlock = ptr::null_mut();

/// 清理函数，使用 abi_free 释放所有已分配的内存块
pub fn cleanup_memory() {
    info!("Clean up memory");
    unsafe {
        let mut current: *mut AllocatedBlock = ALLOCATED_BLOCKS;
        while !current.is_null() {
            let ptr = (*current).ptr.cast(); // 获取用户数据指针
            abi_free(ptr); // 调用 abi_free 释放内存
            current = (*current).next;
        }
        ALLOCATED_BLOCKS = ptr::null_mut();
    }
}

/// Allocate memory and return the memory address.
///
/// Returns 0 on failure (the current implementation does not trigger an exception)
#[unsafe(no_mangle)]
pub extern "C" fn abi_malloc(size: ctypes::size_t) -> *mut c_void {
    info!("[ABI:Mem] malloc entry; size 0x{:x}", size);
    // Allocate `(actual length) + 8`. The lowest 8 Bytes are stored in the actual allocated space size.
    // This is because free(`uintptr_t`) has only one parameter representing the address,
    // So we need to save in advance to know the size of the memory space that needs to be released
    let layout = Layout::from_size_align(size + CTRL_BLK_SIZE, 8).unwrap();

    unsafe {
        if !ALLOCATED_WARMUP.is_null() {
            dealloc(ALLOCATED_WARMUP, Layout::new::<AllocatedBlock>());
        }
        // FIX: 初始化分配器热身，在分配过大内存的时候，可能会导致block的分配报错。
        // 猜测原因：
        // 分配过大块的时候，写爆之后，再分配就会出现`S mode page fault`
        // 因此，手动触发堆或页分配的`lazy`初始化逻辑
        let warmup = black_box(alloc(Layout::new::<AllocatedBlock>()));
        ALLOCATED_WARMUP = warmup;

        let ptr = alloc(layout).cast::<MemoryControlBlock>();
        assert!(!ptr.is_null(), "malloc failed");
        ptr.write(MemoryControlBlock { size });
        let ptr: *mut c_void = ptr.add(1).cast();

        // 创建链表节点并添加到链表头部
        let block = alloc(Layout::new::<AllocatedBlock>()).cast::<AllocatedBlock>();
        assert!(!block.is_null(), "alloc failed");
        (*block).ptr = ptr;
        (*block).next = ALLOCATED_BLOCKS;
        ALLOCATED_BLOCKS = block;

        info!("[ABI:Mem] malloc return; ptr {:?}", ptr);

        ptr
    }
}

/// Deallocate memory.
///
/// (WARNING) If the address to be released does not match the allocated address, an error should
/// occur, but it will NOT be checked out. This is due to the global allocator `Buddy_system`
/// (currently used) does not check the validity of address to be released.
#[unsafe(no_mangle)]
pub extern "C" fn abi_free(ptr: *mut c_void) {
    info!("[ABI:Mem] free");
    if ptr.is_null() {
        return;
    }
    let ptr = ptr.cast::<MemoryControlBlock>();
    assert!(ptr as usize > CTRL_BLK_SIZE, "free a null pointer");
    unsafe {
        let ptr = ptr.sub(1);
        let size = ptr.read().size;
        let layout = Layout::from_size_align(size + CTRL_BLK_SIZE, 8).unwrap();
        dealloc(ptr.cast(), layout);

        // 从链表中删除节点
        let mut current: *mut AllocatedBlock = ALLOCATED_BLOCKS;
        let mut prev: *mut AllocatedBlock = ptr::null_mut();
        let ptr = ptr.add(1);

        while !current.is_null() {
            if (*current).ptr == (ptr as *mut c_void) {
                if prev.is_null() {
                    ALLOCATED_BLOCKS = (*current).next;
                } else {
                    (*prev).next = (*current).next;
                }
                return;
            }
            prev = current;
            current = (*current).next;
        }
    }
}

/// Reallocate memory block
///
/// If ptr is null, this is equivalent to `malloc(size)`
/// If size is 0 and ptr is not null, this is equivalent to free(ptr)
/// Otherwise, try to resize the memory block and copy data
#[unsafe(no_mangle)]
pub extern "C" fn abi_realloc(ptr: *mut c_void, size: ctypes::size_t) -> *mut c_void {
    info!("[ABI:Mem] realloc");
    unsafe {
        // 如果 ptr 为空,相当于 `malloc`
        if ptr.is_null() {
            return abi_malloc(size);
        }

        // 如果 size 为 0,相当于 free
        if size == 0 {
            abi_free(ptr);
            return ptr::null_mut();
        }

        // 获取原内存块大小
        let old_ptr = ptr.cast::<MemoryControlBlock>().sub(1);
        let old_size = old_ptr.read().size;

        // 分配新内存
        let new_ptr = abi_malloc(size);
        if new_ptr.is_null() {
            return ptr::null_mut();
        }

        // 复制数据,使用较小的大小
        let copy_size = core::cmp::min(old_size, size);
        core::ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);

        // 更新链表节点信息
        let mut current: *mut AllocatedBlock = ALLOCATED_BLOCKS;
        while !current.is_null() {
            if (*current).ptr == ptr {
                (*current).ptr = new_ptr; // 更新指针
            }
            current = (*current).next;
        }

        // 释放旧内存
        abi_free(ptr);
        new_ptr
    }
}

/// Allocate memory and set it to zero
///
/// Allocates memory for an array of nmemb elements of size bytes and returns a pointer
/// to the allocated memory. The memory is set to zero.
#[unsafe(no_mangle)]
pub extern "C" fn abi_calloc(nmemb: ctypes::size_t, size: ctypes::size_t) -> *mut c_void {
    info!("[ABI:Mem] calloc");

    // 检查乘法溢出
    let total_size = match nmemb.checked_mul(size) {
        Some(size) => size,
        None => return ptr::null_mut(),
    };

    // 分配内存
    let ptr = abi_malloc(total_size);
    if ptr.is_null() {
        return ptr::null_mut();
    }

    // 清零内存
    unsafe {
        ptr::write_bytes(ptr, 0, total_size);
    }

    ptr
}
