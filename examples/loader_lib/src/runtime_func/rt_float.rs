use axlog::info;
use cty::{c_double, c_float, c_int, c_long, c_uint};

use core::f128;
type CLongDouble = f128;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_addxf3(a: CLongDouble, b: CLongDouble) -> CLongDouble {
    info!("[ABI] addxf3");
    a + b
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_rt_subxf3(a: CLongDouble, b: CLongDouble) -> CLongDouble {
    info!("[ABI] subxf3");
    a - b
}
