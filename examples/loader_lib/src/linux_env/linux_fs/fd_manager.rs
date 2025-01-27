//!`todo`重构fd_table, fd_allocator
extern crate alloc;
use crate::linux_env::{
    axfs_ext::api::{FileIO, OpenFlags},
    linux_fs::stdio::{Stdin, Stdout},
};
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use axlog::info;
use axsync::Mutex;
use core::sync::atomic::{AtomicI32, AtomicU64};
use lazyinit::LazyInit;

/// 通常情况下，一个 FdManager 足以满足单进程多线程的需求。
pub static FDMANAGER: LazyInit<Mutex<FdManager>> = LazyInit::new();
const FD_LIMIT_ORIGIN: usize = 1025;

pub struct FdManager {
    /// 保存文件描述符的数组
    pub fd_table: Mutex<Vec<Option<Arc<dyn FileIO>>>>,
    /// 保存文件描述符的数组的最大长度
    pub limit: AtomicU64,
    /// 创建文件时的`mode`的掩码
    umask: AtomicI32,
    pub cwd: Mutex<String>,
}

impl FdManager {
    // pub fn new(fd_table: Vec<Option<Arc<dyn FileIO>>>, limit: usize) -> Self {
    //     Self {
    //         fd_table: Mutex::new(fd_table),
    //         limit: AtomicU64::new(limit as u64),
    //         umask: AtomicI32::new(0o022),
    //         cwd: Mutex::new(String::from("/")),
    //     }
    // }

    pub fn get_limit(&self) -> u64 {
        self.limit.load(core::sync::atomic::Ordering::Acquire)
    }

    pub fn set_limit(&self, new_limit: u64) {
        self.limit
            .store(new_limit, core::sync::atomic::Ordering::Release)
    }

    #[allow(unused)]
    pub fn get_mask(&self) -> i32 {
        self.umask.load(core::sync::atomic::Ordering::Acquire)
    }

    /// 设置新的`mask`，返回旧的`mask`
    pub fn set_mask(&self, new_mask: i32) -> i32 {
        let old_mask = self.umask.load(core::sync::atomic::Ordering::Acquire);
        self.umask
            .store(new_mask, core::sync::atomic::Ordering::Release);
        old_mask
    }

    /// 在执行 `exec()` 时关闭标记为 `CLOEXEC` 的文件
    pub fn close_on_exec(&self) {
        let mut fd_table = self.fd_table.lock();
        for (index, fd) in fd_table.iter_mut().enumerate() {
            if let Some(f) = fd {
                if f.get_status().is_close_on_exec() {
                    info!("close fd: {} on exec", index);
                    fd.take();
                }
            }
        }
        if fd_table[0].is_none() {
            fd_table[0] = Some(Arc::new(Stdin {
                flags: Mutex::new(OpenFlags::empty()),
            }));
        }
        if fd_table[1].is_none() {
            fd_table[1] = Some(Arc::new(Stdout {
                flags: Mutex::new(OpenFlags::empty()),
            }));
        }
    }
}
