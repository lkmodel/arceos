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
    /// sys_renameat2 用到的选项
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
