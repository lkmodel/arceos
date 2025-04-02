use axhal::{
    paging::MappingFlags,
    time::{MICROS_PER_SEC, NANOS_PER_MICROS, NANOS_PER_SEC, monotonic_time_nanos, nanos_to_ticks},
};
use bitflags::*;

use crate::linux_env::linux_api::config::TIMER_FREQUENCY;

/// A flag used in `sys_dup3`
pub const O_CLOEXEC: u32 = 524288;
/// The nano seconds number per second
pub const NSEC_PER_SEC: usize = 1_000_000_000;

/// `sys_gettimeofday` 中指定的类型
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TimeVal {
    /// Seconds
    pub sec: usize,
    /// Microseconds
    pub usec: usize,
}

impl TimeVal {
    /// Turn the TimeVal to nano seconds
    pub fn turn_to_nanos(&self) -> usize {
        self.sec * NANOS_PER_SEC as usize + self.usec * NANOS_PER_MICROS as usize
    }

    /// Create a TimeVal from nano seconds
    pub fn from_micro(micro: usize) -> Self {
        TimeVal {
            sec: micro / (MICROS_PER_SEC as usize),
            usec: micro % (MICROS_PER_SEC as usize),
        }
    }

    /// Turn the TimeVal to `cpu` ticks, which is related to `cpu` frequency
    pub fn turn_to_ticks(&self) -> u64 {
        (self.sec * TIMER_FREQUENCY) as u64 + nanos_to_ticks((self.usec as u64) * NANOS_PER_MICROS)
    }
}

/// `sys_nanosleep`指定的结构体类型
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct TimeSecs {
    /// Seconds
    pub tv_sec: usize,
    /// Nanoseconds
    pub tv_nsec: usize,
}
/// 当 `nsec` 为这个特殊值时，指示修改时间为现在
pub const UTIME_NOW: usize = 0x3fffffff;
/// 当 `nsec` 为这个特殊值时，指示不修改时间
pub const UTIME_OMIT: usize = 0x3ffffffe;
impl TimeSecs {
    /// 根据当前的时间构造一个 TimeSecs
    pub fn now() -> Self {
        let nano = monotonic_time_nanos() as usize;
        let tv_sec = nano / NSEC_PER_SEC;
        let tv_nsec = nano - tv_sec * NSEC_PER_SEC;
        TimeSecs { tv_sec, tv_nsec }
    }

    /// Turn the TimeSecs to nano seconds
    pub fn turn_to_nanos(&self) -> usize {
        self.tv_sec * NSEC_PER_SEC + self.tv_nsec
    }

    /// Turn the TimeSecs to `cpu` ticks, which is related to `cpu` frequency
    pub fn get_ticks(&self) -> usize {
        self.tv_sec * TIMER_FREQUENCY + (nanos_to_ticks(self.tv_nsec as u64) as usize)
    }

    /// Set the `Timesecs` to the given time
    ///
    /// If the `nsec` is UTIME_NOW, set the time to now
    ///
    /// If the `nsec` is UTIME_OMIT, ignore the setting operation
    pub fn set_as_utime(&mut self, other: &TimeSecs) {
        match other.tv_nsec {
            UTIME_NOW => {
                *self = TimeSecs::now();
            } // 设为当前时间
            UTIME_OMIT => {} // 忽略
            _ => {
                *self = *other;
            } // 设为指定时间
        }
    }
}

numeric_enum_macro::numeric_enum! {
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    #[derive(Debug)]
    /// `sys_fcntl64` 使用的选项
    pub enum Fcntl64Cmd {
        /// 复制这个 `fd`，相当于 `sys_dup`
        F_DUPFD = 0,
        /// 获取 `cloexec` 信息，即 `exec` 成功时是否删除该 `fd`
        F_GETFD = 1,
        /// 设置 `cloexec` 信息，即 `exec` 成功时删除该 `fd`
        F_SETFD = 2,
        /// 获取 `flags` 信息
        F_GETFL = 3,
        /// 设置 `flags` 信息
        F_SETFL = 4,
        /// 复制 `fd`，然后设置 `cloexec` 信息，即 `exec` 成功时删除该 `fd`
        F_DUPFD_CLOEXEC = 1030,
    }
}

