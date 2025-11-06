# Tarner Monitor Architecture

**This document outlines the software architecture of Tarner Monitor, a Rust-based system process manager using the Iced GUI framework.**

## 1. Core Design Pattern: Model-View-Update (MVU)

The application follows MVU pattern via the Iced framework, which separates concerns into three distinct components:
**Model (State)**
-Location: TarnerMonitor struct in src/state.rs
-Purpose: The single, mutable source of truth for the entire application.
-Key Contents: Holds the current process list, selected process, search string, system manager, sorting criteria, theme, and UI state (tabs, toasts, logs).

**View (UI Rendering)**
-Location: view() function in src/view.rs
-Purpose: Declarative, stateless UI generation.
-Behavior: Takes an immutable reference to the Model and produces the complete UI (Element<Message>).

**Update (State Transitions)**
-Location: update() function in src/state.rs
-Purpose: Handles all state mutations based on user actions or system events.
-Input: Message enum (user click, refresh tick, etc.).
-Output: Command<Message> for initiating asynchronous operations (e.g., file export, process kill).

---

## 2. Module Breakdown
The src/ directory defines clear responsibilities aligned with the MVU pattern and data handling:

**main.rs (Bootstrap / Entry Point)**
This file serves as the application's Entry Point. Its primary responsibility is initialization: setting up the logging system and launching the Iced application instance (TarnerMonitor::run_with_settings()).

**state.rs (Model & Update / Core Logic)**
As the Core Logic component, state.rs is responsible for State Management and the Update mechanism. It handles all incoming Message transitions, performs business logic like sorting and filtering, and manages settings persistence.

**view.rs (View / Presentation)**
This file is the Presentation layer and holds the View logic. It handles UI Construction using Iced widgets, rendering all tabs (Processes, System, Settings) and managing notification displays.

**process.rs (Data Abstraction / Data Model)**
This module acts as the Data Model. Its primary role is Data Abstraction, decoupling the application from the raw sysinfo library structures by defining the clean, internal ProcessInfo struct.

**system.rs (Data Source / System Abstraction)**
Serving as the System Abstraction layer and Data Source, system.rs wraps sysinfo::System. It caches static system information, provides data refresh functionality, and offers process management functions (e.g., kill_process).

**logger.rs (Utility / Logging Utility)**
This Utility file is the Logging Utility. It configures simplelog for dual logging (terminal + file) and handles platform-specific log path provision.

---

## 3. Data Flow

**A. Real-Time Refresh Flow (Subscription)**
This manages the process and system data updates every second.
-Subscription: state.rs creates an iced::time::every(Duration::from_secs(1)) subscription.
-Message: The timer emits Message::RefreshTick every second.
-Update: state.rs receives the tick, calls system.rs to refresh data via sysinfo, and replaces the process list in the Model.
-Validation: The selection is validated (if a process was killed, selected_process is set to None).
-Re-render: view.rs updates the process table with fresh data.

**B. User Command Flow (Example: CSV Export)**
Long-running tasks are offloaded using Command::perform.
-Message: User sends Message::ExportToCsv.
-Update: state.rs immediately returns a Command::perform that executes the export_action function asynchronously.
-Async Task: The export_action runs on a background thread, writes the data using the csv crate, and returns the result.
-Completion Message: The result is sent back as Message::ExportFinished(Result<String, String>).
-Final Update: state.rs receives ExportFinished and displays a success or error Toast Notification.

---

## 4. Core Dependencies

* `iced` (0.13.1): The cross-platform GUI framework that handles the View layer.
* `sysinfo` (0.32.1): Used for reliable cross-platform system and process information.
* `tokio` (1.0): The asynchronous runtime powering `iced` and background updates.
* `serde` (1.0) & `toml` (0.8): For configuration file handling and persistent settings.
* `csv` (1.3): Enables the CSV export functionality.
* `simplelog` (0.12) & `log` (0.4): Provides the logging infrastructure for events.

---

## 5. State Management
This section details the critical constraints and logic governing the central TarnerMonitor state in src/state.rs.

