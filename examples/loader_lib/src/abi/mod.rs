mod file;
mod mem;
mod noimpl;
mod syscall;
mod thread;

use axhal::time::monotonic_time;
use axlog::{debug, info};
use axstd::{
    io::stdin,
    print, println,
    process::exit,
    string::{String, ToString},
};

use axtask::init_scheduler;
use core::{
    ffi::{CStr, VaList},
    ptr::copy_nonoverlapping,
    slice::from_raw_parts,
};
use cty::{c_char, c_int, size_t};
use mem::*;
use noimpl::abi_noimpl;
use printf_compat::output::display;
use syscall::*;
use thread::*;

// `0-10提供ArceOS相关ABI调用`
const ABI_NOIMPL: usize = 0;
const ABI_INIT_SCHEDULER: usize = 1;
pub const ABI_TERMINATE: usize = 2;
// `stdio`
const ABI_PUTCHAR: usize = 10;
const ABI_TIMESPEC: usize = 11;
const ABI_VFPRINTF: usize = 12;
const ABI_VSNPRINTF: usize = 13;
const ABI_VSCANF: usize = 14;
const ABI_OUT: usize = 15;
// `pthread`
const ABI_PTHREAD_CREATE: usize = 20;
const ABI_PTHREAD_JOIN: usize = 21;
const ABI_PTHREAD_EXIT: usize = 22;
const ABI_PTHREAD_SELF: usize = 23;
const ABI_PTHREAD_MUTEX_INIT: usize = 24;
const ABI_PTHREAD_MUTEX_LOCK: usize = 25;
const ABI_PTHREAD_MUTEX_UNLOCK: usize = 26;
const ABI_PTHREAD_MUTEX_DESTORY: usize = 27;
// `file`
const ABI_OPEN: usize = 30;
const ABI_LSEEK: usize = 31;
const ABI_STAT: usize = 32;
const ABI_FSTAT: usize = 33;
const ABI_LSTAT: usize = 34;
const ABI_GETCWD: usize = 35;
const ABI_RENAME: usize = 36;
// `malloc`
const ABI_MALLOC: usize = 40;
const ABI_CALLOC: usize = 41;
const ABI_REALLOC: usize = 42;
const ABI_FREE: usize = 43;
// `unistd`
const ABI_SLEEP: usize = 50;
// `syscall`
const ABI_SYSCALL0: usize = 60;
const ABI_SYSCALL1: usize = 61;
const ABI_SYSCALL2: usize = 62;
const ABI_SYSCALL3: usize = 63;
const ABI_SYSCALL4: usize = 64;
const ABI_SYSCALL5: usize = 65;
const ABI_SYSCALL6: usize = 66;

/// 当访问到没有被绑定的`ABI`时，将会使用`ABI_NOIMPL`
pub static mut ABI_TABLE: [usize; 100] = [0; 100];

pub fn init_abis() {
    register_abi("noimpl", ABI_NOIMPL, abi_noimpl as usize);
    register_abi("init", ABI_INIT_SCHEDULER, abi_init_scheduler as usize);
    register_abi("exit", ABI_TERMINATE, abi_terminate as usize);

    register_abi("putchar", ABI_PUTCHAR, abi_putchar as usize);
    register_abi("timespec", ABI_TIMESPEC, abi_timespec as usize);
    register_abi("vfprintf", ABI_VFPRINTF, vfprintf as usize);
    register_abi("vsnprintf", ABI_VSNPRINTF, vsnprintf as usize);
    register_abi("vscanf", ABI_VSCANF, vscanf as usize);
    register_abi("out", ABI_OUT, abi_out as usize);

    register_abi(
        "pthread_create",
        ABI_PTHREAD_CREATE,
        abi_pthread_create as usize,
    );
    register_abi("pthread_join", ABI_PTHREAD_JOIN, abi_pthread_join as usize);
    register_abi("pthread_exit", ABI_PTHREAD_EXIT, abi_pthread_exit as usize);
    register_abi("pthread_self", ABI_PTHREAD_SELF, abi_pthread_self as usize);
    register_abi(
        "pthread_mutex_init",
        ABI_PTHREAD_MUTEX_INIT,
        abi_pthread_mutex_init as usize,
    );
    register_abi(
        "pthread_mutex_lock",
        ABI_PTHREAD_MUTEX_LOCK,
        abi_pthread_mutex_lock as usize,
    );
    register_abi(
        "pthread_mutex_unlock",
        ABI_PTHREAD_MUTEX_UNLOCK,
        abi_pthread_mutex_unlock as usize,
    );
    register_abi(
        "pthread_mutex_destroy",
        ABI_PTHREAD_MUTEX_DESTORY,
        abi_pthread_mutex_destroy as usize,
    );

    register_abi("malloc", ABI_MALLOC, abi_malloc as usize);
    register_abi("calloc", ABI_CALLOC, abi_calloc as usize);
    register_abi("realloc", ABI_REALLOC, abi_realloc as usize);
    register_abi("free", ABI_FREE, abi_free as usize);

    register_abi("sleep", ABI_SLEEP, abi_sleep as usize);

    register_abi("syscall0", ABI_SYSCALL0, abi_syscall0 as usize);
    register_abi("syscall1", ABI_SYSCALL1, abi_syscall1 as usize);
    register_abi("syscall2", ABI_SYSCALL2, abi_syscall2 as usize);
    register_abi("syscall3", ABI_SYSCALL3, abi_syscall3 as usize);
    register_abi("syscall4", ABI_SYSCALL4, abi_syscall4 as usize);
    register_abi("syscall5", ABI_SYSCALL5, abi_syscall5 as usize);
    register_abi("syscall6", ABI_SYSCALL6, abi_syscall6 as usize);
}

