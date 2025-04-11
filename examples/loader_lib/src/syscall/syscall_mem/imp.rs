// use crate::{MMAPFlags, MMAPPROT, MREMAPFlags, SyscallError, SyscallResult, syscall_fs::FileDesc};
// extern crate alloc;
//
// use axhal::{arch::flush_tlb, mem::VirtAddr, paging::MappingFlags};
// use axmem::MemorySet;
//
// use axprocess::current_process;

use core::alloc::Layout;

use alloc::{
    alloc::{alloc, dealloc},
    boxed::Box,
};
use axhal::{arch::flush_tlb, mem::VirtAddr, paging::MappingFlags};
use axlog::{debug, error, info};
use bitflags::bitflags;

use crate::{
    config::MAX_HEAP_SIZE,
    linux_env::linux_api::api::process_api,
    syscall::{
        MMAPFlags, MMAPPROT, SyscallError, SyscallResult, syscall_fs::ctype::file::FileDesc,
    },
};

static mut HEAP_BASE: usize = 0;
static mut HEAP_TOP: usize = 0;

/// 修改用户堆大小，
///
/// - 如输入`brk`为`0`，则返回堆顶地址
/// - 重新设置堆顶地址，如成功则返回设置后的堆顶地址，否则保持不变，并返回之前的堆顶地址。
///
/// # Arguments
/// * `brk - usize`
pub fn syscall_brk(args: [usize; 6]) -> SyscallResult {
    let brk = args[0];
    unsafe {
        if HEAP_BASE == 0 {
            error!("Unikernel Heap not initialized!");
            return Err(SyscallError::ENOMEM);
        }
        if brk == 0 {
            return Ok(HEAP_TOP as isize);
        }
        if brk >= HEAP_BASE && brk <= HEAP_BASE + MAX_HEAP_SIZE {
            HEAP_TOP = brk;
            Ok(HEAP_TOP as isize)
        } else {
            // 在物理内存模型下，超出范围的 brk 请求通常是无效的
            Err(SyscallError::EINVAL)
        }
    }
}

/// 将文件内容映射到内存中 (`Unikernel` 版本)
/// 在 `Unikernel` 中，由于没有独立的进程地址空间和 `MMU`，
/// `mmap` 的概念会简化为直接在某个物理地址范围内分配内存，
/// 并将文件内容拷贝到该内存中。权限管理也会失效。
/// 这里我们假设文件系统是可直接访问的。
///
/// # Arguments
/// * `start` - `usize` (期望的物理地址，如果为 `0` 则由系统分配)
/// * `len` - `usize` (映射长度)
/// * `prot` - `usize` (权限，在 `Unikernel` 中通常忽略)
/// * `flags` - `usize` (标志，`MAP_FIXED` 如果 `start != 0` 则强制使用，`MAP_ANONYMOUS` 表示匿名映射)
/// * `fd` - `i32` (文件描述符，用于标识要映射的文件)
/// * `offset` - `usize` (文件偏移)
pub fn syscall_mmap(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    // let start = args[0];
    // let len = args[1];
    // let _prot = MMAPPROT::from_bits_truncate(args[2] as u32);
    // let flags = MMAPFlags::from_bits_truncate(args[3] as u32);
    // let fd = args[4] as i32;
    // let offset = args[5];

    // let map_fixed = flags.contains(MMAPFlags::MAP_FIXED);
    // let map_anonymous = flags.contains(MMAPFlags::MAP_ANONYMOUS);

    // let process = process_api();

    // /if map_fixed && start == 0 {
    //     return Err(SyscallError::EINVAL);
    // }

    // if map_anonymous {
    //     // 匿名映射，直接分配物理内存
    //     let addr = if start == 0 {
    //         unsafe {
    //             let layout = Layout::from_size_align(len, 8).map_err(|_| SyscallError::ENOMEM)?;
    //             let ptr = alloc(layout);
    //             if ptr.is_null() {
    //                 return Err(SyscallError::ENOMEM);
    //             }
    //             ptr as usize
    //         }
    //     } else if map_fixed {
    //         // MAP_FIXED 的匿名映射，直接使用指定的物理地址
    //         start
    //     } else {
    //         return Err(SyscallError::EINVAL); // `start != 0` 但不是 `MAP_FIXED`
    //     };
    //     info!(
    //         "Unikernel mmap (anonymous) at 0x{:x}, len 0x{:x}",
    //         addr, len
    //     );
    //     Ok(addr as isize)
    // } else {
    //     // file backend
    //     debug!("[mmap] fd: {}, offset: 0x{:x}", fd, offset);
    //     if fd >= process.fd_manager.fd_table.lock().len() as i32 || fd < 0 {
    //         return Err(SyscallError::EINVAL);
    //     }

    //     let file = match &process.fd_manager.fd_table.lock()[fd as usize] {
    //         // 文件描述符表里面存的是文件描述符，这很合理罢
    //         Some(file) => alloc::boxed::Box::new(
    //             file.as_any()
    //                 .downcast_ref::<FileDesc>()
    //                 .expect("Try to mmap with a non-file backend")
    //                 .file
    //                 .lock()
    //                 .clone(),
    //         ),
    //         // fd not found
    //         None => return Err(SyscallError::EINVAL),
    //     };

    //     let addr = if start == 0 {
    //         unsafe {
    //             let layout = Layout::from_size_align(len, 8).map_err(|_| SyscallError::ENOMEM)?;
    //             let ptr = alloc(layout);
    //             if ptr.is_null() {
    //                 return Err(SyscallError::ENOMEM);
    //             }
    //             ptr as usize
    //         }
    //     } else if map_fixed {
    //         start
    //     } else {
    //         return Err(SyscallError::EINVAL); // start != 0 但不是 MAP_FIXED
    //     };

    //     unsafe {
    //         let bytes_read = unikernel_read_file(fd, offset, addr as *mut u8, len);
    //         if bytes_read < 0 {
    //             // 模拟读取错误
    //             dealloc(addr as *mut u8, Layout::from_size_align(len, 8).unwrap());
    //             return Err(SyscallError::EIO);
    //         }
    //         info!(
    //             "Unikernel mmap (file, fd {}) at 0x{:x}, len 0x{:x}, read {} bytes",
    //             fd, addr, len, bytes_read
    //         );
    //         Ok(addr as isize)
    //     }
    // }
}

