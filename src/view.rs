use crate::state::{AppTheme, Message, Tab, TarnerMonitor, ToastType};
use iced::widget::{button, column, container, row, scrollable, text, text_input, Column};
use iced::{Element, Length, Theme, Alignment, Color};

pub fn view<'a>(state: &'a TarnerMonitor, theme: Theme) -> Element<'a, Message> {
    let tab_buttons = row![
        button("Processes")
            .on_press(Message::TabSelected(Tab::Processes))
            .style(if state.active_tab == Tab::Processes {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            }),
        button("System")
            .on_press(Message::TabSelected(Tab::System))
            .style(if state.active_tab == Tab::System {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            }),
        button("Settings")
            .on_press(Message::TabSelected(Tab::Settings))
            .style(if state.active_tab == Tab::Settings {
                iced::theme::Button::Primary
            } else {
                iced::theme::Button::Secondary
            })
    ]
    .spacing(5);

    // Choose content based on the active tab
    let tab_content = match state.active_tab {
        Tab::Processes => view_processes(state),
        Tab::System => view_system(state),
        Tab::Settings => view_settings(state, theme.clone()),
    };

    let main_layout = column![
        tab_buttons,
        tab_content,
    ]
    .spacing(10);

    let toast = if let Some((status, toast_type)) = &state.toast {

        let text_color = match toast_type {
            ToastType::Error => Color::from_rgb(0.8, 0.0, 0.0),
            ToastType::Success => Color::from_rgb(0.0, 0.7, 0.0), 
        };

        let toast_content = container(
            text(status).style(text_color)
        )
        .padding(10)
        .style(iced::theme::Container::Box);

        container(toast_content)
            .width(Length::Fill)
            .height(Length::Shrink)
            .align_x(iced::alignment::Horizontal::Center)
            .padding(5)

    } else {
        container(text("")).height(Length::Shrink)
    };

    let final_content = column![
        main_layout.height(Length::Fill),
        toast,
    ];

    container(final_content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .into()
}

