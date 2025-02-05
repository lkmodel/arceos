//! 支持 futex 相关的 syscall ABI

extern crate alloc;

pub mod flags;
pub mod queues;
pub mod futex;
mod jhash;