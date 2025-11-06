mod state;
mod process;
mod system;
mod view;
mod logger;
use state::TarnerMonitor;

fn main() -> iced::Result {
    logger::init_logging().expect("Failed to initialize logger");
    TarnerMonitor::run_with_settings()
}