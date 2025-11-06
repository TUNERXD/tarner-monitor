use crate::process::ProcessInfo;
use crate::system::SystemManager;
use iced::{Application, Command, Theme, time, Subscription}; 
use std::time::Duration;
use sysinfo::Pid;
use serde::{Serialize, Deserialize};
use std::{fs, io};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    Dark,
}

impl From<AppTheme> for Theme {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
        }
    }
}

/// Structure to hold and manage application settings for persistence
#[derive(Serialize, Deserialize)]
struct AppSettings {
    theme: AppTheme,
}

impl Default for AppSettings {
    fn default() -> Self {
        AppSettings {
            theme: AppTheme::Dark, // Default to Dark theme
        }
    }
}

impl AppSettings {
    const CONFIG_FILE: &str = "tarner_monitor_config.toml";

    fn config_path() -> Option<PathBuf> {
        dirs::config_dir().map(|path| path.join(Self::CONFIG_FILE))
    }

    fn load() -> Self {
        if let Some(path) = Self::config_path() {
            match fs::read_to_string(path) {
                Ok(contents) => match toml::from_str(&contents) {
                    Ok(settings) => return settings,
                    Err(e) => {
                        eprintln!("Error parsing config file: {}", e);
                    }
                },
                Err(e) if e.kind() != io::ErrorKind::NotFound => {
                    eprintln!("Error reading config file: {}", e);
                }
                _ => {}
            }
        }
        Self::default()
    }

