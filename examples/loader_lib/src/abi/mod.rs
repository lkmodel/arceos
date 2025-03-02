mod file;
mod mem;
mod noimpl;
mod syscall;
mod thread;

use axhal::time::monotonic_time;
use axlog::{debug, info};
use axstd::{println, process::exit};

use crate::runtime_func::{
    rt_float::{
        abi_rt_adddf3, abi_rt_addsf3, abi_rt_addtf3, abi_rt_addxf3, abi_rt_divtf3, abi_rt_eqtf2,
        abi_rt_extenddftf2, abi_rt_extendsftf2, abi_rt_fixtfdi, abi_rt_fixtfsi, abi_rt_fixunstfsi,
        abi_rt_floatditf, abi_rt_floatsitf, abi_rt_floatunsitf, abi_rt_getf2, abi_rt_gttf2,
        abi_rt_letf2, abi_rt_lttf2, abi_rt_multf3, abi_rt_netf2, abi_rt_subdf3, abi_rt_subsf3,
        abi_rt_subtf3, abi_rt_subxf3, abi_rt_trunctfdf2, abi_rt_trunctfsf2,
    },
    rt_integer::{abi_rt_bswapdi2, abi_rt_bswapsi2, abi_rt_clzdi2, abi_rt_clzsi2, abi_rt_clzti2},
};
use axtask::init_scheduler;
use core::{
    ffi::{CStr, VaList},
    slice::from_raw_parts,
};
use cty::{c_char, c_int, size_t};
use mem::*;
use noimpl::abi_noimpl;
use syscall::*;
use thread::*;

// `0-10提供ArceOS相关ABI调用`
const ABI_NOIMPL: usize = 0;
const ABI_INIT_SCHEDULER: usize = 1;
pub const ABI_TERMINATE: usize = 2;
// `stdio`
const ABI_TIMESPEC: usize = 11;
// `pthread`
const ABI_PTHREAD_CREATE: usize = 20;
const ABI_PTHREAD_JOIN: usize = 21;
const ABI_PTHREAD_EXIT: usize = 22;
const ABI_PTHREAD_SELF: usize = 23;
const ABI_PTHREAD_MUTEX_INIT: usize = 24;
const ABI_PTHREAD_MUTEX_LOCK: usize = 25;
const ABI_PTHREAD_MUTEX_UNLOCK: usize = 26;
const ABI_PTHREAD_MUTEX_DESTORY: usize = 27;
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
// `rt_float`的实现
// Arithmetic functions[100, 119]
const ABI_RT_ADDSF3: usize = 100;
const ABI_RT_ADDDF3: usize = 101;
const ABI_RT_ADDTF3: usize = 102;
const ABI_RT_ADDXF3: usize = 103;

const ABI_RT_SUBSF3: usize = 104;
const ABI_RT_SUBDF3: usize = 105;
const ABI_RT_SUBTF3: usize = 106;
const ABI_RT_SUBXF3: usize = 107;

const _ABI_RT_MULSF3: usize = 108;
const _ABI_RT_MULDF3: usize = 109;
const ABI_RT_MULTF3: usize = 110;
const _ABI_RT_MULXF3: usize = 111;

const _ABI_RT_DIVSF3: usize = 112;
const _ABI_RT_DIVDF3: usize = 113;
const ABI_RT_DIVTF3: usize = 114;
const _ABI_RT_DIVXF3: usize = 115;

const _ABI_RT_NEGSF2: usize = 116;
const _ABI_RT_NEGDF2: usize = 117;
const _ABI_RT_NEGTF2: usize = 118;
const _ABI_RT_NEGXF2: usize = 119;
// Conversion functions[120, 187]
const _ABI_RT_EXTENDSFDF2: usize = 120;
const ABI_RT_EXTENDSFTF2: usize = 121;
const _ABI_RT_EXTENDSFXF2: usize = 122;
const ABI_RT_EXTENDDFTF2: usize = 123;
const _ABI_RT_EXTENDDFXF2: usize = 124;

