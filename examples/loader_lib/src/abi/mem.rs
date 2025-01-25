use core::{ffi::c_void, ptr};

use arceos_posix_api::ctypes;

use axlog::info;

use alloc::alloc::{alloc, dealloc};
use core::alloc::Layout;

struct MemoryControlBlock {
    size: usize,
}

const CTRL_BLK_SIZE: usize = core::mem::size_of::<MemoryControlBlock>();

/// Allocate memory and return the memory address.
///
/// Returns 0 on failure (the current implementation does not trigger an exception)
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_malloc(size: ctypes::size_t) -> *mut c_void {
    info!("[ABI:Mem] malloc");
    // Allocate `(actual length) + 8`. The lowest 8 Bytes are stored in the actual allocated space size.
    // This is because free(uintptr_t) has only one parameter representing the address,
    // So we need to save in advance to know the size of the memory space that needs to be released
    let layout = Layout::from_size_align(size + CTRL_BLK_SIZE, 8).unwrap();
    unsafe {
        let ptr = alloc(layout).cast::<MemoryControlBlock>();
        assert!(!ptr.is_null(), "malloc failed");
        ptr.write(MemoryControlBlock { size });
        ptr.add(1).cast()
    }
}

/// Deallocate memory.
///
/// (WARNING) If the address to be released does not match the allocated address, an error should
/// occur, but it will NOT be checked out. This is due to the global allocator `Buddy_system`
/// (currently used) does not check the validity of address to be released.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_free(ptr: *mut c_void) {
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
        dealloc(ptr.cast(), layout)
    }
}

/// Reallocate memory block
///
/// If ptr is null, this is equivalent to malloc(size)
/// If size is 0 and ptr is not null, this is equivalent to free(ptr)
/// Otherwise, try to resize the memory block and copy data
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_realloc(ptr: *mut c_void, size: ctypes::size_t) -> *mut c_void {
    info!("[ABI:Mem] realloc");

    // 如果 ptr 为空,相当于 malloc
    if ptr.is_null() {
        return unsafe { abi_malloc(size) };
    }

    // 如果 size 为 0,相当于 free
    if size == 0 {
        unsafe { abi_free(ptr) };
        return ptr::null_mut();
    }

    // 获取原内存块大小
    let old_ptr = unsafe { ptr.cast::<MemoryControlBlock>().sub(1) };
    let old_size = unsafe { old_ptr.read().size };

    // 分配新内存
    let new_ptr = unsafe { abi_malloc(size) };
    if new_ptr.is_null() {
        return ptr::null_mut();
    }

    // 复制数据,使用较小的大小
    let copy_size = core::cmp::min(old_size, size);
    unsafe {
        core::ptr::copy_nonoverlapping(ptr, new_ptr, copy_size);
    }

    // 释放旧内存
    unsafe { abi_free(ptr) };

    new_ptr
}

/// Allocate memory and set it to zero
///
/// Allocates memory for an array of nmemb elements of size bytes and returns a pointer
/// to the allocated memory. The memory is set to zero.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_calloc(nmemb: ctypes::size_t, size: ctypes::size_t) -> *mut c_void {
    info!("[ABI:Mem] calloc");

    // 检查乘法溢出
    let total_size = match nmemb.checked_mul(size) {
        Some(size) => size,
        None => return ptr::null_mut(),
    };

    // 分配内存
    let ptr = unsafe { abi_malloc(total_size) };
    if ptr.is_null() {
        return ptr::null_mut();
    }

    // 清零
    unsafe {
        ptr::write_bytes(ptr, 0, total_size);
    }

    ptr
}
