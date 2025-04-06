use axlog::info;
use axstd::{print, println, process::exit};

#[unsafe(no_mangle)]
pub extern "C" fn abi_noimpl() {
    info!("[ABI: SYS CTL] noimpl");
    print!("\x1b[31m");
    println!("[ABI] No impl yet...");
    print!("\x1b[0m");
    exit(0);
}
