cfg_if::cfg_if! {
    if #[cfg(feature = "myfs")] {
        pub mod myfs;
        pub const BLOCK_SIZE: usize = 512;
    } else if #[cfg(feature = "fatfs")] {
        pub mod fatfs;
        pub use fatfs::BLOCK_SIZE;
    } else if #[cfg(feature = "ext4fs")] {
        pub mod ext4fs;
        pub use ext4fs::BLOCK_SIZE;
    } else if #[cfg(feature = "ext4_rs")] {

        pub mod ext4;
        pub use ext4::BLOCK_SIZE;
    }
}

// #[cfg(feature = "ext4_rs")]
// pub mod ext4;

#[cfg(feature = "devfs")]
pub use axfs_devfs as devfs;

#[cfg(feature = "ramfs")]
pub use axfs_ramfs as ramfs;
