// use alloc::{sync::Arc;
// use alloc::vec;
// use alloc::vec::Vec;
// use core::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};
//
// /// The one process control block
// pub struct Process {
//     /// 文件描述符管理器
//     pub fd_manager: FdManager,
//
// }
//
// /// 与文件相关的进程方法
// impl Process {
//     /// 为进程分配一个文件描述符
//     pub fn alloc_fd(&self, fd_table: &mut Vec<Option<Arc<dyn FileIO>>>) -> AxResult<usize> {
//         for (i, fd) in fd_table.iter().enumerate() {
//             if fd.is_none() {
//                 return Ok(i);
//             }
//         }
//         if fd_table.len() >= self.fd_manager.get_limit() as usize {
//             debug!("fd table is full");
//             return Err(AxError::StorageFull);
//         }
//         fd_table.push(None);
//         Ok(fd_table.len() - 1)
//     }
//
//     /// 获取当前进程的工作目录
//     pub fn get_cwd(&self) -> String {
//         self.fd_manager.cwd.lock().clone()
//     }
// }
