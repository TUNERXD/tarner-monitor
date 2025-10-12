mod state;
mod process;
mod system;
mod view;
use state::TarnerMonitor;

fn main() -> iced::Result {
    TarnerMonitor::run_with_settings()
}