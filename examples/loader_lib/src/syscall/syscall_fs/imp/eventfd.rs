use crate::syscall::SyscallResult;

pub fn syscall_eventfd(_args: [usize; 6]) -> SyscallResult {
    unimplemented!();
}
