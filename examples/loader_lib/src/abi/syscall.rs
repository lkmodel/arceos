use crate::syscall::syscall::syscall;
use cty::c_long;

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall0(syscall_id: c_long) -> c_long {
    let retval = syscall(syscall_id as usize, [0; 6]);
    return retval as c_long;
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall1(syscall_id: c_long, a: c_long) -> c_long {
    let retval = syscall(syscall_id as usize, [a as usize, 0, 0, 0, 0, 0]);
    return retval as c_long;
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall2(syscall_id: c_long, a: c_long, b: c_long) -> c_long {
    let retval = syscall(syscall_id as usize, [a as usize, b as usize, 0, 0, 0, 0]);
    return retval as c_long;
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall3(syscall_id: c_long, a: c_long, b: c_long, c: c_long) -> c_long {
    let retval = syscall(syscall_id as usize, [
        a as usize, b as usize, c as usize, 0, 0, 0,
    ]);
    return retval as c_long;
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall4(
    syscall_id: c_long,
    a: c_long,
    b: c_long,
    c: c_long,
    d: c_long,
) -> c_long {
    let retval = syscall(syscall_id as usize, [
        a as usize, b as usize, c as usize, d as usize, 0, 0,
    ]);
    return retval as c_long;
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall5(
    syscall_id: c_long,
    a: c_long,
    b: c_long,
    c: c_long,
    d: c_long,
    e: c_long,
) -> c_long {
    let retval = syscall(syscall_id as usize, [
        a as usize, b as usize, c as usize, d as usize, e as usize, 0,
    ]);
    return retval as c_long;
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_syscall6(
    syscall_id: c_long,
    a: c_long,
    b: c_long,
    c: c_long,
    d: c_long,
    e: c_long,
    f: c_long,
) -> c_long {
    let retval = syscall(syscall_id as usize, [
        a as usize, b as usize, c as usize, d as usize, e as usize, f as usize,
    ]);
    return retval as c_long;
}
