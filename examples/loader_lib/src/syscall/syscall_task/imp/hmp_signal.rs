//! 信号相关系统调用的占位符实现（不依赖 `axsignal`）
//! 这些函数在可能的情况下假装成功，但在需要返回特定状态（如旧的掩码/动作）
//! 或操作本身无法被 stub（如 `sigsuspend/sigreturn`）时返回错误。

use crate::syscall::{SyscallError, SyscallResult};
use axlog::{error, warn};

/// syscall_sigaction 的占位符 (无 `axsignal` 依赖)。
/// 假装设置了新的动作，但无法返回旧的动作。
/// # 参数
/// * `signum` - `usize`: 信号编号
/// * `action` - `usize`: 指向新动作结构体的指针 (地址)
/// * `old_action` - `usize`: 指向存储旧动作结构体的指针 (地址)
pub fn syscall_sigaction(args: [usize; 6]) -> SyscallResult {
    let signum = args[0];
    let action_ptr = args[1]; // action 参数作为地址值
    let old_action_ptr = args[2]; // old_action 参数作为地址值

    warn!(
        "syscall_sigaction(placeholder-no-axsignal): signum: {}, action_ptr: {:X}, old_action_ptr: {:X}",
        signum, action_ptr, old_action_ptr
    );

    // 如果调用者期望返回旧动作 (old_action_ptr 非零)，占位符无法提供。
    if old_action_ptr != 0 {
        warn!(
            "syscall_sigaction(placeholder-no-axsignal): Cannot return old_action, returning EFAULT."
        );
        // 返回 EFAULT (坏地址)，因为我们通常会尝试写入这里。
        return Err(SyscallError::EFAULT);
    }

    // 如果只是设置新动作 (action_ptr 非零) 或什么都不做 (两个指针都为零)，则假装成功。
    warn!(
        "syscall_sigaction(placeholder-no-axsignal): Pretending to successfully set new action (if provided)."
    );
    Ok(0)
}

/// Placeholder for syscall_sigsuspend.
/// This function normally blocks until a signal arrives. A placeholder cannot block.
/// It typically returns -1 with `errno` EINTR.
/// # Arguments
/// * `mask` - *const `usize`
///
/// syscall_sigsuspend 的占位符。
/// 此函数通常阻塞直到信号到达。占位符无法阻塞。
/// 它通常返回 -1 并将 `errno` 设置为 EINTR。
/// # 参数
/// * `mask` - *const `usize`
pub fn syscall_sigsuspend(args: [usize; 6]) -> SyscallResult {
    // 标记为未使用，但保留以匹配签名
    // Mark as unused, but keep for signature
    let _mask = args[0] as *const usize;
    warn!(
        "syscall_sigsuspend(placeholder): Called with mask {:X}. Cannot block, immediately returning EINTR.",
        args[0]
    );

    // `sigsuspend` is designed to wait for a signal. When it returns, it always
    // indicates failure with EINTR because its purpose (waiting) was interrupted
    // by the signal handler. So, returning EINTR is the most logical stub behavior.
    // `sigsuspend` 被设计为等待信号。当它返回时，它总是
    // 通过 EINTR 表示失败，因为其目的（等待）被信号处理程序中断了。
    // 因此，返回 EINTR 是最合乎逻辑的 stub 行为。
    Err(SyscallError::EINTR)
}

/// Placeholder for syscall_sigreturn.
/// This function is critical for restoring state after a signal handler.
/// It cannot be meaningfully stubbed.
///
/// syscall_sigreturn 的占位符。
/// 此函数对于信号处理程序返回后恢复状态至关重要。
/// 它无法被有意义地 stub。
pub fn syscall_sigreturn() -> SyscallResult {
    error!(
        "syscall_sigreturn(placeholder): This syscall cannot be meaningfully stubbed! It's essential for exiting signal handlers correctly. Returning ENOSYS."
    );
    // Returning an error like `ENOSYS` (Function not implemented) or `EPERM` is appropriate.
    // Panicking might also be an option to halt execution if this path is unexpectedly hit.
    //
    // 返回像 `ENOSYS`（功能未实现）或 `EPERM` 这样的错误是合适的。
    // 如果意外执行到此路径，panic 也可能是一个选项，以停止执行。
    //
    // `panic!("syscall_sigreturn should not be called in placeholder mode!");`
    Err(SyscallError::ENOSYS)
}

