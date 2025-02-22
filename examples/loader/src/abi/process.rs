use core::{fmt::Display, slice::from_raw_parts};
use axlog::{error, info, trace};
use axtask::current;
use crate::process::current_process;

static mut SAVED_TASK_CTX: UserContext = UserContext::new();

#[repr(C)]
#[derive(Debug, Clone)]
pub struct UserContext {
    pub ra: usize,
    pub sp: usize,
    pub s0: usize,
    pub s1: usize,
    pub s2: usize,
    pub s3: usize,
    pub s4: usize,
    pub s5: usize,
    pub s6: usize,
    pub s7: usize,
    pub s8: usize,
    pub s9: usize,
    pub s10: usize,
    pub s11: usize,
    pub tp: usize,
}

impl UserContext {
    pub const fn new() -> Self {
        unsafe { core::mem::MaybeUninit::zeroed().assume_init() }
    }
}

impl Display for UserContext {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "UserContext {{ ra: 0x{:x}, sp: 0x{:x}, s0: 0x{:x}, s1: 0x{:x}, s2: 0x{:x}, s3: 0x{:x}, s4: 0x{:x}, s5: 0x{:x}, s6: 0x{:x}, s7: 0x{:x}, s8: 0x{:x}, s9: 0x{:x}, s10: 0x{:x}, s11: 0x{:x}, tp: 0x{:x} }}",
            self.ra, self.sp, self.s0, self.s1, self.s2, self.s3, self.s4, self.s5, self.s6, self.s7, self.s8, self.s9, self.s10, self.s11, self.tp
        )
    }
}

#[naked]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_fork_entry() -> i32 {
    unsafe {
        core::arch::naked_asm!("
            // 保存当前返回地址，因为后面会被覆盖
            mv      t1, ra
            
            // 保存寄存器到 SAVED_TASK_CTX
            la      t0, {}
            sd      t1, 0(t0)         // 保存原始 ra
            sd      sp, 8(t0)         // sp
            sd      s0, 16(t0)        // s0
            sd      s1, 24(t0)        // s1
            sd      s2, 32(t0)        // s2
            sd      s3, 40(t0)        // s3
            sd      s4, 48(t0)        // s4
            sd      s5, 56(t0)        // s5
            sd      s6, 64(t0)        // s6
            sd      s7, 72(t0)        // s7
            sd      s8, 80(t0)        // s8
            sd      s9, 88(t0)        // s9
            sd      s10, 96(t0)       // s10
            sd      s11, 104(t0)      // s11
            sd      tp, 112(t0)       // tp

            // 调用 abi_fork
            mv      a0, t0
            tail    abi_fork
            ",
            sym SAVED_TASK_CTX,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn abi_fork(task_ctx: UserContext) -> i32 {
    info!("[ABI:Process] Fork a new process!");
    info!("TaskContext: {:x?}", task_ctx);
    let current = current();
    let kernel_top = current.as_task_ref().inner().kernel_stack_top().unwrap();
    let stack_size = kernel_top.as_usize() - task_ctx.sp;

    info!("Kernel stack top: {:x?} sp : {:x?}, stack_size {:x?}", kernel_top, task_ctx.sp, stack_size);

    let stack_data = unsafe { from_raw_parts(task_ctx.sp as *const u8, stack_size) };

    // 1. 获取当前进程
    let curr_process = current_process();
    trace!("Current process: PID = {}", curr_process.pid());

    // 2. 调用进程的 fork 方法
    match curr_process.fork(stack_data, task_ctx) {
        Ok(child_pid) => {
            // 在父进程中返回子进程的 PID
            // 子进程的返回值在 fork() 内部设置为 0
            trace!("Fork success! Parent={}, Child={}", curr_process.pid(), child_pid);
            child_pid as i32
        }
        Err(err) => {
            // fork 失败时返回负数错误码
            error!("[ABI:Process] Fork failed: {:?}", err);
            -1
        }
    }
}