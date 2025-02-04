use alloc::{sync::Arc, vec};
use axsync::Mutex;

use crate::linux_env::{
    axfs_ext::api::OpenFlags,
    linux_fs::{
        fd_manager::{FD_LIMIT_ORIGIN, FDM, FdManager},
        stdio::{Stderr, Stdin, Stdout},
    },
};

pub fn init_all() {
    FDM.init_once(FdManager::new(
        vec![
            // 标准输入
            Some(Arc::new(Stdin {
                flags: Mutex::new(OpenFlags::empty()),
            })),
            // 标准输出
            Some(Arc::new(Stdout {
                flags: Mutex::new(OpenFlags::empty()),
            })),
            // 标准错误
            Some(Arc::new(Stderr {
                flags: Mutex::new(OpenFlags::empty()),
            })),
        ],
        FD_LIMIT_ORIGIN,
    ));
}
