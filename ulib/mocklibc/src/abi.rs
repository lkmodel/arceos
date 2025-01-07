const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_EXIT: usize = 3;
pub static mut ABI_ENTRY: usize = 0;

use core::arch::asm;

#[macro_export]
macro_rules! abi_call {
    ($abi_num: expr, $arg0: expr) => {{
        unsafe { asm!("
            li      a0, {abi_num}
            la      t1, {abi_entry}
            ld      t1, (t1)
            jalr    t1",
            abi_num = const $abi_num,
            abi_entry = sym ABI_ENTRY,
            in("a1") $arg0,
            clobber_abi("C"),
        )}
    }}
}

#[unsafe(no_mangle)]
pub extern "C" fn hello() {
    abi_call!(SYS_HELLO, 0);
}

#[unsafe(no_mangle)]
pub extern "C" fn putchar(c: u8) {
    abi_call!(SYS_PUTCHAR, c as usize);
}

#[unsafe(no_mangle)]
pub extern "C" fn exit(exit_code: i32) {
    abi_call!(SYS_EXIT, exit_code as usize);
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_create() {
    abi_call!(4, 0);
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_join() {
    abi_call!(5, 0);
}

#[unsafe(no_mangle)]
pub extern "C" fn pthread_exit() {
    abi_call!(6, 0);
}