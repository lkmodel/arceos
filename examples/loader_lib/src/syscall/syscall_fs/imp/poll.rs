use axhal::{mem::VirtAddr, time::current_ticks};
use axtask::yield_now;
use bitflags::bitflags;

use crate::{
    linux_env::{axfs_ext::api::FileIO, linux_fs::fd_manager::FDM},
    syscall::{SyscallError, SyscallResult, TimeSecs},
};

use alloc::{
    string::{String, ToString},
    sync::Arc,
    vec,
    vec::Vec,
};

/// 实现`ppoll`系统调用
///
/// 其中timeout是一段相对时间,需要计算出相对于当前时间戳的绝对时间戳
///
/// # Arguments
/// * `ufds - *mut PollFd`
/// * `nfds - usize`
/// * `timeout - *const TimeSecs`
/// * `mask - usize`
pub fn syscall_ppoll(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}

/// 实现`pselect6`系统调用
/// # Arguments
/// * `nfds - usize`
/// * `readfds - *mut usize`
/// * `writefds - *mut usize`
/// * `exceptfds - *mut usize`
/// * `timeout - *const TimeSecs`
/// * `mask - usize`
pub fn syscall_pselect6(args: [usize; 6]) -> SyscallResult {
    let nfds = args[0];
    let readfds = args[1] as *mut usize;
    let writefds = args[2] as *mut usize;
    let exceptfds = args[3] as *mut usize;
    let timeout = args[4] as *const TimeSecs;
    let _mask = args[5];
    let (rfiles, rfds, mut rset) = match init_fd_set(readfds, nfds) {
        Ok(ans) => (ans.files, ans.fds, ans.shadow_bitset),
        Err(e) => return Err(e),
    };
    let (wfiles, wfds, mut wset) = match init_fd_set(writefds, nfds) {
        Ok(ans) => (ans.files, ans.fds, ans.shadow_bitset),
        Err(e) => return Err(e),
    };
    let (efiles, efds, mut eset) = match init_fd_set(exceptfds, nfds) {
        Ok(ans) => (ans.files, ans.fds, ans.shadow_bitset),
        Err(e) => return Err(e),
    };
    //    let process = current_process();

    let expire_time = if !timeout.is_null() {
        // FIX:
        //        if process
        //            .memory_set
        //            .lock()
        //            .lock()
        //            .manual_alloc_type_for_lazy(timeout)
        //            .is_err()
        //        {
        //            axlog::error!("[pselect6()] timeout addr {timeout:?} invalid");
        //            return Err(SyscallError::EFAULT);
        //        }
        current_ticks() as usize + unsafe { (*timeout).get_ticks() }
    } else {
        usize::MAX
    };

    axlog::debug!("[pselect6()]: r: {rfds:?}, w: {wfds:?}, e: {efds:?}");

    loop {
        // Why yield first?
        //
        // 当用户程序中出现如下结构：
        // while (true) { select(); }
        // 如果存在 ready 的 fd,select() 立即返回,
        // 但并不完全满足用户程序的要求,可能出现死循环。
        //
        // 因此先 yield 避免其他进程 starvation。
        //
        // 可见 iperf 测例。
        yield_now();
        //        yield_now_task();

        let mut set = 0;
        if rset.valid() {
            for i in 0..rfds.len() {
                if rfiles[i].ready_to_read() {
                    rset.set(rfds[i]);
                    set += 1;
                }
            }
        }
        if wset.valid() {
            for i in 0..wfds.len() {
                if wfiles[i].ready_to_write() {
                    wset.set(wfds[i]);
                    set += 1;
                }
            }
        }
        if eset.valid() {
            for i in 0..efds.len() {
                if efiles[i].in_exceptional_conditions() {
                    eset.set(efds[i]);
                    set += 1;
                }
            }
        }
        if set > 0 {
            return Ok(set as isize);
        }
        if current_ticks() as usize > expire_time {
            return Ok(0);
        }
        // TODO: fix this and use mask to ignore specific signal
        #[cfg(feature = "signal")]
        if let Some(signalno) = process.have_signals() {
            if signalno == SignalNo::SIGKILL as usize {
                return Err(SyscallError::EINTR);
            }
        }
    }
}

