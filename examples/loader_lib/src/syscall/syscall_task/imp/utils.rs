use crate::{
    linux_env::linux_fs::futex::{FUTEX_WAIT_TASK, FutexKey, WAIT_FOR_FUTEX, get_futex_key},
    syscall::{FutexFlags, SyscallError, SyscallResult, TimeSecs, TimeVal},
};
use alloc::collections::VecDeque;
use axhal::{
    mem::VirtAddr,
    time::{current_ticks, monotonic_time_nanos},
};
use axtask::{CurrentTask, current, yield_now};
use core::time::Duration;

/// 返回值为当前经过的时钟中断数
/// # Arguments
/// * `tms` - *mut Tms
// pub fn syscall_time(args: [usize; 6]) -> SyscallResult {
//     let tms = args[0] as *mut Tms;
//     let (_, utime_us, _, stime_us) = time_stat_output();
//     unsafe {
//         *tms = Tms {
//             tms_utime: utime_us,
//             tms_stime: stime_us,
//             tms_cutime: utime_us,
//             tms_cstime: stime_us,
//         }
//     }
//     Ok(nanos_to_ticks(current_time_nanos()) as isize)
// }

/// 获取当前系统时间并且存储在给定结构体中
/// # Arguments
/// * `ts` - *mut TimeVal
pub fn syscall_get_time_of_day(args: [usize; 6]) -> SyscallResult {
    let ts = args[0] as *mut TimeVal;

    let current_us = monotonic_time_nanos() as usize / 1000;
    unsafe {
        *ts = TimeVal {
            sec: current_us / 1_000_000,
            usec: current_us % 1_000_000,
        }
    }
    Ok(0)
}

/// 用于获取当前系统时间并且存储在对应的结构体中
/// # Arguments
/// * `clock_id` - usize
/// * `ts` - *mut TimeSecs
pub fn syscall_clock_get_time(args: [usize; 6]) -> SyscallResult {
    let _clock_id = args[0];
    let ts = args[1] as *mut TimeSecs;
    unsafe {
        (*ts) = TimeSecs::now();
    }
    Ok(0)
}

/// 当前任务进入睡眠，req指定了睡眠的时间
/// rem存储当睡眠完成时，真实睡眠时间和预期睡眠时间之间的差值
/// # Arguments
/// * `req` - *const TimeSecs
/// * `rem` - *mut TimeSecs
pub fn syscall_sleep(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //    let req = args[0] as *const TimeSecs;
    //    let rem = args[1] as *mut TimeSecs;
    //    let req_time = unsafe { *req };
    //    let start_to_sleep = current_time();
    //    // info!("sleep: req_time = {:?}", req_time);
    //    let dur = Duration::new(req_time.tv_sec as u64, req_time.tv_nsec as u32);
    //    sleep_now_task(dur);
    //    // 若被唤醒时时间小于请求时间，则将剩余时间写入rem
    //    let sleep_time = current_time() - start_to_sleep;
    //    if rem as usize != 0 {
    //        if sleep_time < dur {
    //            let delta = (dur - sleep_time).as_nanos() as usize;
    //            unsafe {
    //                *rem = TimeSecs {
    //                    tv_sec: delta / 1_000_000_000,
    //                    tv_nsec: delta % 1_000_000_000,
    //                }
    //            };
    //        } else {
    //            unsafe {
    //                *rem = TimeSecs {
    //                    tv_sec: 0,
    //                    tv_nsec: 0,
    //                }
    //            };
    //        }
    //    }
    //    #[cfg(feature = "signal")]
    //    if current_process().have_signals().is_some() {
    //        return Err(SyscallError::EINTR);
    //    }
    //    Ok(0)
}

/// # 指定任务进行睡眠
///
/// # Arguments
/// * id: usize,指定使用的时钟ID,对应结构体为ClockId
///
/// * flags: usize,指定是使用相对时间还是绝对时间
///
/// * request: *const TimeSecs指定睡眠的时间,根据flags划分为相对时间或者绝对时间
///
/// * remain: *mut TimeSecs存储剩余睡眠时间。当任务提前醒来时,如果flags不为绝对时间,且remain不为空,则将剩余存储时间存进remain所指向地址。
///
/// 若睡眠被信号处理打断或者遇到未知错误，则返回对应错误码
pub fn syscall_clock_nanosleep(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
    //     let id = args[0];
    //     let flags = args[1];
    //     let request = args[2] as *const TimeSecs;
    //     let remain = args[3] as *mut TimeSecs;
    //     const TIMER_ABSTIME: usize = 1;
    //     let id = if let Ok(opt) = ClockId::try_from(id) {
    //         opt
    //     } else {
    //         return Err(SyscallError::EINVAL);
    //     };
    //
    //     if id != ClockId::CLOCK_MONOTONIC {
    //         // 暂时不支持其他类型
    //         return Err(SyscallError::EINVAL);
    //     }
    //
    //     let process = current_process();
    //
    //     if process.manual_alloc_type_for_lazy(request).is_err() {
    //         return Err(SyscallError::EFAULT);
    //     }
    //     let request_time = unsafe { *request };
    //     let request_time = Duration::new(request_time.tv_sec as u64, request_time.tv_nsec as u32);
    //     let deadline = if flags != TIMER_ABSTIME {
    //         current_time() + request_time
    //     } else {
    //         if request_time < current_time() {
    //             return Ok(0);
    //         }
    //         request_time
    //     };
    //
    //     axtask::sleep_until(deadline);
    //
    //     let current_time = current_time();
    //     if current_time < deadline && !remain.is_null() {
    //         if process.manual_alloc_type_for_lazy(remain).is_err() {
    //             return Err(SyscallError::EFAULT);
    //         } else {
    //             let delta = (deadline - current_time).as_nanos() as usize;
    //             unsafe {
    //                 *remain = TimeSecs {
    //                     tv_sec: delta / 1_000_000_000,
    //                     tv_nsec: delta % 1_000_000_000,
    //                 }
    //             };
    //             return Err(SyscallError::EINTR);
    //         }
    //     }
    //     Ok(0)
}
