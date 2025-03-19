use alloc::sync::Arc;
use axtask::{AxTaskRef, CurrentTask, TaskExtRef, current};

use crate::linux_env::process_ext::process::{PID2PC, Process, TID2TASK};

/// Return the `Arc<Process>` of the current process
#[allow(unused)]
pub fn current_process() -> Arc<Process> {
    let current_task = current();
    let current_process = Arc::clone(
        PID2PC
            .lock()
            .get(&current_task.task_ext().get_process_id())
            .unwrap(),
    );

    current_process
}

/// 退出当前任务
// pub fn exit_current_task(exit_code: i32) -> ! {
//     unimplemented!()
//  let process = current_process();
//  let current_task = current();

//  let curr_id = current_task.id().as_u64();

//  info!("exit task id {} with code _{}_", curr_id, exit_code);
//  clear_wait(
//  /    if current_task.is_leader() {
//          process.pid()
//      } else {
//          curr_id
//      },
//      current_task.is_leader(),
//  );
//  // 检查这个任务是否有sig_child信号
//  /#[cfg(feature = "signal")]
//  if current_task.get_sig_child() || current_task.is_leader() {
//      let parent = process.get_parent();
//      if parent != KERNEL_PROCESS_ID {
//          // 发送sigchild
//          send_signal_to_process(parent as isize, 17).unwrap();
//      }
//  }
//  // clear_child_tid 的值不为 0，则将这个用户地址处的值写为0
//  let clear_child_tid = current_task.get_clear_child_tid();
//  if clear_child_tid != 0 {
//      // 先确认是否在用户空间
//      if process
//          .manual_alloc_for_lazy(clear_child_tid.into())
//          .is_ok()
//      {
//          unsafe {
//              *(clear_child_tid as *mut i32) = 0;
//          }
//      }
//  }
//  if current_task.is_leader() {
//      loop {
//          let mut all_exited = true;

//          for task in process.tasks.lock().deref() {
//              if !task.is_leader() && task.state() != TaskState::Exited {
//                  all_exited = false;
//                  break;
//              }
//          }
//          if !all_exited {
//              yield_now();
//          } else {
//              break;
//          }
//      }
//      TID2TASK.lock().remove(&curr_id);
//      process.set_exit_code(exit_code);

//      process.set_zombie(true);

//      process.tasks.lock().clear();
//      process.fd_manager.fd_table.lock().clear();
//      #[cfg(feature = "signal")]
//      process.signal_modules.lock().clear();

//      let mut pid2pc = PID2PC.lock();
//      let kernel_process = pid2pc.get(&KERNEL_PROCESS_ID).unwrap();
//      // 将子进程交给idle进程
//      // process.memory_set = Arc::clone(&kernel_process.memory_set);
//      for child in process.children.lock().deref() {
//          child.set_parent(KERNEL_PROCESS_ID);
//          kernel_process.children.lock().push(Arc::clone(child));
//      }
//      if let Some(parent_process) = pid2pc.get(&process.get_parent()) {
//          parent_process.set_vfork_block(false);
//      }
//      pid2pc.remove(&process.pid());
//      drop(pid2pc);
//      drop(process);
//  } else {
//      TID2TASK.lock().remove(&curr_id);
//      // 从进程中删除当前线程
//      let mut tasks = process.tasks.lock();
//      let len = tasks.len();
//      for index in 0..len {
//          if tasks[index].id().as_u64() == curr_id {
//              tasks.remove(index);
//              break;
//          }
//      }
//      drop(tasks);
//      #[cfg(feature = "signal")]
//      process.signal_modules.lock().remove(&curr_id);
//      drop(process);
//  }
//  RUN_QUEUE.lock().exit_current(exit_code);
// }

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

/// Current running task
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