/// 根据给定的地址和长度新建一个fd set,包括文件描述符指针数组,文件描述符数值数组,以及一个bitset
fn init_fd_set(addr: *mut usize, len: usize) -> Result<PpollFdSet, SyscallError> {
    //    let process = current_process();
    //   if len >= process.fd_manager.get_limit() as usize {
    if len >= FDM.get_limit() as usize {
        axlog::error!("[pselect6()] len {len} >= limit {}", FDM.get_limit());
        return Err(SyscallError::EINVAL);
    }

    let shadow_bitset = ShadowBitset::new(addr, len);
    if addr.is_null() {
        return Ok(PpollFdSet {
            shadow_bitset,
            ..Default::default()
        });
    }

    // FIXME:
    //    let start: VirtAddr = (addr as usize).into();
    //    let end = start + (len + 7) / 8;
    //    if process.manual_alloc_range_for_lazy(start, end).is_err() {
    //        axlog::error!("[pselect6()] addr {addr:?} invalid");
    //        return Err(SyscallError::EFAULT);
    //    }

    let mut fds = Vec::new();
    let mut files = Vec::new();
    for fd in 0..len {
        if shadow_bitset.check(fd) {
            let fd_table = FDM.fd_table.lock();
            if let Some(file) = fd_table[fd].as_ref() {
                files.push(Arc::clone(file));
                fds.push(fd);
            } else {
                return Err(SyscallError::EBADF);
            }
        }
    }

    shadow_bitset.clear();

    Ok(PpollFdSet {
        files,
        fds,
        shadow_bitset,
    })
}

bitflags! {
    /// 在文件上等待或者发生过的事件
    #[derive(Clone, Copy,Debug)]
    pub struct PollEvents: u16 {
        /// 可读
        const IN = 0x0001;
        /// 可写
        const OUT = 0x0004;
        /// 错误
        const ERR = 0x0008;
        /// 挂起,如pipe另一端关闭
        const HUP = 0x0010;
        /// 无效的事件
        const NVAL = 0x0020;
    }
}

#[derive(Default)]
/// file set used for ppoll
struct PpollFdSet {
    files: Vec<Arc<dyn FileIO>>,
    fds: Vec<usize>,
    shadow_bitset: ShadowBitset,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// file descriptor used for poll
pub struct PollFd {
    /// 等待的fd
    pub fd: i32,
    /// 等待的事件
    pub events: PollEvents,
    /// 返回的事件
    pub revents: PollEvents,
}

struct ShadowBitset {
    /// start address of the bitset which is in user space
    addr: *mut usize,
    /// 是包含的bit数目,而不是字节数目
    len: usize,
}

impl Default for ShadowBitset {
    fn default() -> Self {
        Self {
            addr: core::ptr::null_mut(),
            len: 0,
        }
    }
}

impl ShadowBitset {
    /// create a new bitset
    pub fn new(addr: *mut usize, len: usize) -> Self {
        Self { addr, len }
    }

    /// check if the index is set
    pub fn check(&self, index: usize) -> bool {
        if index >= self.len {
            return false;
        }
        // 因为一次add会移动八个字节,所以这里需要除以64,即8个字节,每一个字节8位
        let byte_index = index / 64;
        let bit_index = index & 0x3f;
        unsafe { *self.addr.add(byte_index) & (1 << bit_index) != 0 }
    }

    /// set the index in the bitset
    pub fn set(&mut self, index: usize) {
        if index >= self.len {
            return;
        }
        let byte_index = index / 64;
        let bit_index = index & 0x3f;
        unsafe {
            *self.addr.add(byte_index) |= 1 << bit_index;
        }
    }

    // 清空自己
    pub fn clear(&self) {
        for i in 0..=(self.len - 1) / 64 {
            unsafe {
                *(self.addr.add(i)) = 0;
            }
        }
    }

    /// check if the bitset is valid
    ///
    /// if the addr is null, it is invalid
    pub fn valid(&self) -> bool {
        self.addr as usize != 0
    }
}
