use core::arch::asm;
use core::ffi::c_int;

const FE_INVALID: c_int = 16;
const FE_DIVBYZERO: c_int = 8;
const FE_OVERFLOW: c_int = 4;
const FE_UNDERFLOW: c_int = 2;
const FE_INEXACT: c_int = 1;
const FE_ALL_EXCEPT: c_int = 31;

const FE_TONEAREST: c_int = 0;
const FE_DOWNWARD: c_int = 2;
const FE_UPWARD: c_int = 3;
const FE_TOWARDZERO: c_int = 1;

pub extern "C" fn abi_fetestexcept(r: c_int) -> c_int {
    let o: c_int;
    unsafe {
        asm!(
        "frflags {0}
        and a0, {0}, a0
        ret",
        in(reg) r,
        out("a0") o,
        )
    }
    o
}

pub extern "C" fn abi_fegetround() -> c_int {
    let o: c_int;
    unsafe {
        asm!("
        frrm a0
	    ret",
        out("a0") o
        )
    }
    o
}
extern "C" fn _fesetround(r: c_int) -> c_int {
    let o: c_int;
    unsafe {
        asm!(
        "fsrm {0}, a0
        li a0, 0
        ret",
        in(reg) r,
        out("a0") o
        )
    }
    o
}
pub extern "C" fn abi_fesetround(r: c_int) -> c_int {
    if r != FE_TONEAREST && r != FE_DOWNWARD && r != FE_UPWARD && r != FE_TOWARDZERO {
        -1
    } else {
        _fesetround(r)
    }
}