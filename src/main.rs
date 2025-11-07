mod logger;
mod process;
mod state;
mod system;
mod view;
use state::TarnerMonitor;

fn main() -> iced::Result {
    logger::init_logging().expect("Failed to initialize logger");
    TarnerMonitor::run_with_settings()
}
