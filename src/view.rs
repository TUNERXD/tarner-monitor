use crate::state::{Message, TarnerMonitor};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column, horizontal_space};
use iced::{Alignment, Element, Length, Theme, Settings};

pub fn view(state: &TarnerMonitor) -> Element<Message> {

    // for testing
        
    let header = container(
        row![
            text("TarnerMonitor"),
            horizontal_space(),
            button("Sort by Alpha").on_press(Message::SortAlpha),
            button("Sort CPU ascending").on_press(Message::SortCpuA),
        ]
        .spacing(10)
    )
    .padding(10);

    let sfr = container(
        row![
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