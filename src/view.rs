use crate::state::{Message, TarnerMonitor, AppTheme};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Element, Length, Theme};

// TODO: Refractor Code -> Selected Process: Option<ProcessInfo>
// TODO: Process Detail below
// TODO: Another window for Computer Detail
// TODO: Confirm when Kill process
// TODO: Export Processes to CSV

pub fn view<'a>(state: &'a TarnerMonitor, _theme: Theme) -> Element<'a, Message> {
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

    let theme_text = match state.theme {
        AppTheme::Light => "Dark Mode",
        AppTheme::Dark => "Light Mode",
    };

    let theme_toggle = button(theme_text)
        .on_press(Message::ToggleTheme)
        .style(iced::theme::Button::Secondary);


    let controls = row![search_input, end_task_button, sort_buttons, theme_toggle]
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

    let details_pane = column![
        row![
            text("Process Name").width(Length::FillPortion(1)),
            text("Name").width(Length::FillPortion(1)),
            text("Parent Pid").width(Length::FillPortion(1)),
            text("Pid").width(Length::FillPortion(1)),
        ],
        row![
            text("Cpu%").width(Length::FillPortion(1)),
            text("Cpu%").width(Length::FillPortion(1)),
        ],
        row![
            text("Memory Usage").width(Length::FillPortion(1)),
            text("Memory Usage").width(Length::FillPortion(1)),
        ],
        row![
            text("Disc Usage").width(Length::FillPortion(1)),
            text("Discs").width(Length::FillPortion(1)),
        ],
        row![
            text("Network Usage").width(Length::FillPortion(1)),
            text("Network").width(Length::FillPortion(1)),
        ],
    ];

    let filtered = state.get_filtered();
    let mut process_list = Column::new().spacing(2);

    for process in filtered {
        let cpu_percent = process.cpu_usage / state.cpu_len as f32;
        let mem_percent = (process.memory_usage as f64 / state.total_memory as f64) * 100.0;

        let is_selected = state.selected_process.as_ref().map(|p| p.pid) == Some(process.pid);

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
        details_pane,
    ]
    .spacing(10);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}