use crate::process::ProcessInfo;
use crate::system::SystemManager;
use iced::{Application, Command, Settings};
use sysinfo::Pid;
use std::io::{self, Write};

pub struct TarnerMonitor {
    pub processes: Vec<ProcessInfo>,
    pub selected_process: Option<Pid>,
    pub seach_str: String,
    pub total_memory: u64,
    pub cpu_len: usize,
    system_manager: SystemManager,
}

#[derive(Debug, Clone)]
pub enum Message{
    // ProcessSelected(Pid),
    // SearchChanged(String),
    // KillProcess,
    // RefreshProcesses,
    SortAlpha,
    SortCpuA,
    SortCpuD,
    SortMemA,
    SortMemD,
    Test,
}

impl TarnerMonitor {

    pub fn new() -> Self {
        let system_manager = SystemManager::new();
        let processes = system_manager.get_processes();

        TarnerMonitor {
            processes,
            selected_process: None,
            seach_str: String::new(),
            total_memory: system_manager.total_memory(),
            cpu_len: system_manager.cpu_count(),
            system_manager,
        }
    }

    // pub fn select_process(&mut self, process: ProcessInfo) {
    //     self.selected_process = Some(process.pid);
    // }

    pub fn get_filtered(&self) -> Vec<&ProcessInfo> {
        self.processes.iter().filter(|x| {
                x.name.to_string_lossy().to_lowercase().contains(&self.seach_str.to_lowercase())
            })
            .collect()
    }

}

impl Application for TarnerMonitor {

    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("TarnerMonitor - Process Manager")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::SortAlpha => self.processes.sort_by(|a, b| a.name.cmp(&b.name)),
            Message::SortCpuA => self.processes.sort_by(|a, b| a.cpu_usage.partial_cmp(&b.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            Message::SortMemA => self.processes.sort_by(|a, b| a.memory_usage.cmp(&b.memory_usage)),
            Message::SortCpuD => self.processes.sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap_or(std::cmp::Ordering::Equal)),
            Message::SortMemD => self.processes.sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage)),
            _ => { 
                for process in &self.processes {
                    println!("{:?}, CPU: {:.3}%, Memory: {:.3}%", process.name, process.cpu_usage / self.cpu_len as f32, (process.memory_usage as f64 / self.total_memory as f64) * 100.0)
                }
                io::stdout().flush().expect("Failed to flush")
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        crate::view::view(self)
    }
}