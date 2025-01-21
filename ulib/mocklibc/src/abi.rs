//! # Application Binary Interface (ABI) Implementation
//! 
//! This module implements the low-level interface between the application and its runtime
//! environment on RISC-V 64-bit architecture. The ABI defines the binary contract that 
//! allows our application to interact with the underlying runtime services, specifying 
//! how functions are called, how parameters are passed, and how results are returned.
//! 
//! ## Design Philosophy
//! The implementation adheres to the RISC-V calling convention, ensuring proper
//! register usage and parameter passing. It provides a stable interface that bridges
//! the gap between high-level Rust code and low-level runtime services.
//! 
//! ## Architecture Overview
//! The ABI uses a table-based approach where the runtime environment provides an entry 
//! point (stored in ABI_ENTRY) that serves as the gateway for all runtime service requests.
//! Each service is identified by a unique number and follows the RISC-V calling convention
//! for parameter passing.
//!
//! ## Memory Safety
//! This implementation involves direct hardware interaction and raw memory operations.
//! All functions are marked as unsafe and proper care must be taken when using them.
//! Users of this interface must ensure they understand and respect the safety requirements.

/// ABI function numbers defining available runtime services.
/// These constants identify specific functions implemented by the runtime
/// environment that our application can invoke through the ABI interface.
const SYS_HELLO: usize = 1;    // Print greeting message
const SYS_PUTCHAR: usize = 2;  // Output single character
const SYS_EXIT: usize = 3;     // Terminate program execution

/// Global storage for the ABI entry point address.
/// 
/// This variable holds the address of the runtime service dispatcher provided
/// by the environment during program initialization. All ABI calls are routed
/// through this entry point.
/// 
/// # Safety
/// This value must only be initialized during program startup and should remain
/// constant throughout program execution. It represents a critical part of the
/// ABI contract between the application and its runtime environment.
pub static mut ABI_ENTRY: usize = 0;

use core::arch::asm;

/// Macro for making ABI function calls on RISC-V 64-bit architecture.
/// 
/// This macro generates the assembly code necessary to invoke runtime services
/// through the ABI entry point. It handles parameter passing according to the
/// RISC-V calling convention and manages the control flow for ABI function calls.
/// 
/// # Arguments
/// * `$abi_num` - The ABI function identifier for the requested service
/// * `$arg0` - The parameter to pass to the ABI function
/// 
/// # Example
/// ```ignore
/// abi_call!(SYS_PUTCHAR, b'A' as usize);  // Request character output service
/// ```
/// 
/// # Technical Details
/// The macro generates assembly that:
/// 1. Loads the ABI function number into a0 (first parameter register)
/// 2. Loads the ABI entry point address
/// 3. Jumps to the entry point while preserving the return address
/// 4. Handles parameter passing according to the RISC-V ABI specification
#[cfg(target_arch = "riscv64")]
#[macro_export]
macro_rules! abi_call {
   ($abi_num: expr, $arg0: expr) => {{
       unsafe { asm!("
           li      a0, {abi_num}     // Load ABI function number into a0 register
           la      t1, {abi_entry}   // Load address of ABI entry point into t1
           ld      t1, (t1)          // Load actual entry point address
           jalr    t1                // Jump to entry point and save return address
           ",
           abi_num = const $abi_num,         // Bind ABI function number
           abi_entry = sym ABI_ENTRY,        // Bind entry point symbol
           in("a1") $arg0,                   // Pass argument in a1 register
           clobber_abi("C"),                 // Mark C ABI registers as clobbered
       )}
   }}
}

/// Outputs a greeting message through the runtime environment.
/// 
/// This ABI function requests the runtime to display a predefined greeting message.
/// The actual message content and display method are determined by the runtime 
/// implementation.
/// 
/// # Safety
/// Must only be called after proper ABI initialization where ABI_ENTRY is set.
/// The runtime environment must support the HELLO service (function number 1).
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn hello() {
   abi_call!(SYS_HELLO, 0);
}

/// Outputs a single character through the runtime environment.
/// 
/// # Arguments
/// * `c` - The character to output (as a byte)
/// 
/// # Safety
/// Must only be called after proper ABI initialization where ABI_ENTRY is set.
/// The runtime environment must provide character output capabilities.
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn putchar(c: u8) {
   abi_call!(SYS_PUTCHAR, c as usize);
}

/// Requests program termination through the runtime environment.
/// 
/// # Arguments
/// * `exit_code` - The program's termination status (0 typically indicates success)
/// 
/// # Safety
/// This function will terminate the program through the runtime environment.
/// Ensure all cleanup operations are performed before calling.
#[cfg(target_arch = "riscv64")]
#[no_mangle]
pub extern "C" fn exit(exit_code: i32) {
   abi_call!(SYS_EXIT, exit_code as usize);
}
