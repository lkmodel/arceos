use axlog::info;
use cty::{c_double, c_float, c_int, c_long, c_uint};

use core::f128;
type CLongDouble = f128;

/// 102
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_addtf3(a: CLongDouble, b: CLongDouble) -> CLongDouble {
    info!("[ABI] addtf3");
    a + b
}

/// 106
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_subtf3(a: CLongDouble, b: CLongDouble) -> CLongDouble {
    info!("[ABI] subtf3");
    a - b
}

/// 110
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_multf3(a: CLongDouble, b: CLongDouble) -> CLongDouble {
    info!("[ABI] multf3");
    a * b
}

/// 114
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_divtf3(a: CLongDouble, b: CLongDouble) -> CLongDouble {
    info!("[ABI] divtf3");
    a / b
}

/// 121
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_extendsftf2(a: c_float) -> CLongDouble {
    info!("[ABI] extendsftf2");
    a.into()
}

/// 123
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_extenddftf2(a: c_double) -> CLongDouble {
    info!("[ABI] extenddftf2");
    a.into()
}

/// 126
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_trunctfdf2(a: CLongDouble) -> c_double {
    info!("[ABI] trunctfdf2");
    a as c_double
}

/// 128
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_trunctfsf2(a: CLongDouble) -> c_float {
    info!("[ABI] trunctfsf2");
    a as c_float
}

/// 132
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_fixtfsi(a: CLongDouble) -> c_int {
    info!("[ABI] fixtfsi");
    a as c_int
}

/// 136
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_fixtfdi(a: CLongDouble) -> c_long {
    info!("[ABI] fixtfdi");
    a as c_long
}

/// 144
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_fixunstfsi(a: CLongDouble) -> c_uint {
    info!("[ABI] fixunstfsi");
    a as c_uint
}

/// 156
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_floatsitf(i: c_int) -> CLongDouble {
    info!("[ABI] floatsitf");
    i as CLongDouble
}

/// 160
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_floatditf(i: c_long) -> CLongDouble {
    info!("[ABI] floatditf");
    i as CLongDouble
}

/// 168
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_floatunsitf(i: c_uint) -> CLongDouble {
    info!("[ABI] floatunsitf");
    i as CLongDouble
}

/// 196
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_eqtf2(a: CLongDouble, b: CLongDouble) -> c_int {
    info!("[ABI] eqtf2");
    // 将 c_long_double 转换为 f64 进行比较
    // 检查是否为 NaN
    if a.is_nan() || b.is_nan() {
        return 1; // 如果有一个是 NaN，返回 1
    }

    // 检查是否相等
    if a == b {
        return 0; // 如果相等且都不是 NaN，返回 0
    }

    return 2; // 如果不相等，返回 2
}

/// 199
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_netf2(a: CLongDouble, b: CLongDouble) -> c_int {
    info!("[ABI] netf2");
    // 检查是否为 NaN
    if a.is_nan() || b.is_nan() {
        return 1; // 如果有一个是 NaN，返回 1
    }

    // 检查是否相等
    if a != b {
        return 2; // 如果不相等，返回 2
    }

    return 0; // 如果相等且都不是 NaN，返回 0
}

/// 202
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_getf2(a: CLongDouble, b: CLongDouble) -> c_int {
    info!("[ABI] getf2");
    // 检查是否为 NaN
    if a.is_nan() || b.is_nan() {
        return -1; // 如果有一个是 NaN，返回 -1
    }

    // 检查 a 是否大于或等于 b
    if a >= b {
        return 1; // 如果 a >= b，返回 1（大于或等于零）
    }

    return 0;
}

/// 205
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_lttf2(a: CLongDouble, b: CLongDouble) -> c_int {
    info!("[ABI] lttf2");
    // 检查是否为 NaN
    if a.is_nan() || b.is_nan() {
        return -1; // 如果有一个是 NaN，返回 -1
    }

    // 检查 a 是否严格小于 b
    if a < b {
        return -2; // 如果 a < b，返回 -2（小于零）
    }

    return 0; // 如果 a >= b，返回 0
}

/// 208
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_letf2(a: CLongDouble, b: CLongDouble) -> c_int {
    info!("[ABI] letf2");
    // 检查是否为 NaN
    if a.is_nan() || b.is_nan() {
        return -1; // 如果有一个是 NaN，返回 -1
    }

    // 检查 a 是否小于或等于 b
    if a <= b {
        return -2; // 如果 a <= b，返回 -2（小于或等于零）
    }

    return 0; // 如果 a > b，返回 0
}

/// 211
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_gttf2(a: CLongDouble, b: CLongDouble) -> c_int {
    info!("[ABI] gttf2");
    // 检查是否为 NaN
    if a.is_nan() || b.is_nan() {
        return -1; // 如果有一个是 NaN，返回 -1
    }

    // 检查 a 是否严格小于 b
    if a < b {
        return -2; // 如果 a < b，返回 -2（小于零）
    }

    return 0; // 如果 a >= b，返回 0
}
