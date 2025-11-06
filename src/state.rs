use crate::process::ProcessInfo;
use crate::system::SystemManager;
use crate::logger;
use iced::{Application, Command, Theme, time, Subscription, keyboard, event, Event};
use std::time::Duration;
use sysinfo::Pid;
use serde::{Serialize, Deserialize};
use std::{fs, io};
use std::path::PathBuf;
use log::{info, warn, error};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastType {
    Success,
    Error,
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
    LoadLogs,
    LogsLoaded(Result<Vec<String>, String>),
    EventOccurred(iced::Event),
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
    pub toast: Option<(String, ToastType)>,
    pub log_lines: Vec<String>,
}

impl TarnerMonitor {
    pub fn new() -> Self {
        let settings = AppSettings::load();
        let system_manager = SystemManager::new();
        let processes = system_manager.get_processes();

        info!("Application started");

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
            log_lines: Vec::new(),
        };  

        app.apply_sort(); 
        app
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

async fn load_logs_action() -> Result<Vec<String>, String> {
    let Some(log_path) = logger::get_log_path() else {
        return Err("Log path not found.".to_string());
    };
    
    fs::read_to_string(log_path)
        .map(|contents| contents.lines().map(String::from).collect())
        .map_err(|e| format!("Failed to read log file: {}", e))
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
                    info!("Selected process: {:?}", self.selected_process.as_ref().unwrap().name);
                }
                self.kill_confirm = false;
                
            },
            Message::SearchChanged(search) => {
                self.search_str = search;
                info!("Set process filter to: {}", self.search_str);
            },
            Message::RequestKill => {
                if self.selected_process.is_some() {
                    self.kill_confirm = true;
                    warn!("Kill requested for: {:?}", self.selected_process.as_ref().unwrap().name);
                }
            },
            Message::ConfirmKill => {
                let (success, name) = self.kill_selected_parent();
                self.kill_confirm = false;
                self.selected_process = None;

                let (msg, style) = if success {
                    let msg = format!("Successfully killed parent of {}", name);
                    (msg, ToastType::Success)
                } else {
                    let msg = format!("Failed to kill parent of {}", name);
                    (msg, ToastType::Error)
                };
                
                if success {
                    info!("{}", msg);
                }
                else {
                    error!("{}", msg)
                }

                self.toast = Some((msg, style));
                
                return Command::perform(
                    async { tokio::time::sleep(Duration::from_secs(3)).await },
                    |_| Message::HideToast,
                );
            },
            Message::CancelKill => {
                self.kill_confirm = false;
                info!("Kill canceled");
            },
            Message::SortAlpha => {
                if self.current_sort == SortBy::AlphaAsc {
                    self.current_sort = SortBy::AlphaDesc;
                    info!("Sort Alphabet Descending");
                } else {
                    self.current_sort = SortBy::AlphaAsc;
                    info!("Sort Alphabet Ascending");
                }
                self.apply_sort();
            },
            Message::SortCpu => {
                if self.current_sort == SortBy::CpuAsc {
                    self.current_sort = SortBy::CpuDesc;
                    info!("Sort Cpu Descending");
                } else {
                    self.current_sort = SortBy::CpuAsc;
                    info!("Sort Cpu Ascending");
                }
                self.apply_sort();
            },
            Message::SortMem => {
                if self.current_sort == SortBy::MemAsc {
                    self.current_sort = SortBy::MemDesc;
                    info!("Sort Memory Descending");
                } else {
                    self.current_sort = SortBy::MemAsc;
                    info!("Sort Memory Ascending");
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
                        info!("Changed to Dark Theme");
                        AppTheme::Dark
                    },
                    AppTheme::Dark => {
                        info!("Changed to Light Theme");
                        AppTheme::Light
                    },
                };
                let settings = AppSettings { theme: self.theme };
                settings.save();
            },
            Message::TabSelected(tab) => {
                self.active_tab = tab;
                info!("Changed Tab to {:?}", tab);

                if tab == Tab::Settings {
                    return Command::perform(load_logs_action(), Message::LogsLoaded);
                }
                
            },
            Message::ExportToCsv => {
                self.toast = Some(("Exporting...".to_string(), ToastType::Success));
                info!("Exporting to CSV...");
                
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
                let (msg, style) = match result {
                    Ok(success_message) => {
                        info!("Export Success: {}", success_message);
                        (success_message, ToastType::Success)
                    },
                    Err(error_message) => {
                        error!("Export Failed: {}", error_message);
                        (format!("Error: {}", error_message), ToastType::Error)
                    },
                };
                self.toast = Some((msg, style));

                return Command::perform(
                    async { tokio::time::sleep(Duration::from_secs(3)).await },
                    |_| Message::HideToast,
                );
            },
            Message::HideToast => {
                self.toast = None;
            },
            Message::LoadLogs => {
                self.log_lines = vec!["Loading logs...".to_string()];
                return Command::perform(load_logs_action(), Message::LogsLoaded);
            },
            Message::LogsLoaded(result) => {
                match result {
                    Ok(lines) => {
                        info!("Successfully loaded {} log lines", lines.len());
                        self.log_lines = lines;
                    },
                    Err(e) => {
                        error!("Failed to load logs for view: {}", e);
                        self.log_lines = vec![e];
                    }
                }
            },
            Message::EventOccurred(event) => {
                // Filter for *only* the event we care about
                if let Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) = event {
                    // Check if the key is 'Delete'
                    match key {
                        keyboard::Key::Named(keyboard::key::Named::Delete) => {
                            // Check our app's state
                            if self.active_tab == Tab::Processes 
                                && !self.kill_confirm 
                                && self.selected_process.is_some() 
                            {
                                // Trigger the kill confirmation
                                self.kill_confirm = true;
                                warn!("Kill requested for: {:?}", self.selected_process.as_ref().unwrap().name);
                            }
                        },
                        _ => {}
                    }
                }
                return Command::none();
            },
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        crate::view::view(self, self.theme.into())
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::time::every(Duration::from_secs(1))
                .map(Message::RefreshTick),
            
            event::listen().map(Message::EventOccurred),
        ])
    }
}