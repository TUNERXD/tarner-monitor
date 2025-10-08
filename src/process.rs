use sysinfo::Pid;
use std::ffi::OsString;

#[derive(Clone, Debug)]
pub struct ProcessInfo {
    pub name: OsString,
    pub parent_pid: Option<Pid>,
    pub pid: Pid,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

impl ProcessInfo {
    pub fn new(
        name: OsString,
        parent_pid: Option<Pid>,
        pid: Pid,
        cpu_usage: f32,
        memory_usage: u64,
    ) -> Self {
        ProcessInfo {
            name,
            parent_pid,
            pid,
            cpu_usage,
            memory_usage,
        }
    }
}