bitflags! {
    /// `sys_renameat2` 用到的选项
    pub struct RenameFlags: u32 {
        /// Nothing
        const NONE = 0;
        /// 不要替换目标位置的文件，如果预定位置已经有文件，不要删除它
        const NOREPLACE = 1 << 0;
        /// 交换原位置和目标位置的文件
        const EXCHANGE = 1 << 1;
        /// 替换后在原位置放一个 "whiteout" 类型对象，仅在一些文件系统中有用，这里不考虑
        const WHITEOUT = 1 << 2;
    }
}

bitflags! {
    /// `unlinkat`用到的选项
    pub struct UnlinkatFlags: u32 {
        /// Nothing
        const NONE = 0;
        /// 在路径名上与`rmdir`等效
        const AT_REMOVEDIR = 1 << 9;
    }
}

/// `readv/writev`使用的结构体
#[repr(C)]
pub struct IoVec {
    /// Base address of the buffer
    pub base: *mut u8,
    /// Length of the buffer
    pub len: usize,
}

bitflags! {
    /// 指定 st_mode 的选项
    pub struct StMode: u32 {
        /// Regular file
        const S_IFREG = 1 << 15;
        /// Directory
        const S_IFDIR = 1 << 14;
        /// Character device
        const S_IFCHR = 1 << 13;
        // ```
        /// 是否设置 uid/gid/sticky
        //const S_ISUID = 1 << 14;
        //const S_ISGID = 1 << 13;
        //const S_ISVTX = 1 << 12;
        /// User-read permission
        const S_IRUSR = 1 << 8;
        /// User-write permission
        const S_IWUSR = 1 << 7;
        /// User-execute permission
        const S_IXUSR = 1 << 6;
        /// Group-read permission
        const S_IRGRP = 1 << 5;
        /// Group-write permission
        const S_IWGRP = 1 << 4;
        /// Group-execute permission
        const S_IXGRP = 1 << 3;
        /// Other-read permission
        const S_IROTH = 1 << 2;
        /// Other-write permission
        const S_IWOTH = 1 << 1;
        /// Other-execute permission
        const S_IXOTH = 1 << 0;
        /// Exited-user-process status
        const WIMTRACED = 1 << 1;
        /// Continued-process status
        const WCONTINUED = 1 << 3;
    }
}
/// 文件类型，输入`IFCHR`/`IFDIR`/`IFREG`等具体类型，
/// 输出这些类型加上普遍的文件属性后得到的`mode`参数
pub fn normal_file_mode(file_type: StMode) -> StMode {
    file_type | StMode::S_IWUSR | StMode::S_IRUSR | StMode::S_IRGRP | StMode::S_IROTH
}

/// 对`futex`的操作
pub enum FutexFlags {
    /// 检查用户地址`uaddr`处的值。如果不是要求的值则等待`wake`
    Wait,
    /// 唤醒最多`val`个在等待`uaddr`位置的线程。
    Wake,
    /// 将等待`uaddr`的线程移动到`uaddr2`
    Requeue,
    /// 不支持的操作
    Unsupported,
}

impl FutexFlags {
    /// Create a FutexFlags from an i32 value
    pub fn new(val: i32) -> Self {
        match val & 0x7f {
            0 => FutexFlags::Wait,
            1 => FutexFlags::Wake,
            3 => FutexFlags::Requeue,
            _ => FutexFlags::Unsupported,
        }
    }
}

bitflags! {
    #[derive(Debug)]
    /// 指定`mmap`的选项
    pub struct MMAPPROT: u32 {
        /// 区域内容可读取
        const PROT_READ = 1 << 0;
        /// 区域内容可修改
        const PROT_WRITE = 1 << 1;
        /// 区域内容可执行
        const PROT_EXEC = 1 << 2;
    }
}

