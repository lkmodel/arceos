use crate::{
    linux_env::{
        axfs_ext::api::{FileIO, FileIOType, Kstat, OpenFlags, SeekFrom},
        linux_fs::{
            fd_manager::{FDM, alloc_fd},
            link::{FilePath, create_link},
            utils::{UtilsError, deal_path, has_permission},
        },
    },
    syscall::{
        IoVec, O_CLOEXEC, StMode, SyscallError, SyscallResult, normal_file_mode,
        syscall_fs::ctype::{
            dir::{get_dir_desc, new_dir},
            epoll::{EpollCtl, EpollEvent, EpollEventType, EpollFile},
            file::{new_fd, new_inode},
        },
    },
};
use alloc::{string::ToString, sync::Arc, vec};
use axerrno::AxError;
use axfs::api::{Permissions, lookup};
use axlog::{debug, error, info, warn};
use core::slice::{from_raw_parts, from_raw_parts_mut};

// FIX: 快速开发
/// 根据给定的路径获取对应的文件stat
pub fn get_stat_in_fs(path: &FilePath) -> Result<Kstat, SyscallError> {
    // 根目录算作一个简单的目录文件，不使用特殊的stat
    // 否则在fat32中查找
    let real_path = path.path();
    let mut ans = Kstat::default();
    info!("get_stat_in_fs: {}", real_path);
    if real_path.starts_with("/var")
        || real_path.starts_with("/dev")
        || real_path.starts_with("/tmp")
        || real_path.starts_with("/proc")
        || real_path.starts_with("/sys")
    {
        if path.is_dir() {
            ans.st_dev = 2;
            ans.st_mode = normal_file_mode(StMode::S_IFDIR).bits();
            return Ok(ans);
        }
        if let Ok(node) = lookup(path.path()) {
            let mut stat = Kstat {
                st_nlink: 1,
                ..Kstat::default()
            };
            // 先检查是否在vfs中存在对应文件
            // 判断是在哪个vfs中
            if node
                .as_any()
                .downcast_ref::<axfs::axfs_devfs::DirNode>()
                .is_some()
                || node
                    .as_any()
                    .downcast_ref::<axfs::axfs_ramfs::DirNode>()
                    .is_some()
            {
                stat.st_dev = 2;
                stat.st_mode = normal_file_mode(StMode::S_IFDIR).bits();
                return Ok(stat);
            }
            if node
                .as_any()
                .downcast_ref::<axfs::axfs_devfs::ZeroDev>()
                .is_some()
                || node
                    .as_any()
                    .downcast_ref::<axfs::axfs_devfs::NullDev>()
                    .is_some()
            // FIX: 这里直接删除了相关逻辑，希望不要出现问题
            //                || node
            //                    .as_any()
            //                    .downcast_ref::<axfs::axfs_devfs::RandomDev>()
            //                    .is_some()
            {
                stat.st_mode = normal_file_mode(StMode::S_IFCHR).bits();
                return Ok(stat);
            }
            if node
                .as_any()
                .downcast_ref::<axfs::axfs_ramfs::FileNode>()
                .is_some()
            {
                stat.st_mode = normal_file_mode(StMode::S_IFREG).bits();
                stat.st_size = node.get_attr().unwrap().size();
                return Ok(stat);
            }
        }
    }
    // 是文件
    let metadata = axfs::api::metadata(path.path()).unwrap();
    if metadata.is_file() {
        if let Ok(file) = new_fd(real_path.to_string(), 0.into()) {
            match file.get_stat() {
                Ok(stat) => Ok(stat),
                Err(e) => {
                    debug!("get stat error: {:?}", e);
                    Err(SyscallError::EINVAL)
                }
            }
        } else {
            Err(SyscallError::ENOENT)
        }
    } else if metadata.is_dir() {
        // 是目录
        if let Ok(dir) = new_dir(real_path.to_string(), OpenFlags::DIR) {
            match dir.get_stat() {
                Ok(stat) => Ok(stat),
                Err(e) => {
                    debug!("get stat error: {:?}", e);
                    Err(SyscallError::EINVAL)
                }
            }
        } else {
            Err(SyscallError::ENOENT)
        }
    } else {
        // 是字符设备
        Ok(Kstat {
            st_nlink: 1,
            st_mode: normal_file_mode(StMode::S_IFCHR).bits(),
            ..Kstat::default()
        })
    }
}
