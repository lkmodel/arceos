mod mem;
mod init;
mod thread;
mod string;

use core::arch::asm;
use init::{
    abi_exit, abi_fini, abi_hello,
    abi_init, abi_libc_start_main,
    abi_printf, abi_putchar,
    abi_puts, abi_sleep
};

use mem::{abi_calloc, abi_free, abi_malloc, abi_realloc};
use string::abi_strlen;
use thread::{
    abi_pthread_create, abi_pthread_exit, abi_pthread_join, abi_pthread_mutex_destroy, abi_pthread_mutex_init, abi_pthread_mutex_lock, abi_pthread_mutex_unlock, abi_pthread_self
};

const SYS_HELLO: usize = 1;
const SYS_PUTCHAR: usize = 2;
const SYS_EXIT: usize = 3;

pub fn abi_entry(abi_num: usize, arg0: usize) {    
    match abi_num  {
        SYS_HELLO => abi_hello(),
        SYS_PUTCHAR => abi_putchar(arg0 as u8 as char),
        SYS_EXIT => abi_exit(arg0 as i32),
        _ => panic!("[ABI:Unknown] Unknown ABI: 0x{abi_num:x}")
    }
}

macro_rules! register_abi_table {
    ($(($name:expr, $func:expr)),*) => {
        pub fn lookup_abi_call(name: &str) -> Option<usize> {
            match name {
                $(
                    $name => Some($func as usize),
                )*
                _ => None
            }
        }
    }
}

register_abi_table! {
    ("hello", abi_hello),
    ("putchar", abi_putchar),
    ("exit", abi_exit),
    ("pthread_create", abi_pthread_create),
    ("pthread_join", abi_pthread_join),
    ("pthread_exit", abi_pthread_exit),
    ("pthread_self", abi_pthread_self),
    ("pthread_mutex_init", abi_pthread_mutex_init),
    ("pthread_mutex_lock", abi_pthread_mutex_lock),
    ("pthread_mutex_unlock", abi_pthread_mutex_unlock),
    // TODO
    ("pthread_mutex_destroy", abi_pthread_mutex_destroy),
    ("printf", abi_printf),
    ("puts", abi_puts),
    ("sleep", abi_sleep),
    ("_init", abi_init),
    ("_fini", abi_fini),
    ("__libc_start_main", abi_libc_start_main),
    ("malloc", abi_malloc),
    ("free", abi_free),
    ("realloc", abi_realloc),
    ("calloc", abi_calloc),
    ("strlen", abi_strlen),
    ("longjmp",abi_longjmp),
    ("setjmp", abi_setjmp)
}
unsafe extern "C" fn abi_longjmp() {
    asm!(
    "
    ld s0,    0(a0)
	ld s1,    8(a0)
	ld s2,    16(a0)
	ld s3,    24(a0)
	ld s4,    32(a0)
	ld s5,    40(a0)
	ld s6,    48(a0)
	ld s7,    56(a0)
	ld s8,    64(a0)
	ld s9,    72(a0)
	ld s10,   80(a0)
	ld s11,   88(a0)
	ld sp,    96(a0)
	ld ra,    104(a0)

	seqz a0, a1
	add a0, a0, a1
	ret
	"
    )
}
unsafe extern "C" fn abi_setjmp() {
    asm!("
    sd s0,    0(a0)
	sd s1,    8(a0)
	sd s2,    16(a0)
	sd s3,    24(a0)
	sd s4,    32(a0)
	sd s5,    40(a0)
	sd s6,    48(a0)
	sd s7,    56(a0)
	sd s8,    64(a0)
	sd s9,    72(a0)
	sd s10,   80(a0)
	sd s11,   88(a0)
	sd sp,    96(a0)
	sd ra,    104(a0)

	li a0, 0
	ret
	")
}