**Immutable State Access**
-View receives &TarnerMonitor (immutable borrow). This ensures no accidental state modification occurs during the rendering process.
-Predictability: The UI output is always a predictable consequence of the current state.

**Mutable State Updates**
-Single Mutator: Only the central update() function in state.rs is permitted to mutate the application state.
-Traceability: All mutations are explicit and traceable, enforcing Centralized State Management.

**State Validation**
Example: Process selection after refresh
```rust
pub fn refresh_processes(&mut self) {
    self.system_manager.refresh();
    self.processes = self.system_manager.get_processes();

    // Critical: Re-validate selected process
    if let Some(selected_proc) = &self.selected_process {
        let pid = selected_proc.pid;
        self.selected_process = self.processes
            .iter()
            .find(|p| p.pid == pid)
            .cloned(); // Updates data or sets None if terminated
    }
}
```

---

## 6. Asynchronous Operations

### Subscriptions
Iced subscriptions enable background tasks without blocking the UI:

```rust
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch(vec![
        // Timer: Refresh every 1 second
        iced::time::every(Duration::from_secs(1))
            .map(Message::RefreshTick),
        
        // Keyboard events: Delete key handling
        event::listen().map(Message::EventOccurred),
    ])
}
```

### Commands
Commands execute async operations and return messages:

```rust
// CSV Export
Command::perform(
    export_action(processes, cpu_cores, total_memory),
    Message::ExportFinished
)

// Toast Auto-hide
Command::perform(
    async { tokio::time::sleep(Duration::from_secs(3)).await },
    |_| Message::HideToast
)

// Log Loading
Command::perform(
    load_logs_action(),
    Message::LogsLoaded
)
```

**Design Benefit**: Long-running operations never block the UI thread.

---

## 7. Persistence (Settings & Logging)
This section covers how the application manages user preferences and tracks operational events.

-Settings Storage: User settings (currently only Theme: Light/Dark) are saved in a TOML configuration file in a platform-specific config directory.
-Load: Settings are loaded at startup. The Dark theme is used as the default fallback on error.
-Save: Settings are written when the user toggles the theme.
-Logging System: Logs are written to both the terminal and a persistent file (~/.config/TarnerMonitor/tarner-monitor.log) at the Info level.
-Log Levels: Used to distinguish events: info!() for normal operations, warn!() for kill requests, and error!() for failed operations.
-Viewing: Logs are displayed in the Settings tab with color-coding (Red for [ERROR], Yellow for [WARN], Gray for [INFO]).

---

## 8. Testing Architecture
A comprehensive testing suite ensures the reliability and stability of the application.

**Unit Tests (17 tests)**
Focus: Cover low-level component logic, including Data Model validation (ProcessInfo), System Manager initialization (CPU cores, total memory), Filtering logic, Sorting correctness, and State Management (Theme, Tabs).

**Integration Tests (6 tests)**
Focus: Cover high-level, end-to-end user scenarios and component interaction, including the Complete Monitoring Cycle, full-text Search and Filter functionality, Sorting verification, Process Selection/Details persistence, and Tab Navigation/System Information retrieval.

---

## 9. Directory Structure 

```
tarner-monitor/
├── Cargo.toml              # Dependencies, metadata
├── Cargo.lock              # Locked dependency versions
├── README.md               # Project overview
├── ARCHITECTURE.md         # This file
├── USER_GUIDE.md           # User documentation
├── TEST_PLAN.md            # Testing documentation
├── src/
│   ├── main.rs            # Entry point (7 lines)
│   ├── lib.rs             # Module exports (5 lines)
│   ├── state.rs           # State + Update logic (~480 lines)
│   ├── view.rs            # View rendering (~330 lines)
│   ├── process.rs         # ProcessInfo model (~30 lines)
│   ├── system.rs          # SystemManager (~70 lines)
│   └── logger.rs          # Logging setup (~40 lines)
└── tests/
    ├── unit_tests.rs      # 17 unit tests (~370 lines)
    └── integration_tests.rs # 6 integration tests (~270 lines)
```