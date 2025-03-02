use axlog::info;
use cty::{c_int, c_uint, c_ulong, c_ulonglong, int32_t, int64_t};

/// # 271
/// These functions return the number of leading 0-bits in a, starting at the most significant bit position.
/// If `a` is zero, the result is undefined.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_clzsi2(a: c_int) -> c_uint {
    info!("[ABI] clzsi2");
    a.leading_zeros()
}
/// # 272
/// These functions return the number of leading 0-bits in a, starting at the most significant bit position.
/// If `a` is zero, the result is undefined.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_clzdi2(a: c_ulong) -> c_uint {
    info!("[ABI] clzdi2");
    a.leading_zeros()
}
/// # 273
/// These functions return the number of leading 0-bits in a, starting at the most significant bit position.
/// If `a` is zero, the result is undefined.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_clzti2(a: c_ulonglong) -> c_uint {
    info!("[ABI] clzti2");
    a.leading_zeros()
}

/// 285
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_bswapsi2(a: int32_t) -> int32_t {
    info!("[ABI] bswapsi2");
    ((a >> 24) & 0xff) | ((a >> 8) & 0xff00) | ((a & 0xff00) << 8) | ((a & 0xff) << 24)
}

/// 286
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_bswapdi2(a: int64_t) -> int64_t {
    info!("[ABI] bswapdi2");
    ((a >> 56) & 0xff)
        | ((a >> 40) & 0xff00)
        | ((a >> 24) & 0xff0000)
        | ((a >> 8) & 0xff000000)
        | ((a & 0xff000000) << 8)
        | ((a & 0xff0000) << 24)
        | ((a & 0xff00) << 40)
        | ((a & 0xff) << 56)
}
