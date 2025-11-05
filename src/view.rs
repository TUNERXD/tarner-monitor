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
        button("Name").on_press(Message::SortAlpha),
        button("CPU").on_press(Message::SortCpu),
        button("Mem").on_press(Message::SortMem),
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

    let details_pane: Element<'a, Message> = if let Some(process) = &state.selected_process {
        // Calculate percentages using state totals
        let cpu_percent = process.cpu_usage / state.cpu_len as f32;
        let mem_percent = (process.memory_usage as f64 / state.total_memory as f64) * 100.0;
        let parent_pid_str = process.parent_pid.map_or_else(
            || "N/A".to_string(), // Handle processes with no parent
            |pid| pid.as_u32().to_string()
        );

        // Helper closure to create a detail row
        let detail_row = |label: &str, value: String| {
            row![
                text(label).width(Length::FillPortion(1)),
                text(value).width(Length::FillPortion(1)),
            ]
            .spacing(10)
            .padding(2)
        };

        // Build the column with process details
        let details_column = column![
            text("Process Details").size(20),
            row![
                text("Name: ").width(Length::FillPortion(1)),
                text(process.name.to_string_lossy().to_string()).width(Length::FillPortion(3)),
            ]
            .spacing(10)
            .padding(2),
            row![
                detail_row("Status:", format!("{}", process.status)).width(Length::FillPortion(1)),
                detail_row("Runtime(h):", format!("{}", process.run_time / 360)).width(Length::FillPortion(1)),  
            ],
            row![
                detail_row("PID:", process.pid.as_u32().to_string()).width(Length::FillPortion(1)),
                detail_row("Parent PID:", parent_pid_str).width(Length::FillPortion(1)),
            ],
            row![
                detail_row("CPU %:", format!("{:.2}", cpu_percent)).width(Length::FillPortion(1)),
                detail_row("Acc CPU time(ms):", format!("{}", process.acc_cpu_time)).width(Length::FillPortion(1)),
            ],
            row![
                detail_row("Memory (bytes):", format!("{}", process.memory_usage)).width(Length::FillPortion(1)),
                detail_row("Memory %:", format!("{:.2}", mem_percent)).width(Length::FillPortion(1)),
                
            ],
            row![
                detail_row("read bytes: new/total:", format!("{}/{}", process.disk_usage.read_bytes, process.disk_usage.total_read_bytes)).width(Length::FillPortion(1)),
                detail_row("written bytes: new/total:", format!("{}/{}", process.disk_usage.written_bytes, process.disk_usage.total_written_bytes)).width(Length::FillPortion(1)),
                
            ],
        ]
        .spacing(5)
        .padding(10)
        .width(Length::Fill);

        details_column.into()
    } else {
        text("").into()
    };

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