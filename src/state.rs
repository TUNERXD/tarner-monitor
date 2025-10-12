use crate::process::ProcessInfo;
use crate::system::SystemManager;
use iced::{Application, Command, Theme, time, Subscription}; 
use std::time::Duration;
use sysinfo::Pid;

pub struct TarnerMonitor {
    pub processes: Vec<ProcessInfo>,
    pub selected_process: Option<Pid>,
    pub search_str: String,  
    pub total_memory: u64,
    pub cpu_len: usize,
    system_manager: SystemManager,
    current_sort: SortBy,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SortBy {
    AlphaAsc,
    AlphaDesc,
    CpuAsc,
    CpuDesc,
    MemAsc,
    MemDesc,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProcessSelected(Pid),     
    SearchChanged(String),     
    KillProcess,
    RefreshProcesses,         
    SortAlpha,
    SortCpu,
    SortMem,
    RefreshTick(time::Instant),
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
            current_sort: SortBy::AlphaAsc,
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

    // Sorting Processes
    fn apply_sort(&mut self) {
        match self.current_sort {
            SortBy::AlphaAsc => {
                self.processes.sort_by(|a, b| a.name.cmp(&b.name));
            },
            SortBy::AlphaDesc => {
                self.processes.sort_by(|a, b| b.name.cmp(&a.name));
            },
            SortBy::CpuAsc => {
                self.processes.sort_by(|a, b| {
                    a.cpu_usage
                        .partial_cmp(&b.cpu_usage)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            },
            SortBy::CpuDesc => {
                self.processes.sort_by(|a, b| {
                    b.cpu_usage
                        .partial_cmp(&a.cpu_usage)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            },
            SortBy::MemAsc => {
                self.processes
                    .sort_by(|a, b| a.memory_usage.cmp(&b.memory_usage));
            },
            SortBy::MemDesc => {
                self.processes
                    .sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            }
        }
    }

    // Kill the parent of the instance
    pub fn kill_selected_parent(&mut self) -> bool {
        if let Some(process,) = self.system_manager.system.process(self.selected_process.unwrap()) {
            if let Some(parent_pid) = process.parent() {
                let success = self.system_manager.kill_process(parent_pid);
                if success {
                    self.selected_process = None;
                }
                self.refresh_processes();
                success
            }
            else {
                self.refresh_processes();
                false
            }
        }
        else {
            self.refresh_processes();
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
            },
            Message::SearchChanged(search) => {
                self.search_str = search;
            },
            Message::KillProcess => {
                self.kill_selected_parent();
            },
            Message::RefreshProcesses => {
                self.refresh_processes();
            },
            Message::SortAlpha => {
                if self.current_sort == SortBy::AlphaAsc {
                    self.current_sort = SortBy::AlphaDesc;
                } else {
                    self.current_sort = SortBy::AlphaAsc;
                }
                self.apply_sort();
            },
            Message::SortCpu => {
                if self.current_sort == SortBy::CpuAsc {
                    self.current_sort = SortBy::CpuDesc;
                } else {
                    self.current_sort = SortBy::CpuAsc;
                }
                self.apply_sort();
            },
            Message::SortMem => {
                if self.current_sort == SortBy::MemAsc {
                    self.current_sort = SortBy::MemDesc;
                } else {
                    self.current_sort = SortBy::MemAsc;
                }
                self.apply_sort();
            },
            Message::RefreshTick(_instant) => {
                self.refresh_processes();
                self.apply_sort();
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        crate::view::view(self)
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1))
            .map(Message::RefreshTick)
    }
}