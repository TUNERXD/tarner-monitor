use crate::process::ProcessInfo;
use sysinfo::{Pid, System};

pub struct SystemManager {
    pub system: System,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname: String,
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub total_memory: u64,

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

        let os_name = System::name().unwrap_or_else(|| String::from("N/A"));
        let os_version = System::os_version().unwrap_or_else(|| String::from("N/A"));
        let kernel_version = System::kernel_version().unwrap_or_else(|| String::from("N/A"));
        let hostname = System::host_name().unwrap_or_else(|| String::from("N/A"));
        let cpu_brand = system.cpus().first().map_or("N/A".to_string(), |cpu| cpu.brand().to_string());
        let cpu_cores = system.cpus().len();
        let total_memory = system.total_memory();

        SystemManager {
            system,
            os_name,
            os_version,
            kernel_version,
            hostname,
            cpu_brand,
            cpu_cores,
            total_memory,
        }

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
                process.run_time(),
                process.status(),
                process.accumulated_cpu_time(),
                process.disk_usage(),
            )
        }).collect() 
    }

    pub fn kill_process(&mut self, pid: Pid) -> bool {
        if let Some(process) = self.system.process(pid) {
            process.kill()
        } else {
            false
        }
    }
}