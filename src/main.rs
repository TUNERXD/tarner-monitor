use iced::{Element, Theme};
use iced::widget::{column, row, container, button, text, horizontal_space, text_input, combo_box};


fn main() -> iced::Result {
    iced::run("Tarner Monitor - Process Monitor & Manager", TarnerMonitor::update, TarnerMonitor::view)
}

struct TarnerMonitor;

impl Default for TarnerMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
enum Message {
    Test,
    TestInput(String),
}

struct Search {
    placeholder: String,
}

#[derive(Debug, Clone)]
struct Filter {
    option: combo_box::State<FilterOptions>,
}

#[derive(Debug, Clone)]
enum FilterOptions {
    One,
}

struct Refresh;

impl TarnerMonitor {

    fn new() -> Self {
        Self
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Test => println!("Test"),
            _ => println!("Test Input"),
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {

        let search = Search {
            placeholder: String::from("Type to search..."),
        };
        
        let header = container(
            row![
                text("TarnerMonitor"),
                horizontal_space(),
                button("Settings").on_press(Message::Test),
                button("Export").on_press(Message::Test),
            ]
            .spacing(10)
        )
        .padding(10);

        let sfr = container(
            row![
                horizontal_space(),
                text("Search: "),
                text_input("Search", &search.placeholder).on_input(move |input: String| Message::TestInput(input)),
                horizontal_space(),
                button("End Task").on_press(Message::Test),
                horizontal_space(),
                text("Filter: "),
                horizontal_space(),
                text("Refresh: "),
            ]
        )
        .padding(10);

        column![
            header,
            sfr
        ]
        .into()
    }
}