pub fn view_processes<'a>(state: &'a TarnerMonitor) -> Element<'a, Message> {
    let search_input = text_input("Search processes...", &state.search_str)
        .on_input(Message::SearchChanged)
        .padding(10);

    let end_task_button = button("End Task (Del)")
        .on_press(Message::RequestKill)
        .style(iced::theme::Button::Destructive);

    let sort_buttons = row![
        button("Name").on_press(Message::SortAlpha),
        button("CPU").on_press(Message::SortCpu),
        button("Mem").on_press(Message::SortMem),
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

    let details_pane: Element<'a, Message> = if state.kill_confirm {

        if let Some(process) = &state.selected_process {
            let parent_pid_str = process.parent_pid.map_or_else(
                || "N/A".to_string(),
                |pid| pid.as_u32().to_string()
            );

            let confirm_content = column![
                text(format!("End parent process of '{}'?", process.name.to_string_lossy())).size(20),
                text(format!("This will kill the parent (PID: {}) of the selected process (PID: {}).", parent_pid_str, process.pid.as_u32())),
                row![
                    button("Yes, End Task")
                        .on_press(Message::ConfirmKill)
                        .style(iced::theme::Button::Destructive),
                    button("Cancel")
                        .on_press(Message::CancelKill)
                        .style(iced::theme::Button::Secondary)
                ].spacing(10)
            ]
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .align_items(Alignment::Center);

            container(confirm_content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        } else {
            let error_content = column![
                text("Error: No process selected for kill confirmation."),
                button("Cancel").on_press(Message::CancelKill)
            ].align_items(Alignment::Center);
            container(error_content).center_x().center_y().into()
        }

    } else if let Some(process) = &state.selected_process {

        let cpu_percent = process.cpu_usage / state.system_manager.cpu_cores as f32;
        let mem_percent = (process.memory_usage as f64 / state.system_manager.total_memory as f64) * 100.0;
        let parent_pid_str = process.parent_pid.map_or_else(
            || "N/A".to_string(),
            |pid| pid.as_u32().to_string()
        );

        let detail_row = |label: &str, value: String| {
            row![
                text(label).width(Length::FillPortion(1)),
                text(value).width(Length::FillPortion(1)),
            ]
            .spacing(10)
            .padding(2)
        };
        
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
        let cpu_percent = process.cpu_usage / state.system_manager.cpu_cores as f32;
        let mem_percent = (process.memory_usage as f64 / state.system_manager.total_memory as f64) * 100.0;

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


fn view_system<'a>(state: &'a TarnerMonitor) -> Element<'a, Message> {
    
    // Helper to create styled rows
    let detail_row = |label: &str, value: String| {
        row![
            text(label).width(Length::Fixed(150.0)),
            text(value),
        ]
        .spacing(10)
        .padding(2)
    };

    // Get system info from the system_manager
    let sys = &state.system_manager.system;
    
    let os_name = state.system_manager.os_name.to_string();
    let os_version = state.system_manager.os_version.to_string();
    let kernel = state.system_manager.kernel_version.to_string();
    let hostname = state.system_manager.hostname.to_string();
    let cpu_brand = state.system_manager.cpu_brand.to_string();
    let cpu_cores = state.system_manager.cpu_cores.to_string();
    
    // Convert memory from bytes to Megabytes (MB) for readability
    let total_mem_mb = state.system_manager.total_memory / 1024 / 1024;
    let used_mem_mb = sys.used_memory() / 1024 / 1024;

    let content = column![
        text("System Information").size(24),
        detail_row("OS:", os_name),
        detail_row("OS Version:", os_version),
        detail_row("Kernel Version:", kernel),
        detail_row("Hostname:", hostname),
        detail_row("CPU:", cpu_brand),
        detail_row("Logical Cores:", cpu_cores),
        detail_row("Total Memory:", format!("{} MB", total_mem_mb)),
        detail_row("Used Memory:", format!("{} MB", used_mem_mb)),
    ]
    .spacing(10)
    .padding(10);

    // Return as a scrollable container
    scrollable(content).height(Length::Fill).into()
}

fn view_settings<'a>(state: &'a TarnerMonitor, _theme: Theme) -> Element<'a, Message> {

    let theme_text = match state.theme {
        AppTheme::Light => "Dark Mode",
        AppTheme::Dark => "Light Mode",
    };

    let theme_toggle = button(theme_text)
        .on_press(Message::ToggleTheme)
        .style(iced::theme::Button::Secondary);

    let export_csv = button("Export to CSV")
        .on_press(Message::ExportToCsv)
        .style(iced::theme::Button::Positive);

    let reload_logs_button = button("Reload Logs")
        .on_press(Message::LoadLogs);

    let logs_title = row![
        text("Event Logs").size(20),
        reload_logs_button,
    ]
    .spacing(10)
    .align_items(Alignment::Center);

    let mut logs_column = Column::new().spacing(10);

    for line in state.log_lines.iter().rev() {
        // Simple parsing to color the logs
        let (log_text, log_color) = if line.contains("[ERROR]") {
            (line, iced::Color::from_rgb(0.8, 0.0, 0.0)) // Red
        } else if line.contains("[WARN]") {
            (line, iced::Color::from_rgb(0.9, 0.9, 0.0)) // Yellow
        } else {
            (line, iced::Color::from_rgb(0.7, 0.7, 0.7)) // gray
        };
        
        logs_column = logs_column.push(
            text(log_text).style(log_color)
        );
    }


    let logs_container = container(
        scrollable(logs_column)
            .width(Length::Fill)
            .height(Length::Fill)
    )
    .style(iced::theme::Container::Box)
    .padding(5);

    let content = column! [
        text("Settings").size(24),
        row![
            theme_toggle.padding(20),
            export_csv.padding(20),
        ].spacing(10).padding(20),
        logs_title,
        logs_container,

    ]
    .spacing(10)
    .padding(10)
    .height(Length::Fill);

    content.into()
}