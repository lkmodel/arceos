use core::mem;
use axlog::info;
use axtask::init_scheduler;
use crate::load::EXEC_ZONE_START;

type MainFn = unsafe extern "C" fn(argc: i32, argv: *mut *mut i8, envp: *mut *mut i8) -> i32;

/// Description
/// The `__libc_start_main()` function shall initialize the process, call the main function with appropriate arguments, and handle the return from main().
/// `__libc_start_main()` is not in the source standard; it is only in the binary standard. 
#[unsafe(no_mangle)]
pub extern "C" fn abi_libc_start_main(
	main: MainFn,
	argc: i32,
    argv: *mut *mut i8,
) {
	info!("[ABI:Init]: abi_libc_start_main");
	info!("main: {:?}, argc: {}, argv: {:?}", main, argc, argv);

	init_scheduler();

	let main = unsafe {
		mem::transmute::<usize, MainFn>(main as usize + EXEC_ZONE_START)
	};
	unsafe { 
		main(argc, argv, core::ptr::null_mut());
	}

	abi_fini();
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_init() {
    info!("[ABI:Init]: abi_init");
}

#[unsafe(no_mangle)]
pub extern "C" fn abi_fini() {
	info!("[ABI:Init]: abi_fini");
}