const _ABI_RT_TRUNCXFDF2: usize = 125;
const ABI_RT_TRUNCTFDF2: usize = 126;
const _ABI_RT_TRUNCXFSF2: usize = 127;
const ABI_RT_TRUNCTFSF2: usize = 128;
const _ABI_RT_TRUNCDFSF2: usize = 129;

const _ABI_RT_FIXSFSI: usize = 130;
const _ABI_RT_FIXDFSI: usize = 131;
const ABI_RT_FIXTFSI: usize = 132;
const _ABI_RT_FIXXFSI: usize = 133;

const _ABI_RT_FIXSFDI: usize = 134;
const _ABI_RT_FIXDFDI: usize = 135;
const ABI_RT_FIXTFDI: usize = 136;
const _ABI_RT_FIXXFDI: usize = 137;

const _ABI_RT_FIXSFTI: usize = 138;
const _ABI_RT_FIXDFTI: usize = 139;
const _ABI_RT_FIXTFTI: usize = 140;
const _ABI_RT_FIXXFTI: usize = 141;

const _ABI_RT_FIXUNSSFSI: usize = 142;
const _ABI_RT_FIXUNSDFSI: usize = 143;
const ABI_RT_FIXUNSTFSI: usize = 144;
const _ABI_RT_FIXUNSXFSI: usize = 145;

const _ABI_RT_FIXUNSSFDI: usize = 146;
const _ABI_RT_FIXUNSDFDI: usize = 147;
const _ABI_RT_FIXUNSTFDI: usize = 148;
const _ABI_RT_FIXUNSXFDI: usize = 149;

const _ABI_RT_FIXUNSSFTI: usize = 150;
const _ABI_RT_FIXUNSDFTI: usize = 151;
const _ABI_RT_FIXUNSTFTI: usize = 152;
const _ABI_RT_FIXUNSXFTI: usize = 153;

const _ABI_RT_FLOATSISF: usize = 154;
const _ABI_RT_FLOATSIDF: usize = 155;
const ABI_RT_FLOATSITF: usize = 156;
const _ABI_RT_FLOATSIXF: usize = 157;

const _ABI_RT_FLOATDISF: usize = 158;
const _ABI_RT_FLOATDIDF: usize = 159;
const ABI_RT_FLOATDITF: usize = 160;
const _ABI_RT_FLOATDIXF: usize = 161;

const _ABI_RT_FLOATTISF: usize = 162;
const _ABI_RT_FLOATTIDF: usize = 163;
const _ABI_RT_FLOATTITF: usize = 164;
const _ABI_RT_FLOATTIXF: usize = 165;

const _ABI_RT_FLOATUNSISF: usize = 166;
const _ABI_RT_FLOATUNSIDF: usize = 167;
const ABI_RT_FLOATUNSITF: usize = 168;
const _ABI_RT_FLOATUNSIXF: usize = 169;

const _ABI_RT_FLOATUNDISF: usize = 170;
const _ABI_RT_FLOATUNDIDF: usize = 171;
const _ABI_RT_FLOATUNDITF: usize = 172;
const _ABI_RT_FLOATUNDIXF: usize = 173;

const _ABI_RTFLOATUNTISF: usize = 174;
const _ABI_RTFLOATUNTIDF: usize = 175;
const _ABI_RTFLOATUNTITF: usize = 176;
const _ABI_RTFLOATUNTIXF: usize = 177;

const _ABI_RT_FIXSFBITINT: usize = 178;
const _ABI_RT_FIXDFBITINT: usize = 179;
const _ABI_RT_FIXXFBITINT: usize = 180;
const _ABI_RT_FIXTFBITINT: usize = 181;

const _ABI_RT_FLOATBITINTSF: usize = 182;
const _ABI_RT_FLOATBITINTDF: usize = 183;
const _ABI_RT_FLOATBITINTXF: usize = 184;
const _ABI_RT_FLOATBITINTTF: usize = 185;
const _ABI_RT_FLOATBITINTHF: usize = 186;
const _ABI_RT_FLOATBITINTBF: usize = 187;
// 3.2.3 Comparison functions [180, ]
const _ABI_RT_CMPSF2: usize = 188;
const _ABI_RT_CMPDF2: usize = 189;
const _ABI_RT_CMPTF2: usize = 190;

