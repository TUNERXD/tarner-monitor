use std::ffi::OsString;
use sysinfo::{ Pid, System};
use std::thread;

//TODO: Kill selected process + confirm
 
struct TarnerMonitor {
    processes: Vec<ProcessInfo>,
    selected_process: Option<Pid>,
    seach_str: String,
    total_memory: u64,
    cpu_len: usize,
}

impl TarnerMonitor {

    fn new(
        selected_process: Option<Pid>,
        seach_str: String,
        total_memory: u64,
        cpu_len: usize,
    ) -> Self {

        TarnerMonitor {
            processes: Vec::new(),
            selected_process,
            seach_str,
            total_memory,
            cpu_len,
        }
    }

    fn add(&mut self, process: ProcessInfo) {
        self.processes.push(process);
    }

    fn select_process(&mut self, process: ProcessInfo) {
        self.selected_process = Some(process.pid);
    }

    fn get_filtered(&self, filter: &str) -> Vec<&ProcessInfo> {
        self.processes
            .iter()
            .filter(|x| {
                x.name
                    .to_string_lossy()
                    .to_lowercase()
                    .contains(&filter.to_lowercase())
            })
            .collect()
    }

}
#[derive(Clone)]
struct ProcessInfo {
    name: OsString,
    parent_pid: Option<Pid>,
    pid: Pid,
    cpu_usage: f32,
    memory_usage: u64,
}

#[derive(Debug)]
enum Message{
    // ProcessSelected(Pid),
    // SearchChanged(String),
    // KillProcess,
    // RefreshProcesses,
    SortAlpha,
    SortCpuA,
    SortCpuD,
    SortMemA,
    SortMemD,
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPU usage to get actual value.
    sys.refresh_all();
    

    let mut tarnermonitor = TarnerMonitor::new(
        None,
        String::new(),
        sys.total_memory(),
        sys.cpus().len()
    );

    for (pid, process) in sys.processes() {
        tarnermonitor.add(ProcessInfo { name: process.name().to_os_string(), parent_pid: process.parent(), pid: *pid, cpu_usage: process.cpu_usage(), memory_usage: process.memory() });
    }

    let search_term = "ope";
    let mut filtered_processes = tarnermonitor.get_filtered(search_term);

    let filt = Message::SortAlpha;
    sys.refresh_cpu_all();
    match filt {
        Message::SortAlpha => filtered_processes.sort_by(|a, b| a.name.cmp(&b.name)),
        Message::SortCpuA => filtered_processes.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
        Message::SortMemA => filtered_processes.sort_by(|a, b| a.memory_usage.cmp(&b.memory_usage)),
        Message::SortCpuD => filtered_processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
        Message::SortMemD => filtered_processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage)),
    }

    for process in filtered_processes {
        println!("{:?}, CPU: {:.3}%, Memory: {:.3}%", process.name, process.cpu_usage / tarnermonitor.cpu_len as f32, (process.memory_usage as f64 / tarnermonitor.total_memory as f64) * 100.0)
    }
}