    fn save(&self) {
        if let Some(path) = Self::config_path() {
            let config_dir = path.parent().unwrap();
            if let Err(e) = fs::create_dir_all(config_dir) {
                eprintln!("Error creating config directory: {}", e);
                return;
            }
            match toml::to_string(self) {
                Ok(contents) => {
                    if let Err(e) = fs::write(path, contents) {
                        eprintln!("Error saving config file: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error serializing config: {}", e);
                }
            }
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    Processes,
    System,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    ProcessSelected(Pid),     
    SearchChanged(String),           
    SortAlpha,
    SortCpu,
    SortMem,
    RefreshTick(time::Instant),
    ToggleTheme,
    TabSelected(Tab),
    RequestKill,
    ConfirmKill,
    CancelKill,
    ExportToCsv,
    ExportFinished(Result<String, String>),
    HideToast,
}
pub struct TarnerMonitor {
    pub processes: Vec<ProcessInfo>,
    pub selected_process: Option<ProcessInfo>,
    pub search_str: String,  
    pub system_manager: SystemManager,
    current_sort: SortBy,
    pub theme: AppTheme,
    pub active_tab: Tab,
    pub kill_confirm: bool,
    pub toast: Option<String>,
    pub logs: Vec<String>,
}

impl TarnerMonitor {
    pub fn new() -> Self {
        let settings = AppSettings::load();
        let system_manager = SystemManager::new();
        let processes = system_manager.get_processes();

        let mut logs = Vec::new();
        logs.push("Application started".to_string());

        let mut app = TarnerMonitor {
            processes,
            selected_process: None,
            search_str: String::new(),
            system_manager,
            current_sort: SortBy::AlphaAsc,
            theme: settings.theme,
            active_tab: Tab::Processes,
            kill_confirm: false,
            toast: None,
            logs,
        };  

        app.apply_sort(); 
        app
    }

    fn push_log(&mut self, message: String) {
        self.logs.push(message);
        if self.logs.len() > 100 {
            self.logs.remove(0);
        }
    }

    // For searching processes
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

        if let Some(selected_proc) = &self.selected_process {
            let pid = selected_proc.pid;
            self.selected_process = self.processes
                .iter()
                .find(|p| p.pid == pid)
                .cloned();
        }
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
    pub fn kill_selected_parent(&mut self) -> (bool, String) {
        let target_process = self.selected_process.as_ref();
        
        if let Some(process) = target_process {
            let name = process.name.to_string_lossy().to_string();
            // Check if there is a parent PID
            if let Some(parent_pid) = process.parent_pid {
                // Attempt to kill the parent
                if self.system_manager.kill_process(parent_pid) {
                    return (true, name);
                }
            }
            return (false, name);
        }
        (false, "".to_string())
    }

    pub fn run_with_settings() -> iced::Result {
        let settings = iced::Settings::with_flags(());
        TarnerMonitor::run(settings)
    }
}

async fn export_action(
    processes: Vec<ProcessInfo>, 
    cpu_cores: usize, 
    total_memory: u64
) -> Result<String, String> {
    
    let Some(mut path) = dirs::download_dir() else {
        return Err("Could not find download directory.".to_string());
    };
    path.push("tarner_monitor_export.csv");
    let file_path_str = path.to_string_lossy().to_string();

    let mut writer = match csv::Writer::from_path(&path) {
        Ok(w) => w,
        Err(e) => return Err(format!("Failed to create file: {}", e)),
    };

    writer.write_record([
        "PID",
        "Name",
        "Parent PID",
        "Status",
        "CPU %",
        "Memory %",
        "Memory (bytes)",
        "Disk Read (bytes)",
        "Disk Write (bytes)",
        "Runtime (sec)",
    ]).map_err(|e| format!("Failed to write header: {}", e))?;

    for p in processes {
        let cpu_percent = p.cpu_usage / cpu_cores as f32;
        let mem_percent = (p.memory_usage as f64 / total_memory as f64) * 100.0;
        let parent_pid = p.parent_pid.map_or_else(|| "N/A".to_string(), |pid| pid.as_u32().to_string());
        
        writer.write_record([
            p.pid.as_u32().to_string(),
            p.name.to_string_lossy().to_string(),
            parent_pid,
            format!("{}", p.status),
            format!("{:.2}", cpu_percent),
            format!("{:.2}", mem_percent),
            p.memory_usage.to_string(),
            p.disk_usage.read_bytes.to_string(),
            p.disk_usage.written_bytes.to_string(),
            p.run_time.to_string(),
        ]).map_err(|e| format!("Failed to write record: {}", e))?;
    }

    writer.flush().map_err(|e| format!("Failed to flush CSV: {}", e))?;
    Ok(format!("Export successful to {}", file_path_str))
}

impl Application for TarnerMonitor {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self::new();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("TarnerMonitor - Process Manager")
    }

    fn theme(&self) -> Self::Theme {
        self.theme.into()
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::ProcessSelected(pid) => {
                self.selected_process = self.processes
                    .iter()
                    .find(|p| p.pid == pid)
                    .cloned();
                if self.selected_process.is_some() {
                    self.push_log(format!("selected {:?}", self.selected_process.as_ref().unwrap().name));
                }
                self.kill_confirm = false;
                
            },
            Message::SearchChanged(search) => {
                self.search_str = search;
                self.push_log(format!("Filter for {}", self.search_str));
            },
            Message::RequestKill => {
                if self.selected_process.is_some() {
                    self.kill_confirm = true;
                    self.push_log(format!("Requested Kill {:?}", self.selected_process.as_ref().unwrap().name));
                }
            },
            Message::ConfirmKill => {
                let (success, name) = self.kill_selected_parent();
                self.kill_confirm = false;
                self.selected_process = None;

                let msg = if success {
                    format!("Successfully killed parent of {}", name)
                } else {
                    format!("Failed to kill parent of {}", name)
                };
                
                self.toast = Some(msg.clone());
                self.push_log(msg);
                
                return Command::perform(
                    async { tokio::time::sleep(Duration::from_secs(3)).await },
                    |_| Message::HideToast,
                );
            },
            Message::CancelKill => {
                self.kill_confirm = false;
                self.push_log(String::from("Kill Canceled"));
            },
            Message::SortAlpha => {
                if self.current_sort == SortBy::AlphaAsc {
                    self.current_sort = SortBy::AlphaDesc;
                    self.push_log(String::from("Sort Alphabet Descending"));
                } else {
                    self.current_sort = SortBy::AlphaAsc;
                    self.push_log(String::from("Sort Alphabet Ascending"));
                }
                self.apply_sort();
            },
            Message::SortCpu => {
                if self.current_sort == SortBy::CpuAsc {
                    self.current_sort = SortBy::CpuDesc;
                    self.push_log(String::from("Sort Cpu Descending"));
                } else {
                    self.current_sort = SortBy::CpuAsc;
                    self.push_log(String::from("Sort Cpu Ascending"));
                }
                self.apply_sort();
            },
            Message::SortMem => {
                if self.current_sort == SortBy::MemAsc {
                    self.current_sort = SortBy::MemDesc;
                    self.push_log(String::from("Sort Memory Descending"));
                } else {
                    self.current_sort = SortBy::MemAsc;
                    self.push_log(String::from("Sort Memory Ascending"));
                }
                self.apply_sort();
            },
            Message::RefreshTick(_instant) => {
                self.refresh_processes();
                self.apply_sort();
            },
            Message::ToggleTheme => {
                self.theme = match self.theme {
                    AppTheme::Light => {
                        self.push_log(String::from("Change to Dark Theme"));
                        AppTheme::Dark
                    },
                    AppTheme::Dark => {
                        self.push_log(String::from("Change to Light Theme"));
                        AppTheme::Light
                    },
                };
                let settings = AppSettings { theme: self.theme };
                self.push_log(String::from("Theme Saved"));
                settings.save();
            },
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                self.push_log(String::from("Changed Tab"));
                
            },
            Message::ExportToCsv => {
                self.toast = Some("Exporting...".to_string());
                
                let processes_to_export: Vec<ProcessInfo> = self.get_filtered()
                    .into_iter()
                    .cloned()
                    .collect();
                
                let cpu_cores = self.system_manager.cpu_cores;
                let total_memory = self.system_manager.total_memory;
                
                return Command::perform(
                    export_action(processes_to_export, cpu_cores, total_memory), 
                    Message::ExportFinished
                );
            },
            Message::ExportFinished(result) => {
                match result {
                    Ok(success_message) => {
                        self.push_log(String::from("Export Success"));
                        self.toast = Some(success_message)
                    },
                    Err(error_message) => {
                        self.push_log(String::from("Export Failed"));
                        self.toast = Some(format!("Error: {}", error_message));
                    },
                }
                return Command::perform(
                    async { tokio::time::sleep(Duration::from_secs(3)).await },
                    |_| Message::HideToast,
                );
            },
            
            Message::HideToast => {
                self.toast = None;
                self.push_log(String::from("Toast Hidden"));
            }
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        crate::view::view(self, self.theme.into())
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::time::every(Duration::from_secs(1))
            .map(Message::RefreshTick)
    }
}