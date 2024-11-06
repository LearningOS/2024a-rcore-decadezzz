//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
        current_user_token,get_task_info,select_cur_task_to_mmap,select_cur_task_to_munmap
    },
    mm::page_table::translated_data_mut,
    timer::get_time_us,
};
///TimeVal
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    /// second and microsecond
    pub sec: usize,
    /// second and microsecond
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
/// inore _tz is timezone, we don't use it
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let ptr=translated_data_mut(current_user_token(),_ts);
    let us = get_time_us();
        *ptr = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    get_task_info(_ti);
    0
}

// YOUR JOB: Implement mmap.
///申请长度为 len 字节的物理内存（不要求实际物理内存位置，可以随便找一块）
/// 将其映射到 start 开始的虚存，内存页属性为 port
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _len == 0 {
        return 0;
    }
    if _port & !0x7 != 0 || _port & 0x7 == 0 {
        return -1;
    }
    select_cur_task_to_mmap(_start, _len, _port)
}

// YOUR JOB: Implement munmap.
////取消到 [start, start + len) 虚存的映射。特别地，在 rCore 课程实验中，
/// 正确执行的 sys_munmap 仅会对应 唯一且完整 的 mmap 区间，不考虑交叉、截断区间的情况。
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    select_cur_task_to_munmap(_start, _len)

}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
