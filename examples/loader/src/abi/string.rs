use crate::{ABI_TABLE, AbiEntry};
use abi_macro::abi;
use alloc::ffi::CString;
use axerrno::LinuxError;
use core::ffi::{CStr, c_char, c_int, c_size_t, c_void};
use core::str::FromStr;

#[abi(strlen)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_strlen(s: *const c_char) -> usize {
    let mut ptr = s;
    while !ptr.is_null() && unsafe { *ptr != 0 } {
        ptr = unsafe { ptr.add(1) };
    }
    (ptr as usize - s as usize) as usize
}

#[abi(strerror)]
#[unsafe(no_mangle)]
extern "C" fn abi_strerror(e: c_int) -> *const c_char {
    let err_str = if e == 0 {
        "Success"
    } else {
        LinuxError::try_from(e)
            .map(|e| e.as_str())
            .unwrap_or("Unknown error")
    };
    CString::from_str(err_str).unwrap().as_ptr()
}

#[abi(memmove)]
#[unsafe(no_mangle)]
extern "C" fn abi_memmove(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    unsafe {
        core::ptr::copy(s, d, n);
    }
    dest
}

#[abi(memcpy)]
#[unsafe(no_mangle)]
extern "C" fn abi_memcpy(dest: *mut c_void, src: *const c_void, n: c_size_t) -> *mut c_void {
    let d = dest as *mut u8;
    let s = src as *const u8;
    unsafe {
        // core::ptr::copy_nonoverlapping(s, d, n);
        core::ptr::copy(s, d, n);
    }
    dest
}

#[abi(memchr)]
#[unsafe(no_mangle)]
extern "C" fn abi_memchr(src: *const c_void, c: c_int, n: c_size_t) -> *mut c_void {
    let byte = c as u8;
    let s_ptr = src.cast::<u8>();
    for i in 0..n as usize {
        unsafe {
            if s_ptr.add(i).read() == byte {
                return s_ptr.add(i).cast_mut().cast();
            }
        }
    }
    core::ptr::null_mut()
}

#[abi(memcmp)]
#[unsafe(no_mangle)]
extern "C" fn abi_memcmp(vl: *const c_void, vr: *const c_void, n: c_size_t) -> c_int {
    let (vl, vr) = (vl.cast::<u8>(), vr.cast::<u8>());
    unsafe {
        for i in 0..n as usize {
            let (a, b) = (vl.add(i).read(), vr.add(i).read());
            let diff = a as i32 - b as i32;
            if diff != 0 {
                return diff;
            }
        }
    }
    0
}

#[abi(memset)]
#[unsafe(no_mangle)]
extern "C" fn abi_memset(dest: *mut c_void, c: c_int, n: c_size_t) -> *mut c_void {
    let dest_ptr = dest.cast::<u8>();
    let slice = unsafe { core::slice::from_raw_parts_mut(dest_ptr, n as usize) };
    slice.fill(c as u8);
    dest
}

#[abi(strrchr)]
#[unsafe(no_mangle)]
extern "C" fn abi_strrchr(s: *const c_char, c: c_int) -> *mut c_char {
    unsafe { __memrchr(s.cast::<c_void>(), c, abi_strlen(s) + 1) as *mut _ }
}

#[unsafe(no_mangle)]
extern "C" fn __memrchr(m: *const c_void, c: c_int, n: c_size_t) -> *mut c_void {
    let byte = c as u8;
    let m_ptr = m.cast::<u8>();
    for i in 0..n as usize {
        unsafe {
            if m_ptr.add(i).read() == byte {
                return m_ptr.add(i).cast_mut().cast();
            }
        }
    }
    core::ptr::null_mut()
}

#[abi(strncmp)]
#[unsafe(no_mangle)]
extern "C" fn abi_strncmp(l: *const c_char, r: *const c_char, mut n: c_size_t) -> c_int {
    let (mut l, mut r) = (l.cast::<u8>(), r.cast::<u8>());
    if n == 0 {
        return 0;
    }
    n -= 1;
    unsafe {
        while *l != 0 && *r != 0 && n > 0 && *l == *r {
            l = l.add(1);
            r = r.add(1);
            n -= 1;
        }
    }
    unsafe { (*l as c_int) - (*r as c_int) }
}

#[abi(strcmp)]
#[unsafe(no_mangle)]
extern "C" fn abi_strcmp(l: *const c_char, r: *const c_char) -> c_int {
    if l.is_null() || r.is_null() {
        return if l.is_null() && r.is_null() { 0 } else { 1 };
    }
    let (l, r) = (unsafe { CStr::from_ptr(l) }, unsafe { CStr::from_ptr(r) });
    // match l.cmp(r){
    //     Ordering::Less => -1,
    //     Ordering::Equal => 0,
    //     Ordering::Greater => 1,
    // }
    _strcmp(l.to_bytes(), r.to_bytes())
}
fn _strcmp(s1: &[u8], s2: &[u8]) -> i32 {
    let min_len = s1.len().min(s2.len());

    for i in 0..min_len {
        let diff = s1[i] as i32 - s2[i] as i32;
        if diff != 0 {
            return diff;
        }
    }

    s1.len().cmp(&s2.len()) as i32
}

#[abi(strcspn)]
#[unsafe(no_mangle)]
extern "C" fn abi_strcspn(s: *const c_char, c: *const c_char) -> c_size_t {
    if s.is_null() || c.is_null() {
        return 0;
    }
    let ss = unsafe { CStr::from_ptr(s).to_bytes() };
    let sc = unsafe { CStr::from_ptr(c).to_bytes() };
    ss.iter().position(|&c| sc.contains(&c)).unwrap_or(sc.len())
}

#[abi(strspn)]
#[unsafe(no_mangle)]
extern "C" fn abi_strspn(s: *const c_char, c: *const c_char) -> c_size_t {
    if s.is_null() || c.is_null() {
        return 0;
    }
    let ss = unsafe { CStr::from_ptr(s).to_bytes() };
    let sc = unsafe { CStr::from_ptr(c).to_bytes() };
    ss.iter().take_while(|&&c| sc.contains(&c)).count() as c_size_t
}

#[abi(strchr)]
#[unsafe(no_mangle)]
extern "C" fn abi_strchr(s: *const c_char, c: c_int) -> *const c_char {
    if s.is_null() {
        return core::ptr::null();
    }
    let t = c as u8;
    let ss = unsafe { CStr::from_ptr(s) }.to_bytes();

    match ss.iter().position(|&c| c == t) {
        None => core::ptr::null(),
        Some(pos) => unsafe { s.add(pos) },
    }
}

#[abi(strcpy)]
#[unsafe(no_mangle)]
extern "C" fn abi_strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char {
    let src_data = unsafe { CStr::from_ptr(src) }.to_bytes_with_nul();
    let dest_slice = core::ptr::slice_from_raw_parts_mut(dest.cast::<u8>(), src_data.len());

    unsafe {
        (*dest_slice).copy_from_slice(src_data);
    }
    dest
}
