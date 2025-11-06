use simplelog::{LevelFilter, WriteLogger, CombinedLogger, Config, TermLogger, TerminalMode, ColorChoice};
use std::fs::File;
use std::path::PathBuf;

pub fn get_log_path() -> Option<PathBuf> {
    dirs::config_dir().map(|mut path| {
        path.push("TarnerMonitor");
        path.push("tarner-monitor.log");
        path
    })
}

pub fn init_logging() -> Result<(), String> {
    let Some(log_path) = get_log_path() else {
        return Err("Could not find config directory to create log file.".to_string());
    };

    // Create the config directory if it doesn't exist
    if let Some(dir) = log_path.parent() {
        if !dir.exists() {
            std::fs::create_dir_all(dir)
                .map_err(|e| format!("Failed to create log directory: {}", e))?;
        }
    }

    // Create log file
    let log_file = File::create(&log_path)
        .map_err(|e| format!("Failed to create log file at {:?}: {}", log_path, e))?;

    // Configure the loggers
    CombinedLogger::init(vec![
        // Log to the terminal (for debugging with `cargo run`)
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // Log to the file
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            log_file,
        ),
    ])
    .map_err(|e| format!("Failed to initialize logger: {}", e))
}