use axhal::{
    paging::MappingFlags,
    time::{MICROS_PER_SEC, NANOS_PER_MICROS, NANOS_PER_SEC, monotonic_time_nanos, nanos_to_ticks},
};
use bitflags::*;
use core::panic;

/// A flag used in `sys_dup3`
pub const O_CLOEXEC: u32 = 524288;
/// The nano seconds number per second
pub const NSEC_PER_SEC: usize = 1_000_000_000;
// bitflags! {
//     /// 指定 sys_wait4 的选项
//     pub struct WaitFlags: u32 {
//         /// 不挂起当前进程，直接返回
//         const WNOHANG = 1 << 0;
//         /// 报告已执行结束的用户进程的状态
//         const WIMTRACED = 1 << 1;
//         /// 报告还未结束的用户进程的状态
//         const WCONTINUED = 1 << 3;
//     }
// }
// /// sys_times 中指定的结构体类型
// #[repr(C)]
// pub struct Tms {
//     /// 进程用户态执行时间，单位为us
//     pub tms_utime: usize,
//     /// 进程内核态执行时间，单位为us
//     pub tms_stime: usize,
//     /// 子进程用户态执行时间和，单位为us
//     pub tms_cutime: usize,
//     /// 子进程内核态执行时间和，单位为us
//     pub tms_cstime: usize,
// }
//
// /// sys_gettimeofday 中指定的类型
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct TimeVal {
//     /// seconds
//     pub sec: usize,
//     /// microseconds
//     pub usec: usize,
// }
//
// impl TimeVal {
//     /// turn the TimeVal to nano seconds
//     pub fn turn_to_nanos(&self) -> usize {
//         self.sec * NANOS_PER_SEC as usize + self.usec * NANOS_PER_MICROS as usize
//     }
//
//     /// create a TimeVal from nano seconds
//     pub fn from_micro(micro: usize) -> Self {
//         TimeVal {
//             sec: micro / (MICROS_PER_SEC as usize),
//             usec: micro % (MICROS_PER_SEC as usize),
//         }
//     }
//
//     /// turn the TimeVal to cpu ticks, which is related to cpu frequency
//     pub fn turn_to_ticks(&self) -> u64 {
//         (self.sec * axconfig::TIMER_FREQUENCY) as u64
//             + nanos_to_ticks((self.usec as u64) * NANOS_PER_MICROS)
//     }
// }
//
// /// sys_gettimer / sys_settimer 指定的类型，用户输入输出计时器
// #[repr(C)]
// #[derive(Debug, Clone, Copy)]
// pub struct ITimerVal {
//     /// The cycle of the timer
//     pub it_interval: TimeVal,
//     /// The remaining time of the timer
//     pub it_value: TimeVal,
// }

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

    //    /// turn the TimeSecs to cpu ticks, which is related to cpu frequency
    //    pub fn get_ticks(&self) -> usize {
    //        self.tv_sec * axconfig::TIMER_FREQUENCY + (nanos_to_ticks(self.tv_nsec as u64) as usize)
    //    }

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
    /// sys_fcntl64 使用的选项
    pub enum Fcntl64Cmd {
        /// 复制这个 fd，相当于 sys_dup
        F_DUPFD = 0,
        /// 获取 cloexec 信息，即 exec 成功时是否删除该 fd
        F_GETFD = 1,
        /// 设置 cloexec 信息，即 exec 成功时删除该 fd
        F_SETFD = 2,
        /// 获取 flags 信息
        F_GETFL = 3,
        /// 设置 flags 信息
        F_SETFL = 4,
        /// 复制 fd，然后设置 cloexec 信息，即 exec 成功时删除该 fd
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

/// readv/writev使用的结构体
#[repr(C)]
pub struct IoVec {
    /// base address of the buffer
    pub base: *mut u8,
    /// length of the buffer
    pub len: usize,
}

bitflags! {
    /// 指定 st_mode 的选项
    pub struct StMode: u32 {
        /// regular file
        const S_IFREG = 1 << 15;
        /// directory
        const S_IFDIR = 1 << 14;
        /// character device
        const S_IFCHR = 1 << 13;
        /// 是否设置 uid/gid/sticky
        //const S_ISUID = 1 << 14;
        //const S_ISGID = 1 << 13;
        //const S_ISVTX = 1 << 12;
        /// user-read permission
        const S_IRUSR = 1 << 8;
        /// user-write permission
        const S_IWUSR = 1 << 7;
        /// user-execute permission
        const S_IXUSR = 1 << 6;
        /// group-read permission
        const S_IRGRP = 1 << 5;
        /// group-write permission
        const S_IWGRP = 1 << 4;
        /// group-execute permission
        const S_IXGRP = 1 << 3;
        /// other-read permission
        const S_IROTH = 1 << 2;
        /// other-write permission
        const S_IWOTH = 1 << 1;
        /// other-execute permission
        const S_IXOTH = 1 << 0;
        /// exited-user-process status
        const WIMTRACED = 1 << 1;
        /// continued-process status
        const WCONTINUED = 1 << 3;
    }
}
/// 文件类型，输入 IFCHR / IFDIR / IFREG 等具体类型，
/// 输出这些类型加上普遍的文件属性后得到的 mode 参数
pub fn normal_file_mode(file_type: StMode) -> StMode {
    file_type | StMode::S_IWUSR | StMode::S_IRUSR | StMode::S_IRGRP | StMode::S_IROTH
}

/// 对 futex 的操作
pub enum FutexFlags {
    /// 检查用户地址 uaddr 处的值。如果不是要求的值则等待 wake
    Wait,
    /// 唤醒最多 val 个在等待 uaddr 位置的线程。
    Wake,
    /// 将等待 uaddr 的线程移动到 uaddr2
    Requeue,
    /// 不支持的操作
    Unsupported,
}

impl FutexFlags {
    /// Create a FutexFlags from a i32 value
    pub fn new(val: i32) -> Self {
        match val & 0x7f {
            0 => FutexFlags::Wait,
            1 => FutexFlags::Wake,
            3 => FutexFlags::Requeue,
            _ => FutexFlags::Unsupported,
        }
    }
}