const _ABI_RT_UNORDSF2: usize = 191;
const _ABI_RT_UNORDDF2: usize = 192;
const _ABI_RT_UNORDTF2: usize = 193;

const _ABI_RT_EQSF2: usize = 194;
const _ABI_RT_EQDF2: usize = 195;
const ABI_RT_EQTF2: usize = 196;

const _ABI_RT_NESF2: usize = 197;
const _ABI_RT_NEDF2: usize = 198;
const ABI_RT_NETF2: usize = 199;

const _ABI_RT_GESF2: usize = 200;
const _ABI_RT_GEDF2: usize = 201;
const ABI_RT_GETF2: usize = 202;

const _ABI_RT_LTSF2: usize = 203;
const _ABI_RT_LTDF2: usize = 204;
const ABI_RT_LTTF2: usize = 205;

const _ABI_RT_LESF2: usize = 206;
const _ABI_RT_LEDF2: usize = 207;
const ABI_RT_LETF2: usize = 208;

const _ABI_RT_GTSF2: usize = 209;
const _ABI_RT_GTDF2: usize = 210;
const ABI_RT_GTTF2: usize = 211;

const _ABI_RT_POWISF2: usize = 212;
const _ABI_RT_POWIDF2: usize = 213;
const _ABI_RT_POWITF2: usize = 214;
const _ABI_RT_POWIXF2: usize = 215;

const _ABI_RT_MULSC3: usize = 216;
const _ABI_RT_MULDC3: usize = 217;
const _ABI_RT_MULTC3: usize = 218;
const _ABI_RT_MULXC3: usize = 219;

const _ABI_RT_DIVSC3: usize = 220;
const _ABI_RT_DIVDC3: usize = 221;
const _ABI_RT_DIVTC3: usize = 222;
const _ABI_RT_DIVXC3: usize = 223;

// `rt_integer`的实现
// Arithmetic functions[230, 256]
const _ABI_RT_ASHLSI3: usize = 230;
const _ABI_RT_ASHLDI3: usize = 231;
const _ABI_RT_ASHLTI3: usize = 231;

const _ABI_RT_ASHRSI3: usize = 232;
const _ABI_RT_ASHRDI3: usize = 233;
const _ABI_RT_ASHRTI3: usize = 234;

const _ABI_RT_DIVSI3: usize = 235;
const _ABI_RT_DIVDI3: usize = 236;
const _ABI_RT_DIVTI3: usize = 237;

const _ABI_RT_LSHRSI3: usize = 238;
const _ABI_RT_LSHRDI3: usize = 239;
const _ABI_RT_LSHRTI3: usize = 240;

const _ABI_RT_MODSI3: usize = 241;
const _ABI_RT_MODDI3: usize = 242;
const _ABI_RT_MODTI3: usize = 243;

const _ABI_RT_MULSI3: usize = 244;
const _ABI_RT_MULDI3: usize = 245;
const _ABI_RT_MULTI3: usize = 246;

const _ABI_RT_NEGDI2: usize = 247;
const _ABI_RT_NEGTI2: usize = 248;

const _ABI_RT_UDIVSI3: usize = 249;
const _ABI_RT_UDIVDI3: usize = 250;
const _ABI_RT_UDIVTI3: usize = 251;

const _ABI_RT_UDIVMODDI4: usize = 252;
const _ABI_RT_UDIVMODTI4: usize = 253;

const _ABI_RT_UMODSI3: usize = 254;
const _ABI_RT_UMODDI3: usize = 255;
const _ABI_RT_UMODTI3: usize = 256;

// Comparison functions[257, 260]
const _ABI_RT_CMPDI2: usize = 257;
const _ABI_RT_CMPTI2: usize = 258;

const _ABI_RT_UCMPDI2: usize = 259;
const _ABI_RT_UCMPTI2: usize = 260;

