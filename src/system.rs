use crate::process::ProcessInfo;
use sysinfo::{Pid, System};

pub struct SystemManager {
    pub system: System,
}

impl SystemManager {

    pub fn new() -> Self {
        let mut system = System::new_all();

        //Initial Refresh
        system.refresh_all();

        // sleep for MINIMUM_CPU_UPDATE_INTERVAL
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        //second refresh for CPU usage
        system.refresh_all();

        SystemManager { system }

    }

    pub fn refresh(&mut self) {
        self.system.refresh_all();
    }

    pub fn get_processes (&self) -> Vec<ProcessInfo> {

        self.system.processes().iter().map(|(pid, process)| {
            ProcessInfo::new(
                process.name().to_os_string(),
                process.parent(),
                *pid,
                process.cpu_usage(),
                process.memory(),
            )
        }).collect() 
    }

    pub fn total_memory(&self) -> u64 {
        self.system.total_memory()
    }

    pub fn cpu_count(&self) -> usize {
        self.system.cpus().len()
    }

    pub fn kill_process(&mut self, pid: Pid) -> bool {
        if let Some(process) = self.system.process(pid) {
            process.kill()
        } else {
            false
        }
    }
}