fn register_abi(name: &str, num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
    info!("[ABI]{}: 0x{:x}", name, handle);
}

pub fn abi_init_scheduler() {
    init_scheduler();
}

/// `SYS_PUTCHAR: 2`
#[unsafe(no_mangle)]
fn abi_putchar(c: char) {
    print!("{c}");
}

/// `SYS_TERMINATE: 3`
#[unsafe(no_mangle)]
fn abi_terminate() -> ! {
    exit(0);
}

#[repr(C)]
#[derive(Debug)]
struct TimeSpec {
    tv_sec: usize,
    tv_nsec: usize,
}

/// `SYS_TIMESPEC: 4`
#[unsafe(no_mangle)]
fn abi_timespec(ts: *mut TimeSpec) {
    unsafe {
        let ts = &mut *ts;
        let now = monotonic_time();
        ts.tv_nsec = now.as_nanos() as usize;
        ts.tv_sec = now.as_secs() as usize;
        debug!("{:?}", ts);
    }
}

/// `SYS_VFPRINTF: 5`
#[unsafe(no_mangle)]
unsafe extern "C" fn vfprintf(str: *const c_char, args: VaList) -> c_int {
    unimplemented!();
    //    unsafe {
    //        let format = display(str, args);
    //        print!("{}", format);
    //        format.bytes_written()
    //    }
}

/// `SYS_VSNPRINTF: 6`
#[unsafe(no_mangle)]
unsafe extern "C" fn vsnprintf(
    out: *mut c_char,
    maxlen: size_t,
    str: *const c_char,
    args: VaList,
) -> c_int {
    // 检查str是否为null
    if str.is_null() {
        return -1; // 返回一个错误代码
    }
    // 创建格式化字符串
    let format = unsafe { display(str, args) };
    let output_string = format.to_string();
    let bytes_written = output_string.len();

    // 限制写入的字节数
    let len_to_copy = bytes_written.min(maxlen - 1); // 保留一个字节用于Null终止符
    unsafe {
        copy_nonoverlapping(output_string.as_ptr(), out as *mut u8, len_to_copy);
    }

    // 添加null终止符
    unsafe {
        *out.add(len_to_copy) = 0;
    }

    bytes_written as c_int
}

/// `SYS_VSCANF: 7`
#[unsafe(no_mangle)]
unsafe extern "C" fn vscanf(_str: *mut c_char, _args: VaList) -> c_int {
    println!("DONT USE THIS YET");
    return -1;
    // ```
    //     if str.is_null() {
    //         return -1;
    //     }
    //
    //     let mut output: String = String::new();
    //     let bytes_read = stdin().read_line(&mut output).unwrap_or(0);
    //
    //     let output_string = output.to_string();
    //
    //     // 读取
    //     copy_nonoverlapping(output_string.as_ptr(), str, output_string.len());
    //     0
}

/// `SYS_OUT: 16`
#[unsafe(no_mangle)]
extern "C" fn abi_out(s: *const c_char, l: size_t) {
    unsafe {
        let bytes = from_raw_parts(s as *const u8, l);

        match CStr::from_bytes_with_nul(bytes) {
            Ok(c_str) => {
                let str_slice = c_str.to_str().unwrap();
                axhal::console::write_bytes(str_slice.as_bytes());
            }
            Err(_) => println!("Failed to convert to &str"),
        }
    }
}
