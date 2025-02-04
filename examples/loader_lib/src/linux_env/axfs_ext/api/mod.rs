pub mod port;
pub use axio::{Read, Seek, SeekFrom, Write};
pub use port::*;

// /// Check if a path exists.
// pub fn path_exists(path: &str) -> bool {
//     crate::root::lookup(None, path).is_ok()
// }
