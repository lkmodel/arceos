#![no_std]
#![no_main]

mod abi;

#[cfg(not(test))]
use core::panic::PanicInfo;

use abi::ABI_ENTRY;
pub use abi::{
	hello,
	exit,
	putchar,
};

#[unsafe(no_mangle)]
unsafe extern "C" fn _start() {
    unsafe { 
        core::arch::asm!("
            mv      {abi_table}, a2",
            abi_table = out(reg) ABI_ENTRY,
        );
		main();
	}
}

unsafe extern "C" {
    fn main();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
