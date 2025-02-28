//! 与内存相关的系统调用

mod imp;

mod mem_syscall_id;
pub use mem_syscall_id::MemSyscallId::{self, *};

use imp::*;

use super::SyscallResult;
/// 与内存相关的系统调用
pub fn mem_syscall(syscall_id: mem_syscall_id::MemSyscallId, args: [usize; 6]) -> SyscallResult {
    match syscall_id {
        BRK => syscall_brk(args),
        MUNMAP => syscall_munmap(args),
        MREMAP => syscall_mremap(args),
        //        #[cfg(feature = "fs")]
        MMAP => syscall_mmap(args),
        MSYNC => syscall_msync(args),
        MPROTECT => syscall_mprotect(args),
        MEMBARRIER => Ok(0),
        SHMGET => syscall_shmget(args),
        SHMCTL => Ok(0),
        SHMAT => syscall_shmat(args),
        #[allow(unused)]
        _ => {
            panic!("Invalid Syscall Id: {:?}!", syscall_id);
        }
    }
}