/// 释放内存映射 (`Unikernel` 版本)
pub fn syscall_munmap(args: [usize; 6]) -> SyscallResult {
    let start = args[0];
    let len = args[1];
    unsafe {
        if start != 0 && len > 0 {
            let layout = Layout::from_size_align(len, 8).map_err(|_| SyscallError::EINVAL)?;
            dealloc(start as *mut u8, layout);
            info!("Unikernel munmap at 0x{:x}, len 0x{:x}", start, len);
        }
        Ok(0)
    }
}

/// 同步内存映射到文件 (`Unikernel` 版本)
/// 在没有独立进程和虚拟内存的情况下，`msync` 的意义不大，
/// 因为所有修改都直接发生在物理内存上。
/// 这里我们简单地返回成功。
pub fn syscall_msync(args: [usize; 6]) -> SyscallResult {
    let start = args[0];
    let len = args[1];
    info!("Unikernel msync at 0x{:x}, len 0x{:x} (no-op)", start, len);
    Ok(0)
}

/// 修改内存映射的保护属性 (`Unikernel` 版本)
/// 在没有特权级和 `MMU` 的情况下，`mprotect` 无法实现。
/// 这里我们简单地返回成功。
pub fn syscall_mprotect(args: [usize; 6]) -> SyscallResult {
    let start = args[0];
    let len = args[1];
    let prot = args[2];
    info!(
        "Unikernel mprotect at 0x{:x}, len 0x{:x}, prot {} (no-op)",
        start, len, prot
    );
    Ok(0)
}

