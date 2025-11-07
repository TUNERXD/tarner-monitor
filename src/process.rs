use std::ffi::OsString;
use sysinfo::DiskUsage;
use sysinfo::Pid;
use sysinfo::ProcessStatus;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: OsString,
    pub parent_pid: Option<Pid>,
    pub pid: Pid,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub run_time: u64,
    pub status: ProcessStatus,
    pub acc_cpu_time: u64,
    pub disk_usage: DiskUsage,
}

impl ProcessInfo {
    pub fn new(
        name: OsString,
        parent_pid: Option<Pid>,
        pid: Pid,
        cpu_usage: f32,
        memory_usage: u64,
        run_time: u64,
        status: ProcessStatus,
        acc_cpu_time: u64,
        disk_usage: DiskUsage,
    ) -> Self {
        ProcessInfo {
            name,
            parent_pid,
            pid,
            cpu_usage,
            memory_usage,
            run_time,
            status,
            acc_cpu_time,
            disk_usage,
        }
    }
}
