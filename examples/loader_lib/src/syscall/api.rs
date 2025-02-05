use crate::linux_env::axfs_ext::api::OpenFlags;
use alloc::{string::String, sync::Arc};
use axerrno::AxResult;
use axfs::api::File;
use axhal::arch::{flush_tlb, write_page_table_root};
use axlog::debug;

// use axhal::{
//     arch::{flush_tlb, write_page_table_root},
//     KERNEL_PROCESS_ID,
// };
// use axprocess::{yield_now_task, PID2PC};
// use axruntime::KERNEL_PAGE_TABLE;
// use axtask::{TaskId, EXITED_TASKS};

/// 若使用多次new file打开同名文件，那么不同new file之间读写指针不共享，但是修改的内容是共享的
pub fn new_file(path: &str, flags: &OpenFlags) -> AxResult<File> {
    let mut file = File::options();
    file.read(flags.readable());
    file.write(flags.writable());
    file.create(flags.creatable());
    file.create_new(flags.new_creatable());
    file.open(path)
}
