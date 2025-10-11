use crate::process::ProcessInfo;
use crate::system::SystemManager;
use iced::{Application, Command, Theme}; 
use sysinfo::Pid;

pub struct TarnerMonitor {
    pub processes: Vec<ProcessInfo>,
    pub selected_process: Option<Pid>,
    pub search_str: String,  
    pub total_memory: u64,
    pub cpu_len: usize,
    system_manager: SystemManager,  
}

#[derive(Debug, Clone)]
pub enum Message {
    ProcessSelected(Pid),     
    SearchChanged(String),     
    KillProcess,             
    RefreshProcesses,         
    SortAlpha,
    SortCpuA,
    SortCpuD,
    SortMemA,
    SortMemD,
}

impl TarnerMonitor {
    pub fn new() -> Self {
        let system_manager = SystemManager::new();
        let processes = system_manager.get_processes();

        TarnerMonitor {
            processes,
            selected_process: None,
            search_str: String::new(),
            total_memory: system_manager.total_memory(),
            cpu_len: system_manager.cpu_count(),
            system_manager,
        }
    }

    pub fn get_filtered(&self) -> Vec<&ProcessInfo> {
        self.processes
            .iter()
            .filter(|x| {
                if self.search_str.is_empty() {
                    true
                } else {
                    x.name
                        .to_string_lossy()
                        .to_lowercase()
                        .contains(&self.search_str.to_lowercase())
                }
            })
            .collect()
    }

    pub fn refresh_processes(&mut self) {
        self.system_manager.refresh();
        self.processes = self.system_manager.get_processes();
        self.cpu_len = self.system_manager.cpu_count();
        self.total_memory = self.system_manager.total_memory();
    }

    pub fn kill_selected(&mut self) -> bool {
        if let Some(pid) = self.selected_process {
            let success = self.system_manager.kill_process(pid);
            if success {
                self.selected_process = None;
            }
            success
        } else {
            false
        }
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
            Message::ProcessSelected(pid) => {
                self.selected_process = Some(pid);
            }
            Message::SearchChanged(search) => {
                self.search_str = search;
            }
            Message::KillProcess => {
                if self.kill_selected() {
                    self.refresh_processes();
                }
            }
            Message::RefreshProcesses => {
                self.refresh_processes();
            }
            Message::SortAlpha => {
                self.processes.sort_by(|a, b| a.name.cmp(&b.name));
            }
            Message::SortCpuA => {
                self.processes.sort_by(|a, b| {
                    a.cpu_usage
                        .partial_cmp(&b.cpu_usage)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            Message::SortCpuD => {
                self.processes.sort_by(|a, b| {
                    b.cpu_usage
                        .partial_cmp(&a.cpu_usage)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            Message::SortMemA => {
                self.processes
                    .sort_by(|a, b| a.memory_usage.cmp(&b.memory_usage));
            }
            Message::SortMemD => {
                self.processes
                    .sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        crate::view::view(self)
    }
}