# Tarner Monitor

**A real-time, cross-platform system and process monitoring utility built with Rust and Iced.**

## Features

### Process Management & Monitoring
**Real-time Updates:** Processes are monitored and updated every second.
**Comprehensive Details:** View essential process information including:
    * PID, Parent PID, Name, Status
    * CPU usage
    * Memory usage
    * Disk I/O
    * Runtime duration
**Terminate Processes:** Safely kill any selected process via a button or the $\text{\textlangle}\text{Delete}\text{\textrangle}$ key, complete with a confirmation dialog for safety.

### Search, Filter & Sort
**Real-time Search:** Instantly find processes by name.
**Smart Filtering:** The process list narrows down intelligently as you type.
**Flexible Sorting:** Toggle sorting by:
    * Alphabetical (Name)
    * CPU Usage (Highest to Lowest)
    * Memory Usage (Highest to Lowest)

### System Information
* OS Details: Displays OS name, version, and kernel version.
* Hardware Snapshot: View hostname, CPU brand and logical core count.
* Memory Overview: Total and used memory in MB.

### Settings & Customization
* Theming: Easily toggle between Light and Dark themes.
* Persistent Settings: Theme preference is saved automatically.
* Export: Export the full process details list to a CSV file.
* Event Logs: Color-coded severity levels for easy diagnosis of application events.

### User Interface (GUI)
* Intuitive Navigation: Simple Tab Navigation for Processes, System, and Settings views.
* Modern Design: A clean, modern, and responsive user interface.
* Feedback: Toast Notifications provide clear success/error feedback for user actions.
* Safety: Confirmation Dialogs ensure safe process termination.

---

## Requirements & Installation
* Rust: Version 1.70.0 or higher.
* Operating System: macOS, Windows, or Linux.
* Dependencies: All managed automatically by Cargo.

1.  Clone the repository:
    ```bash
    git clone [https://github.com/your-username/tarner-monitor.git](https://github.com/your-username/tarner-monitor.git)
    cd tarner-monitor
    ```
2.  Run the application:
    ```bash
    cargo run

---

## Project Structure & Architecture
tarner-monitor/
├── Cargo.toml              # Dependencies and metadata
├── Cargo.lock              # Locked dependency versions
├── README.md              
├── ARCHITECTURE.md         # Technical architecture documentation
├── USER_GUIDE.md           # Detailed user guide
├── TEST_PLAN.md            # Testing documentation
├── src/
│   ├── main.rs            # Application entry point
│   ├── lib.rs             # Library exports
│   ├── state.rs           # Application state and business logic
│   ├── view.rs            # UI rendering
│   ├── process.rs         # Process data model
│   ├── system.rs          # System information wrapper
│   └── logger.rs          # Logging configuration
└── tests/
    ├── unit_tests.rs      # Unit tests (17 tests)
    └── integration_tests.rs # Integration tests (6 tests)

### Core Dependencies
* `iced` (0.13.1): The cross-platform GUI framework that handles the View layer.
* `sysinfo` (0.32.1): Used for reliable cross-platform system and process information.
* `tokio` (1.0): The asynchronous runtime powering `iced` and background updates.
* `serde` (1.0) & `toml` (0.8): For configuration file handling and persistent settings.
* `csv` (1.3): Enables the CSV export functionality.
* `simplelog` (0.12) & `log` (0.4): Provides the logging infrastructure for events.