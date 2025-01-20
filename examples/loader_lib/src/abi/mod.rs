use axhal::time::monotonic_time;
use axlog::{debug, info};
use axstd::{
    io::stdin,
    print, println,
    process::exit,
    string::{String, ToString},
};
use core::{
    ffi::{CStr, VaList},
    ptr::copy_nonoverlapping,
    slice::from_raw_parts,
};
use cty::{c_char, c_int, size_t};
use printf_compat::output::display;
use thread::{abi_pthread_create, abi_pthread_exit, abi_pthread_join, abi_pthread_self};

mod thread;

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
pub const SYS_TERMINATE: usize = 3;
const SYS_TIMESPEC: usize = 4;
const SYS_VFPRINTF: usize = 5;
const SYS_VSNPRINTF: usize = 6;
const SYS_VSCANF: usize = 7;
const SYS_PTHREAD_CREATE: usize = 8;
const SYS_PTHREAD_JOIN: usize = 9;
const SYS_PTHREAD_EXIT: usize = 10;
const SYS_PTHREAD_SELF: usize = 11;
const SYS_PTHREAD_MUTEX_INIT: usize = 12;
const SYS_PTHREAD_MUTEX_LOCK: usize = 13;
const SYS_PTHREAD_MUTEX_UNLOCK: usize = 14;
const SYS_SLEEP: usize = 15;
const SYS_OUT: usize = 16;

pub static mut ABI_TABLE: [usize; 32] = [0; 32];

pub fn init_abis() {
    register_abi("hello", SYS_HELLO, abi_hello as usize);
    register_abi("putchar", SYS_PUTCHAR, abi_putchar as usize);
    register_abi("exit", SYS_TERMINATE, abi_terminate as usize);
    register_abi("timespec", SYS_TIMESPEC, abi_timespec as usize);
    register_abi("vfprintf", SYS_VFPRINTF, vfprintf as usize);
    register_abi("vsnprintf", SYS_VSNPRINTF, vsnprintf as usize);
    register_abi("vscanf", SYS_VSCANF, vscanf as usize);
    register_abi(
        "pthread_create",
        SYS_PTHREAD_CREATE,
        abi_pthread_create as usize,
    );
    register_abi("pthread_join", SYS_PTHREAD_JOIN, abi_pthread_join as usize);
    register_abi("pthread_exit", SYS_PTHREAD_EXIT, abi_pthread_exit as usize);
    register_abi("pthread_self", SYS_PTHREAD_SELF, abi_pthread_self as usize);
    register_abi("out", SYS_OUT, abi_out as usize);
}

fn register_abi(name: &str, num: usize, handle: usize) {
    unsafe {
        ABI_TABLE[num] = handle;
    }
    info!("[ABI]{}: 0x{:x}", name, handle);
}

/// `SYS_HELLO: 1`
#[no_mangle]
fn abi_hello() {
    print!("\x1b[34m");
    println!("[ABI:Hello] Hello, Apps!");
    print!("\x1b[0m");
}

/// `SYS_PUTCHAR: 2`
#[no_mangle]
fn abi_putchar(c: char) {
    print!("\x1b[34m");
    print!("{c}");
    print!("\x1b[0m");
}

/// `SYS_TERMINATE: 3`
#[no_mangle]
fn abi_terminate() -> ! {
    print!("\x1b[34m");
    println!("Bye");
    print!("\x1b[0m");

    exit(0);
}

#[repr(C)]
#[derive(Debug)]
struct TimeSpec {
    tv_sec: usize,
    tv_nsec: usize,
}

/// `SYS_TIMESPEC: 4`
#[no_mangle]
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
#[no_mangle]
unsafe extern "C" fn vfprintf(str: *const c_char, args: VaList) -> c_int {
    let format = display(str, args);
    print!("\x1b[34m{}\x1b[0m", format);
    format.bytes_written()
}

/// `SYS_VSNPRINTF: 6`
#[no_mangle]
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
    let format = display(str, args);
    let output_string = format.to_string();
    let bytes_written = output_string.len();

    // 限制写入的字节数
    let len_to_copy = bytes_written.min(maxlen - 1); // 保留一个字节用于Null终止符
    copy_nonoverlapping(output_string.as_ptr(), out, len_to_copy);

    // 添加null终止符
    *out.add(len_to_copy) = 0;

    bytes_written as c_int
}

/// `SYS_VSCANF: 7`
#[no_mangle]
unsafe extern "C" fn vscanf(str: *mut c_char, args: VaList) -> c_int {
    println!("DONT USE THIS YET");
    return -1;
    if str.is_null() {
        return -1;
    }

    let mut output: String = String::new();
    let bytes_read = stdin().read_line(&mut output).unwrap_or(0);

    let output_string = output.to_string();

    // 读取
    copy_nonoverlapping(output_string.as_ptr(), str, output_string.len());
    0
}

/// `SYS_OUT: 16`
#[no_mangle]
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