/// 重新映射内存段 (Unikernel 版本)
/// 在物理内存模型下，mremap 的实现会比较直接的物理内存块移动或调整大小。
pub fn syscall_mremap(args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    // let old_addr = args[0];
    // let old_size = args[1];
    // let new_size = args[2];
    // let flags = args[3];
    // let new_addr = args[4];

    // info!(
    //     "Unikernel mremap old: 0x{:x} sz 0x{:x}, new: 0x{:x} sz 0x{:x}, flags {}",
    //     old_addr, old_size, new_addr, new_size, flags
    // );

    // if old_addr == 0 || old_size == 0 || new_size == 0 {
    //     return Err(SyscallError::EINVAL);
    // }

    // let may_move = (flags & 0x1) != 0; // 对应 MREMAPFlags::MREMAP_MAYMOVE
    // let fixed = (flags & 0x2) != 0; // 对应 MREMAPFlags::MREMAP_FIXED

    // unsafe {
    //     let old_layout = Layout::from_size_align(old_size, 8).map_err(|_| SyscallError::EINVAL)?;
    //     let new_layout = Layout::from_size_align(new_size, 8).map_err(|_| SyscallError::EINVAL)?;

    //     if new_size <= old_size {
    //         // 缩小内存，可以直接返回旧地址
    //         info!(
    //             "Unikernel mremap: shrinking memory at 0x{:x} from 0x{:x} to 0x{:x}",
    //             old_addr, old_size, new_size
    //         );
    //         Ok(old_addr as isize)
    //     } else if may_move {
    //         // 尝试分配新的内存并拷贝数据
    //         let new_ptr = alloc(new_layout);
    //         if new_ptr.is_null() {
    //             return Err(SyscallError::ENOMEM);
    //         }
    //         core::ptr::copy_nonoverlapping(old_addr as *const u8, new_ptr as *mut u8, old_size);
    //         dealloc(old_addr as *mut u8, old_layout);
    //         info!(
    //             "Unikernel mremap: moved memory from 0x{:x} to 0x{:x}, new size 0x{:x}",
    //             old_addr, new_ptr as usize, new_size
    //         );
    //         Ok(new_ptr as usize as isize)
    //     } else if fixed && new_addr != 0 {
    //         // 在固定地址重新分配（需要确保新地址可用，这里简化处理）
    //         // 注意：这可能覆盖原有数据，需要谨慎处理
    //         let new_ptr = new_addr as *mut u8;
    //         // 假设新地址有足够的空间，并且可以安全覆盖
    //         core::ptr::copy_nonoverlapping(old_addr as *const u8, new_ptr, old_size);
    //         dealloc(old_addr as *mut u8, old_layout);
    //         info!(
    //             "Unikernel mremap: moved (fixed) memory from 0x{:x} to 0x{:x}, new size 0x{:x}",
    //             old_addr, new_addr, new_size
    //         );
    //         Ok(new_addr as isize)
    //     } else {
    //         Err(SyscallError::ENOMEM) // 无法在原位置扩展，也不允许移动
    //     }
    // }
}
const IPC_PRIVATE: i32 = 0;

bitflags! {
    #[derive(Debug)]
    struct ShmFlags: i32 {
        const IPC_CREAT = 0o1000;
        const IPC_EXCL = 0o2000;
        // FIX: Unimplemented:
        const SHM_HUGETLB = 0o4000;
        const SHM_NORESERVE = 0o10000;
    }
}

/// 获取共享内存段 (`Unikernel` 版本)
/// 在单地址空间模型下，共享内存的概念变得简单，
/// 只需要分配一块物理内存，并返回其地址即可。
/// key 和 flags 的处理也会简化。
pub fn syscall_shmget(args: [usize; 6]) -> SyscallResult {
    let key = args[0] as i32;
    let size = args[1];
    let flags = args[2] as i32;

    info!(
        "Unikernel shmget key {}, size 0x{:x}, flags {}",
        key, size, flags
    );

    unsafe {
        let layout = Layout::from_size_align(size, 8).map_err(|_| SyscallError::ENOMEM)?;
        let ptr = alloc(layout);
        if ptr.is_null() {
            return Err(SyscallError::ENOMEM);
        }
        info!("Unikernel shmget allocated at 0x{:x}", ptr as usize);
        Ok(ptr as usize as isize)
    }
}

bitflags! {
    #[derive(Debug)]
    struct ShmAtFlags: i32 {
        const SHM_RND = 0o20000;
        const SHM_EXEC = 0o100000;
        const SHM_RDONLY = 0o10000;
        const SHM_REMAP = 0o40000;
    }
}

/// 连接共享内存段到进程地址空间 (`Unikernel` 版本)
/// 在单地址空间模型下，这步只是简单地返回共享内存的地址，
/// 因为所有内存都是直接可访问的。
pub fn syscall_shmat(args: [usize; 6]) -> SyscallResult {
    let shmid = args[0];
    let addr = args[1];
    let flags = args[2];

    info!(
        "Unikernel shmat shmid {}, addr 0x{:x}, flags {}",
        shmid, addr, flags
    );

    // 在 `Unikernel` 中，shmid 就是 `shmget` 返回的物理地址
    Ok(shmid as isize)
}

/// 锁定内存页 (`Unikernel` 版本)
/// 在没有分页和特权级的 `Unikernel` 中，`mlock` 通常是一个空操作。
pub fn syscall_mlock(args: [usize; 6]) -> SyscallResult {
    let start = args[0];
    let len = args[1];
    info!("Unikernel mlock at 0x{:x}, len 0x{:x} (no-op)", start, len);
    Ok(0)
}