impl From<MMAPPROT> for MappingFlags {
    fn from(value: MMAPPROT) -> Self {
        let mut flags = MappingFlags::USER;
        if value.contains(MMAPPROT::PROT_READ) {
            flags |= MappingFlags::READ;
        }
        if value.contains(MMAPPROT::PROT_WRITE) {
            flags |= MappingFlags::WRITE;
        }
        if value.contains(MMAPPROT::PROT_EXEC) {
            flags |= MappingFlags::EXECUTE;
        }
        flags
    }
}

bitflags! {
    #[derive(Debug)]
    /// 指定`mmap`的选项
    pub struct MMAPFlags: u32 {
        /// 对这段内存的修改是共享的
        const MAP_SHARED = 1 << 0;
        /// 对这段内存的修改是私有的
        const MAP_PRIVATE = 1 << 1;
        // 以上两种只能选其一

        /// 取消原来这段位置的映射，即一定要映射到指定位置
        const MAP_FIXED = 1 << 4;
        /// 不映射到实际文件
        const MAP_ANONYMOUS = 1 << 5;
        /// 映射时不保留空间，即可能在实际使用`mmp`出来的内存时内存溢出
        const MAP_NORESERVE = 1 << 14;
    }
}

bitflags! {
    #[derive(Debug)]
    /// 指定`mremap`的选项
    pub struct MREMAPFlags: u32 {
        /// 允许将映射重新定位到新地址
        const MREMAP_MAYMOVE = 1 << 0;
        /// 指定映射必须移动到的页面对齐地址，必须和`MREMAP_MAYMOVE`一起使用
        const MREMAP_FIXED = 1 << 1;

        /// 将映射重新映射到新地址，但不会取消旧地址的映射，必须和`MREMAP_MAYMOVE`一起使用
        const MREMAP_DONTUNMAP = 1 << 2;
    }
}

bitflags! {
    #[derive(Debug)]
    // FIX:
    /// 指定`fstatat`的选项
    pub struct FSTATATFlags: u32 {
        const FSTATAT_EMPTY_PATH = 1 << 12;
        const FSTATAT_NO_AUTOMOUNT = 1 << 11;
        const FSTATAT_SYMLINK_NOFOLLOW = 1 << 8;
    }
}

/// 文件系统的属性
/// 具体参数定义信息来自 `https://man7.org/linux/man-pages/man2/statfs64.2.html`
#[repr(C)]
#[derive(Debug)]
pub struct FsStat {
    /// 是个`magic number`，每个知名的`fs`都各有定义，但显然我们没有
    pub f_type: i64,
    /// 最优传输块大小
    pub f_bsize: i64,
    /// 总的块数
    pub f_blocks: u64,
    /// 还剩多少块未分配
    pub f_bfree: u64,
    /// 对用户来说，还有多少块可用
    pub f_bavail: u64,
    /// 总的`inode`数
    pub f_files: u64,
    /// 空闲的`inode`数
    pub f_ffree: u64,
    /// 文件系统编号，但实际上对于不同的OS差异很大，所以不会特地去用
    pub f_fsid: [i32; 2],
    /// 文件名长度限制，这个OS默认FAT已经使用了加长命名
    pub f_namelen: isize,
    /// 片大小
    pub f_frsize: isize,
    /// 一些选项，但其实也没用到
    pub f_flags: isize,
    /// 空余 padding
    pub f_spare: [isize; 4],
}

/// 获取一个基础的`fsstat`
pub fn get_fs_stat() -> FsStat {
    FsStat {
        f_type: 0,
        f_bsize: 1024,
        f_blocks: 0x4000_0000 / 512,
        f_bfree: 1,
        f_bavail: 1,
        f_files: 1,
        f_ffree: 1,
        f_fsid: [0, 0],
        f_namelen: 256,
        f_frsize: 0x1000,
        f_flags: 0,
        f_spare: [0, 0, 0, 0],
    }
}

