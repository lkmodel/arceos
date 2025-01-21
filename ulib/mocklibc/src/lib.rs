//! # Bare Metal Program Initialization
//! 
//! This module provides the core initialization and runtime support for a bare metal
//! Rust program running on RISC-V architecture. It establishes the necessary environment
//! for running Rust code without an operating system or standard library support.
//! 
//! ## Architecture
//! The initialization process follows these steps:
//! 1. Program starts at `_start`
//! 2. System call table address is retrieved from a2 register
//! 3. Main function is called to begin program execution
//! 
//! ## Safety
//! This code necessarily uses unsafe operations as it interfaces directly with hardware.
//! Care must be taken when modifying this code to maintain system integrity.

#![no_std]
#![no_main]

/// Module containing the Application Binary Interface definitions.
/// This provides the bridge between our Rust code and the underlying system.
mod abi;

#[cfg(not(test))]
use core::panic::PanicInfo;

use abi::ABI_ENTRY;

/// Re-exports of essential system calls for RISC-V 64-bit architecture.
/// These functions provide the basic interaction capabilities with the underlying system.
#[cfg(target_arch = "riscv64")]
pub use abi::{
    hello,   // Outputs a greeting message
    exit,    // Terminates program execution
    putchar, // Outputs a single character
};

/// Program entry point and initialization function.
/// 
/// This function is called first when the program starts and is responsible for:
/// - Capturing the system call table address from register a2
/// - Initializing the runtime environment
/// - Transferring control to the main function
/// 
/// # Safety
/// This function is unsafe because it:
/// - Uses inline assembly to access CPU registers
/// - Performs raw pointer operations
/// - Calls an unsafe main function
#[unsafe(no_mangle) ]
unsafe extern "C" fn _start() {
    unsafe { 
        // Retrieve system call table address from a2 register.
        // This address is provided by the runtime environment and is crucial
        // for making system calls later in the program.
        core::arch::asm!("
            mv      {abi_table}, a2",
            abi_table = out(reg) ABI_ENTRY,
        );
        // Transfer control to the main function after initialization is complete
        main();
    }
}

unsafe extern "C" {
    unsafe fn main();
}

/// Panic handler for non-test builds.
/// 
/// This function is called when a panic occurs in the program.
/// In this bare metal environment, we simply halt the program
/// by entering an infinite loop.
/// 
/// # Arguments
/// * `_info` - Contains information about the panic, currently unused
/// 
/// # Never Returns
/// This function implements a diverging infinite loop
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}