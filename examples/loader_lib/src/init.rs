use alloc::{sync::Arc, vec};
use axsync::Mutex;

use crate::linux_env::{
    axfs_ext::api::OpenFlags,
    linux_api::{
        api::UNI_API,
        stdio::{Stderr, Stdin, Stdout},
    },
    process_ext::process::Process,
};

pub fn init_all() {
    UNI_API.init_once(Arc::new(Process::new(0, vec![
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
    ])));
}
