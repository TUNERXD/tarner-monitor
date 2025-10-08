mod state;
mod process;
mod system;
mod view;
use state::TarnerMonitor;
use iced::Application;

fn main() -> iced::Result {
    TarnerMonitor::run(iced::Settings::default())
}