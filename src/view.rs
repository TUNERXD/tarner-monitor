use crate::state::{Message, TarnerMonitor};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Element, Length};

// TODO: Confirm when Kill process
// TODO: Save User Theme, Default Sort Preference
// TODO: Process Detail below
// TODO: Another window for Computer Detail
// TODO: Export Processes to CSV

pub fn view(state: &TarnerMonitor) -> Element<'_, Message> {
    let search_input = text_input("Search processes...", &state.search_str)
        .on_input(Message::SearchChanged)
        .padding(10);

    let end_task_button = button("End Task")
        .on_press(Message::KillProcess)
        .style(iced::theme::Button::Destructive);

    let sort_buttons = row![
        button("Name ↕").on_press(Message::SortAlpha),
        button("CPU ↕").on_press(Message::SortCpu),
        button("Mem ↕").on_press(Message::SortMem),
    ]
    .spacing(5);

    let controls = row![search_input, end_task_button, sort_buttons]
        .spacing(10)
        .padding(10);

    let header = row![
        text("Process Name").width(Length::FillPortion(3)),
        text("PID").width(Length::FillPortion(1)),
        text("CPU %").width(Length::FillPortion(1)),
        text("Memory %").width(Length::FillPortion(1)),
    ]
    .spacing(10)
    .padding(10);

    let filtered = state.get_filtered();
    let mut process_list = Column::new().spacing(2);

    for process in filtered {
        let cpu_percent = process.cpu_usage / state.cpu_len as f32;
        let mem_percent = (process.memory_usage as f64 / state.total_memory as f64) * 100.0;

        let is_selected = state.selected_process == Some(process.pid);

        let process_row = button(
            row![
                text(process.name.to_string_lossy()).width(Length::FillPortion(3)),
                text(format!("{}", process.pid.as_u32())).width(Length::FillPortion(1)),
                text(format!("{:.2}", cpu_percent)).width(Length::FillPortion(1)),
                text(format!("{:.2}", mem_percent)).width(Length::FillPortion(1)),
            ]
            .spacing(10)
            .padding(5),
        )
        .on_press(Message::ProcessSelected(process.pid))
        .style(if is_selected {
            iced::theme::Button::Primary
        } else {
            iced::theme::Button::Secondary
        })
        .width(Length::Fill);

        process_list = process_list.push(process_row);
    }

    let content = column![
        controls,
        header,
        scrollable(process_list).height(Length::Fill),
    ]
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}