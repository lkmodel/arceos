//! The memory management module, which implements the memory space management of the process.
extern crate alloc;

mod set;
mod area;
mod page;

pub use set::MemorySet;