// Trapping Arithmetic functions[261, 270]
const _ABI_RT_ABSVSI2: usize = 261;
const _ABI_RT_ABSVDI2: usize = 262;

const _ABI_RT_ADDVSI3: usize = 263;
const _ABI_RT_ADDVDI3: usize = 264;

const _ABI_RT_MULVSI3: usize = 265;
const _ABI_RT_MULVDI3: usize = 266;

const _ABI_RT_NEGVSI2: usize = 267;
const _ABI_RT_NEGVDI2: usize = 268;

const _ABI_RT_SUBVSI3: usize = 269;
const _ABI_RT_SUBVDI3: usize = 270;

// Bit operations functions[271, 286]
const ABI_RT_CLZSI2: usize = 271;
const ABI_RT_CLZDI2: usize = 272;
const ABI_RT_CLZTI2: usize = 273;

const _ABI_RT_CTZSI2: usize = 274;
const _ABI_RT_CTZDI2: usize = 275;
const _ABI_RT_CTZTI2: usize = 276;

const _ABI_RT_FFSDI2: usize = 277;
const _ABI_RT_FFSTI2: usize = 278;

const _ABI_RT_PARITYSI2: usize = 279;
const _ABI_RT_PARITYDI2: usize = 280;
const _ABI_RT_PARITYTI2: usize = 281;

const _ABI_RT_POPCOUNTSI2: usize = 282;
const _ABI_RT_POPCOUNTDI2: usize = 283;
const _ABI_RT_POPCOUNTTI2: usize = 284;

const ABI_RT_BSWAPSI2: usize = 285;
const ABI_RT_BSWAPDI2: usize = 286;

// Bit-precise integer arithmetic functions[287, 288]
const _ABI_RT_MULBITINT3: usize = 287;

const _ABI_RT_DIVMODBITINT4: usize = 288;

/// 当访问到没有被绑定的`ABI`时，将会使用`ABI_NOIMPL`
pub static mut ABI_TABLE: [usize; 300] = [0; 300];

