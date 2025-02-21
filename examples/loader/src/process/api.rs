extern crate alloc;
use alloc::{string::ToString, sync::Arc};
use axerrno::AxResult;
use axhal::{mem::VirtAddr, paging::MappingFlags};
use axlog::{debug, info};
use axmm::AddrSpace;
use axtask::{current, AxTaskRef, CurrentTask, TaskExtRef};
use crate::elf::{elf::load_elf, EXEC_ZONE_START};

use super::{Process, PID2PC, TID2TASK};

/// return the `Arc<Process>` of the current process
#[allow(unused)]
pub fn current_process() -> Arc<Process> {
    let current_task = current();
    let current_process = Arc::clone(PID2PC.lock().get(&current_task.task_ext().get_process_id()).unwrap());

    current_process
}

/// 退出当前任务
pub fn exit_current_task(exit_code: i32) -> ! {
    // let process = current_process();
    // let current_task = current();

    // let curr_id = current_task.id().as_u64();

    // info!("exit task id {} with code _{}_", curr_id, exit_code);

    // // clear_child_tid 的值不为 0，则将这个用户地址处的值写为0
    // let clear_child_tid = current_task.get_clear_child_tid();
    // if current_task.is_leader() {
    //     loop {
    //         let mut all_exited = true;
    //         for task in process.tasks.lock().deref() {
    //             if !task.is_leader() && task.state() != TaskState::Exited {
    //                 all_exited = false;
    //             }
    //         }
    //         if !all_exited {
    //             yield_now();
    //         } else {
    //             break;
    //         }
    //     }
    //     TID2TASK.lock().remove(&curr_id);
    //     process.set_exit_code(exit_code);

    //     process.set_zombie(true);

    //     process.tasks.lock().clear();
    //     process.fd_manager.fd_table.lock().clear();

    //     let mut pid2pc = PID2PC.lock();
    //     let kernel_process = pid2pc.get(&KERNEL_PROCESS_ID).unwrap();
    //     // 将子进程交给idle进程
    //     // process.memory_set = Arc::clone(&kernel_process.memory_set);
    //     for child in process.children.lock().deref() {
    //         child.set_parent(KERNEL_PROCESS_ID);
    //         kernel_process.children.lock().push(Arc::clone(child));
    //     }
    //     pid2pc.remove(&process.pid());
    //     drop(pid2pc);
    //     drop(process);
    // } else {
    //     TID2TASK.lock().remove(&curr_id);
    //     // 从进程中删除当前线程
    //     let mut tasks = process.tasks.lock();
    //     let len = tasks.len();
    //     for index in 0..len {
    //         if tasks[index].id().as_u64() == curr_id {
    //             tasks.remove(index);
    //             break;
    //         }
    //     }
    //     drop(tasks);

    //     drop(process);
    // }
    axtask::exit(exit_code);
}

/// Load a user app.
///
/// # Returns
/// - The first return value is the entry point of the user app.
/// - The second return value is the top of the user stack.
/// - The third return value is the address space of the user app.
pub fn load_user_app(
    memory_set: &mut AddrSpace,
    app_name: &str,
    elf_file: &'static [u8]
) -> AxResult<(VirtAddr, VirtAddr)> {

    let elf_info = load_elf(VirtAddr::from(EXEC_ZONE_START), elf_file);
    for segement in elf_info.segments {
        debug!(
            "Mapping ELF segment: [{:#x?}, {:#x?}) flags: {:#x?}",
            segement.start_vaddr,
            segement.start_vaddr + segement.size,
            segement.flags
        );
        memory_set.map_alloc(segement.start_vaddr, segement.size, segement.flags, true)?;

        if segement.data.is_empty() {
            continue;
        }

        memory_set.write(segement.start_vaddr + segement.offset, &segement.data)?;
    }

    info!("Mapping user stack {:?}", memory_set);

    // The user stack is divided into two parts:
    // `ustack_start` -> `ustack_pointer`: It is the stack space that users actually read and write.
    // `ustack_pointer` -> `ustack_end`: It is the space that contains the arguments, environment variables and auxv passed to the app.
    //  When the app starts running, the stack pointer points to `ustack_pointer`.

    let ustack_end = VirtAddr::from_usize(EXEC_ZONE_START);
    let ustack_size = 0x10000;
    let ustack_start = ustack_end - ustack_size;
    debug!(
        "Mapping user stack: {:#x?} -> {:#x?}",
        ustack_start, ustack_end
    );

    // user-heap-base = "0x3FA0_0000"
    // # The base address of the user stack. And the stack bottom is `user-stack-top + max-user-stack-size`.
    // user-stack-top = "0x3FE0_0000"
    // # The size of the user heap.
    // max-user-heap-size = "0x40_0000"

    // FIXME: Add more arguments and environment variables
    let (stack_data, ustack_pointer) = kernel_elf_parser::get_app_stack_region(
        &[app_name.to_string()],
        &[],
        &elf_info.auxv,
        ustack_start,
        ustack_size,
    );

    info!("Mapping user stack data: {:#x?}", ustack_pointer);

    memory_set.map_alloc(
        ustack_start,
        ustack_size,
        MappingFlags::READ | MappingFlags::WRITE,
        true,
    )?;

    info!("Writing user stack data");

    memory_set.write(VirtAddr::from_usize(ustack_pointer), stack_data.as_slice())?;
    Ok((elf_info.entry, VirtAddr::from(ustack_pointer)))
}

/// 以进程作为中转调用 task 的 yield
#[allow(unused)]
pub fn yield_now_task() {
    axtask::yield_now();
}

/// 以进程作为中转调用 task 的 sleep
#[allow(unused)]
pub fn sleep_now_task(dur: core::time::Duration) {
    axtask::sleep(dur);
}

/// current running task
#[allow(unused)]
pub fn current_task() -> CurrentTask {
    axtask::current()
}

/// 设置当前任务的 clear_child_tid
#[allow(unused)]
pub fn set_child_tid(tid: usize) {
    todo!()
}

/// Get the task reference by tid
#[allow(unused)]
pub fn get_task_ref(tid: u64) -> Option<AxTaskRef> {
    TID2TASK.lock().get(&tid).cloned()
}