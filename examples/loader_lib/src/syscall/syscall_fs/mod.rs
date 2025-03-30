pub mod ctype;
mod fs_syscall_id;
pub mod imp;

pub use fs_syscall_id::FsSyscallId::{self, *};

use super::SyscallResult;
use imp::*;

/// 文件系统相关系统调用
pub fn fs_syscall(syscall_id: fs_syscall_id::FsSyscallId, args: [usize; 6]) -> SyscallResult {
    match syscall_id {
        OPENAT => syscall_openat(args),
        CLOSE => syscall_close(args),
        READ => syscall_read(args),
        WRITE => syscall_write(args),
        GETCWD => syscall_getcwd(args),
        PIPE2 => syscall_pipe2(args),
        DUP => syscall_dup(args),
        DUP3 => syscall_dup3(args),
        MKDIRAT => syscall_mkdirat(args),
        CHDIR => syscall_chdir(args),
        GETDENTS64 => syscall_getdents64(args),
        MOUNT => syscall_mount(args),
        UNMOUNT => syscall_umount(args),
        FSTAT => syscall_fstat(args),
        RENAMEAT | RENAMEAT2 => syscall_renameat2(args),
        READV => syscall_readv(args),
        WRITEV => syscall_writev(args),
        FCNTL64 => syscall_fcntl64(args),
        FSTATAT => syscall_fstatat(args),
        STATFS => syscall_statfs(args),
        CHROOT => unimplemented!(),
        FCHMOD => syscall_fchmod(args),
        FCHMODAT => syscall_fchmodat(args),
        FCHOWNAT => syscall_fchownat(args),
        FCHOWN => syscall_fchown(args),
        FACCESSAT => syscall_faccessat(args),
        LSEEK => syscall_lseek(args),
        PREAD64 => syscall_pread64(args),
        PREADLINKAT => syscall_readlinkat(args),
        PWRITE64 => syscall_pwrite64(args),
        SENDFILE64 => syscall_sendfile64(args),
        FSYNC => Ok(0),
        FTRUNCATE64 => {
            syscall_ftruncate64(args)
            // 0
        }
        IOCTL => syscall_ioctl(args),
        // 不做处理即可
        SYNC => Ok(0),
        FDATASYNC => unimplemented!(),
        COPYFILERANGE => syscall_copyfilerange(args),
        LINKAT => sys_linkat(args),
        UNLINKAT => syscall_unlinkat(args),
        SYMLINKAT => Ok(0),
        UTIMENSAT => syscall_utimensat(args),
        EPOLL_CREATE => syscall_epoll_create1(args),
        EPOLL_CTL => syscall_epoll_ctl(args),
        EPOLL_WAIT => syscall_epoll_wait(args),
        PPOLL => syscall_ppoll(args),
        SETPRIORITY => Ok(0),
        GETPRIORITY => Ok(0),
        PSELECT6 => syscall_pselect6(args),
        EVENTFD => syscall_eventfd(args),
    }
}