pub fn init_abis() {
    register_abi("noimpl", ABI_NOIMPL, abi_noimpl as usize);
    register_abi("init", ABI_INIT_SCHEDULER, abi_init_scheduler as usize);
    register_abi("exit", ABI_TERMINATE, abi_terminate as usize);

    // register_abi("putchar", ABI_PUTCHAR, abi_putchar as usize);
    register_abi("timespec", ABI_TIMESPEC, abi_timespec as usize);
    // register_abi("vfprintf", ABI_VFPRINTF, vfprintf as usize);
    // register_abi("vsnprintf", ABI_VSNPRINTF, vsnprintf as usize);
    // register_abi("vscanf", ABI_VSCANF, vscanf as usize);
    // register_abi("out", ABI_OUT, abi_out as usize);

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

    // `rt_float`的实现
    register_abi("rt_addsf3", ABI_RT_ADDSF3, abi_rt_addsf3 as usize);
    register_abi("rt_adddf3", ABI_RT_ADDDF3, abi_rt_adddf3 as usize);
    register_abi("rt_addtf3", ABI_RT_ADDTF3, abi_rt_addtf3 as usize);
    register_abi("rt_addxf3", ABI_RT_ADDXF3, abi_rt_addxf3 as usize);

    register_abi("rt_subsf3", ABI_RT_SUBSF3, abi_rt_subsf3 as usize);
    register_abi("rt_subdf3", ABI_RT_SUBDF3, abi_rt_subdf3 as usize);
    register_abi("rt_subtf3", ABI_RT_SUBTF3, abi_rt_subtf3 as usize);
    register_abi("rt_subxf3", ABI_RT_SUBXF3, abi_rt_subxf3 as usize);

    register_abi("rt_multf3", ABI_RT_MULTF3, abi_rt_multf3 as usize);
    register_abi("rt_divtf3", ABI_RT_DIVTF3, abi_rt_divtf3 as usize);
    register_abi(
        "rt_extendsftf2",
        ABI_RT_EXTENDSFTF2,
        abi_rt_extendsftf2 as usize,
    );
    register_abi(
        "rt_extenddftf2",
        ABI_RT_EXTENDDFTF2,
        abi_rt_extenddftf2 as usize,
    );
    register_abi(
        "rt_trunctfdf2",
        ABI_RT_TRUNCTFDF2,
        abi_rt_trunctfdf2 as usize,
    );
    register_abi(
        "rt_trunctfsf2",
        ABI_RT_TRUNCTFSF2,
        abi_rt_trunctfsf2 as usize,
    );
    register_abi("rt_fixtfsi", ABI_RT_FIXTFSI, abi_rt_fixtfsi as usize);
    register_abi("rt_fixtfdi", ABI_RT_FIXTFDI, abi_rt_fixtfdi as usize);
    register_abi(
        "rt_fixunstfsi",
        ABI_RT_FIXUNSTFSI,
        abi_rt_fixunstfsi as usize,
    );
    register_abi("rt_floatsitf", ABI_RT_FLOATSITF, abi_rt_floatsitf as usize);
    register_abi("rt_floatditf", ABI_RT_FLOATDITF, abi_rt_floatditf as usize);
    register_abi(
        "rt_floatunsitf",
        ABI_RT_FLOATUNSITF,
        abi_rt_floatunsitf as usize,
    );
    register_abi("rt_eqtf2", ABI_RT_EQTF2, abi_rt_eqtf2 as usize);
    register_abi("rt_netf2", ABI_RT_NETF2, abi_rt_netf2 as usize);
    register_abi("rt_getf2", ABI_RT_GETF2, abi_rt_getf2 as usize);
    register_abi("rt_lttf2", ABI_RT_LTTF2, abi_rt_lttf2 as usize);
    register_abi("rt_letf2", ABI_RT_LETF2, abi_rt_letf2 as usize);
    register_abi("rt_gttf2", ABI_RT_GTTF2, abi_rt_gttf2 as usize);

    register_abi("rt_clzsi2", ABI_RT_CLZSI2, abi_rt_clzsi2 as usize);
    register_abi("rt_clzdi2", ABI_RT_CLZDI2, abi_rt_clzdi2 as usize);
    register_abi("rt_clzti2", ABI_RT_CLZTI2, abi_rt_clzti2 as usize);

    register_abi("rt_bswapsi2", ABI_RT_BSWAPSI2, abi_rt_bswapsi2 as usize);
    register_abi("rt_bswapdi2", ABI_RT_BSWAPDI2, abi_rt_bswapdi2 as usize);
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
fn abi_putchar(_c: char) {
    unimplemented!();
    // print!("{c}");
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
unsafe extern "C" fn vfprintf(_str: *const c_char, _args: VaList) -> c_int {
    unimplemented!();
}

/// `SYS_VSNPRINTF: 6`
#[unsafe(no_mangle)]
unsafe extern "C" fn vsnprintf(
    _out: *mut c_char,
    _maxlen: size_t,
    _str: *const c_char,
    _args: VaList,
) -> c_int {
    unimplemented!();
    //    // 检查str是否为null
    //    if str.is_null() {
    //        return -1; // 返回一个错误代码
    //    }
    //    // 创建格式化字符串
    //    let format = unsafe { display(str, args) };
    //    let output_string = format.to_string();
    //    let bytes_written = output_string.len();
    //
    //    // 限制写入的字节数
    //    let len_to_copy = bytes_written.min(maxlen - 1); // 保留一个字节用于Null终止符
    //    unsafe {
    //        copy_nonoverlapping(output_string.as_ptr(), out as *mut u8, len_to_copy);
    //    }
    //
    //    // 添加null终止符
    //    unsafe {
    //        *out.add(len_to_copy) = 0;
    //    }
    //
    //    bytes_written as c_int
}

/// `SYS_VSCANF: 7`
#[unsafe(no_mangle)]
unsafe extern "C" fn vscanf(_str: *mut c_char, _args: VaList) -> c_int {
    unimplemented!();
    //    println!("DONT USE THIS YET");
    //    return -1;
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
