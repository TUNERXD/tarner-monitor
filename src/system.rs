use crate::process::ProcessInfo;
use sysinfo::{System};

pub struct SystemManager {
    system: System,
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
}