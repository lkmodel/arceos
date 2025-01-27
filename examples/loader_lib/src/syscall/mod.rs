pub mod syscall;
mod syscall_fs;

use axerrno::{self, LinuxError};

/// Accept the result of a syscall, and return the `isize` to the user
pub(crate) fn deal_result(result: SyscallResult) -> isize {
    match result {
        Ok(x) => x,
        Err(error) => -(error.code() as isize),
    }
}

pub type SyscallError = LinuxError;

pub type SyscallResult = Result<isize, SyscallError>;