/// Placeholder for syscall_kill (no `axsignal` dependency).
/// Pretends to send a signal without actually doing anything.
/// # Arguments
/// * `pid` - `isize`: Process ID
/// * `signum` - `isize`: Signal number
///
/// syscall_kill 的占位符 (无 `axsignal` 依赖)。
/// 假装发送信号，但实际上什么也不做。
/// # 参数
/// * `pid` - `isize`: 进程 ID
/// * `signum` - `isize`: 信号编号
pub fn syscall_kill(args: [usize; 6]) -> SyscallResult {
    // `isize` conversion needs platform awareness, but `usize` -> `isize` is generally safe
    // `isize` 转换需要注意平台差异，但通常 `usize` -> `isize` 是安全的
    let pid = args[0] as isize;
    let signum = args[1] as isize;

    warn!(
        "syscall_kill(placeholder-no-axsignal): pid: {}, signum: {}. Pretending success if args are valid.",
        pid, signum
    );

    // Basic validation similar to before
    // 与之前类似的基本验证
    if pid > 0 {
        if signum >= 0 {
            warn!(
                "syscall_kill(placeholder-no-axsignal): Pretending success for pid {} signum {}.",
                pid, signum
            );
            Ok(0)
        } else {
            warn!(
                "syscall_kill(placeholder-no-axsignal): Invalid signum {}. Returning EINVAL.",
                signum
            );
            // Invalid signal
            // 无效信号
            Err(SyscallError::EINVAL)
        }
    } else if pid == 0 {
        warn!(
            "syscall_kill(placeholder-no-axsignal): pid 0 not supported by placeholder. Returning ESRCH."
        );
        // No such process (simplification)
        // 没有这样的进程（简化）
        Err(SyscallError::ESRCH)
    } else if pid == -1 {
        warn!(
            "syscall_kill(placeholder-no-axsignal): pid -1 not supported by placeholder. Returning ESRCH."
        );
        // No such process (simplification)
        // 没有这样的进程（简化）
        Err(SyscallError::ESRCH)
    } else {
        warn!(
            "syscall_kill(placeholder-no-axsignal): pid < -1 not supported by placeholder. Returning ESRCH."
        );
        // No such process (simplification)
        // 没有这样的进程（简化）
        Err(SyscallError::ESRCH)
    }
}

/// Placeholder for syscall_tkill (no `axsignal` dependency).
/// Pretends to send a signal to a specific thread without actually doing anything.
/// # Arguments
/// * `tid` - `isize`: Thread ID
/// * `signum` - `isize`: Signal number
///
/// syscall_tkill 的占位符 (无 `axsignal` 依赖)。
/// 假装向特定线程发送信号，但实际上什么也不做。
/// # 参数
/// * `tid` - `isize`: 线程 ID
/// * `signum` - `isize`: 信号编号
pub fn syscall_tkill(args: [usize; 6]) -> SyscallResult {
    let tid = args[0] as isize;
    let signum = args[1] as isize;

    warn!(
        "syscall_tkill(placeholder-no-axsignal): tid: {}, signum: {}. Pretending success if args are valid.",
        tid, signum
    );

    // 与之前类似的基本验证
    if tid > 0 && signum >= 0 {
        warn!(
            "syscall_tkill(placeholder-no-axsignal): Pretending success for tid {} signum {}.",
            tid, signum
        );
        Ok(0)
    } else {
        warn!(
            "syscall_tkill(placeholder-no-axsignal): Invalid tid ({}) or signum ({}). Returning EINVAL.",
            tid, signum
        );
        Err(SyscallError::EINVAL) // 无效参数
    }
}

/// Placeholder for syscall_sigprocmask (no `axsignal` dependency).
/// Pretends to set a new mask but cannot return the old mask.
/// # Arguments
/// * `how` - `usize`: Operation type (e.g., values corresponding to `SIG_BLOCK`, `SIG_UNBLOCK`, `SIG_SETMASK`)
/// * `set` - `usize`: Pointer (address) to the new signal mask, ignored if 0
/// * `oldset` - `usize`: Pointer (address) to store the old signal mask, ignored if 0
/// * `sigsetsize` - `usize`: Size of the signal set in bytes
///
/// syscall_sigprocmask 的占位符 (无 `axsignal` 依赖)。
/// 假装设置了新掩码，但无法返回旧掩码。
/// # 参数
/// * `how` - `usize`: 操作类型 (例如 `SIG_BLOCK`, `SIG_UNBLOCK`, `SIG_SETMASK` 对应的值)
/// * `set` - `usize`: 指向新信号掩码的指针 (地址)，如果为 0 则忽略
/// * `oldset` - `usize`: 指向存储旧信号掩码的指针 (地址)，如果为 0 则忽略
/// * `sigsetsize` - `usize`: 信号集的大小 (字节)
pub fn syscall_sigprocmask(args: [usize; 6]) -> SyscallResult {
    let how = args[0]; // 操作类型
    let set_ptr = args[1]; // 新掩码指针
    let oldset_ptr = args[2]; // 旧掩码指针
    let sigsetsize = args[3]; // 信号集大小

    warn!(
        "syscall_sigprocmask(placeholder-no-axsignal): how: {}, set_ptr: {:X}, oldset_ptr: {:X}, sigsetsize: {}",
        how, set_ptr, oldset_ptr, sigsetsize
    );

    // If the caller expects the old mask back (oldset_ptr is non-zero), we cannot provide it.
    // 如果调用者期望返回旧掩码 (oldset_ptr 非零)，我们无法提供。
    if oldset_ptr != 0 {
        warn!(
            "syscall_sigprocmask(placeholder-no-axsignal): Cannot return old_mask, returning EFAULT."
        );
        // Return EFAULT (Bad address) as we would normally try to write here.
        // 返回 EFAULT (坏地址)，因为我们通常会尝试写入这里。
        return Err(SyscallError::EFAULT);
    }

    // If only setting a new mask (set_ptr is non-zero) or doing nothing (both pointers are zero), pretend it worked.
    // 如果只是设置新掩码 (set_ptr 非零) 或什么都不做 (两个指针都为零)，则假装成功。
    warn!(
        "syscall_sigprocmask(placeholder-no-axsignal): Pretending to successfully apply new mask (if provided)."
    );
    Ok(0)
}
