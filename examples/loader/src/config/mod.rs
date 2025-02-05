pub const KERNEL_PROCESS_ID: u64 = 1;

/// The size of the user heap.
pub const MAX_USER_HEAP_SIZE: usize = 0;
/// The size of the user stack.
pub const MAX_USER_STACK_SIZE: usize = 0;

/// The base address of the user heap.
pub const USER_HEAP_BASE: usize = 0;
/// The base address of the user stack. And the stack bottom is `user-stack-top + max-user-stack-size`.
pub const USER_STACK_TOP: usize = 0;
/// Stack size of each task.
pub const TASK_STACK_SIZE: usize = 0x40000;
/// The base address of the signal trampoline.
pub const SIGNAL_TRAMPOLINE: usize = 0;