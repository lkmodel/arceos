use crate::{ABI_TABLE, AbiEntry};
use abi_macro::abi;
use arceos_posix_api::ctypes::{CLOCK_MONOTONIC, CLOCK_REALTIME, time_t, timespec, timeval};
use arceos_posix_api::sys_nanosleep;
use core::ffi::c_int;

#[abi(utimes)]
#[unsafe(no_mangle)]
extern "C" fn abi_utimes() -> c_int {
    0
}

#[abi(time)]
#[unsafe(no_mangle)]
extern "C" fn abi_time(t: *mut time_t) -> time_t {
    if t.is_null() {
        return 0;
    }
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    gettime(CLOCK_REALTIME, &mut ts);
    unsafe {
        t.write(ts.tv_sec);
    }
    ts.tv_sec
}

#[repr(C)]
struct timezone {
    tz_minuteswest: c_int,
    tz_dsttime: c_int,
}

#[abi(gettimeofday)]
#[unsafe(no_mangle)]
extern "C" fn abi_gettimeofday(tv_ptr: *mut timeval, _tz: *mut timezone) -> c_int {
    let mut ts = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if tv_ptr.is_null() {
        return 0;
    }
    let mut tv = unsafe { tv_ptr.read() };
    gettime(CLOCK_REALTIME, &mut ts);
    unsafe {
        tv.tv_sec = ts.tv_sec;
        tv.tv_usec = ts.tv_nsec / 1000;
        tv_ptr.write(tv);
    }
    0
}

fn gettime(clk: u32, ts: &mut timespec) -> c_int {
    let t: timespec = match clk {
        CLOCK_REALTIME => axhal::time::wall_time().into(),
        CLOCK_MONOTONIC => axhal::time::monotonic_time().into(),
        _ => {
            return -1;
        }
    };
    ts.tv_sec = t.tv_sec;
    ts.tv_nsec = t.tv_nsec;
    0
}

#[abi(nanosleep)]
#[unsafe(no_mangle)]
extern "C" fn abi_nanosleep(req: *const timespec, rem: *mut timespec) -> c_int {
    unsafe { sys_nanosleep(req, rem) }
}