numeric_enum_macro::numeric_enum! {
    #[repr(usize)]
    #[allow(non_camel_case_types)]
    #[derive(PartialEq,Eq)]
    /// sys_fcntl64 使用的选项
    pub enum ClockId {
        /// real-time clock
        CLOCK_REALTIME = 0,
        /// monotonic clock
        CLOCK_MONOTONIC = 1,
    }
}

/// sys_times 中指定的结构体类型
#[repr(C)]
pub struct Tms {
    /// 进程用户态执行时间，单位为us
    pub tms_utime: usize,
    /// 进程内核态执行时间，单位为us
    pub tms_stime: usize,
    /// 子进程用户态执行时间和，单位为us
    pub tms_cutime: usize,
    /// 子进程内核态执行时间和，单位为us
    pub tms_cstime: usize,
}

/// sys_uname 中指定的结构体类型
#[repr(C)]
pub struct UtsName {
    /// 系统名称
    pub sysname: [u8; 65],
    /// 网络上的主机名称
    pub nodename: [u8; 65],
    /// 发行编号
    pub release: [u8; 65],
    /// 版本
    pub version: [u8; 65],
    /// 硬件类型
    pub machine: [u8; 65],
    /// 域名
    pub domainname: [u8; 65],
}

impl Default for UtsName {
    fn default() -> Self {
        Self {
            sysname: Self::from_str("LK_ArceOS"),
            nodename: Self::from_str("LK_ArceOS - machine[0]"),
            release: Self::from_str("100"),
            version: Self::from_str("1.0"),
            machine: Self::from_str("RISC-V 64 on SIFIVE FU740"),
            domainname: Self::from_str("https://github.com/Azure-stars/arceos"),
        }
    }
}

impl UtsName {
    fn from_str(info: &str) -> [u8; 65] {
        let mut data: [u8; 65] = [0; 65];
        data[..info.len()].copy_from_slice(info.as_bytes());
        data
    }
}

/// 目录项
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct DirEnt {
    /// 索引结点号
    pub d_ino: u64,
    /// 到下一个`dirent`的偏移
    pub d_off: u64,
    /// 当前`dirent`的长度
    pub d_reclen: u16,
    /// 文件类型
    pub d_type: u8,
    /// 文件名
    pub d_name: [u8; 0],
}
#[allow(unused)]
/// 目录项类型
pub enum DirEntType {
    /// 未知类型文件
    Unknown = 0,
    /// 先进先出的文件/队列
    Fifo = 1,
    /// 字符设备
    Chr = 2,
    /// 目录
    Dir = 4,
    /// 块设备
    Blk = 6,
    /// 常规文件
    Reg = 8,
    /// 符号链接
    Lnk = 10,
    /// socket
    Socket = 12,
    /// whiteout
    Wht = 14,
}

impl DirEnt {
    /// 定长部分大小
    pub fn fixed_size() -> usize {
        8 + 8 + 2 + 1
    }
    /// 设置定长部分
    pub fn set_fixed_part(&mut self, ino: u64, off: u64, reclen: usize, type_: DirEntType) {
        self.d_ino = ino;
        self.d_off = off;
        self.d_reclen = reclen as u16;
        self.d_type = type_ as u8;
    }
}

/// `sys_prlimit64` 使用的数组
#[repr(C)]
pub struct RLimit {
    /// 软上限
    pub rlim_cur: u64,
    /// 硬上限
    pub rlim_max: u64,
}
// `sys_prlimit64` 使用的选项
/// 用户栈大小
pub const RLIMIT_STACK: i32 = 3;
/// 可以打开的 `fd` 数
pub const RLIMIT_NOFILE: i32 = 7;
/// 用户地址空间的最大大小
pub const RLIMIT_AS: i32 = 9;

/// robust list
#[repr(C)]
pub struct RobustList {
    head: usize,
    off: usize,
    pending: usize,
}
