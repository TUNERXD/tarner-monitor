# **Tarner Monitor Architecture**

This document outlines the software architecture of the Tarner Monitor, a Rust-based system process manager using the Iced GUI framework.

## **1\. Core Design Pattern: Model-View-Update (MVU)**

The application is built using the **Iced** framework, which follows **The Elm Architecture**, a Model-View-Update (MVU) pattern. This pattern separates the application's logic into three distinct parts:

* **Model:** The single source of truth for the application's state. In this project, this is the TarnerMonitor struct defined in src/state.rs. It holds all data, such as the list of processes, the currently selected process, the search string, and the current theme.  
* **View:** A declarative, stateless function that takes the current **Model** and produces the user interface. This is the view() function in src/view.rs. It's responsible for *displaying* the state but not for *changing* it.  
* **Update:** The logic that handles state transitions. This is implemented in the update() function within src/state.rs. It receives Message enums (which represent all possible user actions or events) and modifies the **Model** accordingly.

This architecture ensures a unidirectional data flow, making the application state predictable, easier to debug, and simple to reason about.

## **2\. Module Breakdown**

The project source code is organized into modules with distinct responsibilities:

* **src/main.rs:** The application's entry point. Its sole responsibility is to initialize the iced::Application (our TarnerMonitor struct) and run it with the necessary settings.  
* **src/state.rs:** The core of the application (the "Model" and "Update" components of MVU).  
  * TarnerMonitor (struct): The **Model**. Holds all application state.  
  * Message (enum): Defines all possible events that can change the state (e.g., ProcessSelected(Pid), SearchChanged(String), KillProcess, RefreshTick).  
  * impl Application for TarnerMonitor: Contains the main application logic:  
    * new(): Constructor that initializes the state, loads settings, and creates the SystemManager.  
    * update(): The **Update** logic. A match statement handles every Message and modifies the state.  
    * subscription(): Manages asynchronous background tasks, specifically the 1-second timer that emits the RefreshTick message.  
  * AppSettings (struct): Manages loading and saving user preferences (like the theme) to a .toml file.  
* **src/view.rs:** The presentation layer (the "View" component of MVU).  
  * view() (function): Takes an immutable reference to the TarnerMonitor state and builds the entire UI using iced widgets (row, column, button, text\_input, etc.).  
  * User interactions (e.g., on\_press, on\_input) are mapped to Message enums, which are sent to the update function.  
* **src/system.rs:** The system interaction abstraction layer.  
  * SystemManager (struct): A wrapper around the sysinfo::System object.  
  * This module isolates all direct calls to the sysinfo library. Its responsibilities include:  
    * Initializing the sysinfo object (including the double-refresh for accurate CPU data).  
    * Refreshing system data (refresh()).  
    * Fetching and mapping system processes to our internal ProcessInfo struct (get\_processes()).  
    * Executing system-level commands, like kill\_process().  
* **src/process.rs:** The internal data model.  
  * ProcessInfo (struct): Defines the application's internal representation of a process. This is a crucial design choice, as it decouples our application from the sysinfo library's data structures.

## **3\. Data Flow**

There are two primary data flows in the application: user-initiated actions and asynchronous updates.

### **A. User Interaction Flow (e.g., Kill Process)**

1. **View:** A user clicks the "End Task" button in the UI (src/view.rs).  
2. **Message:** The button's on\_press event handler dispatches a Message::KillProcess.  
3. **Update:** The Iced runtime delivers this message to the update() function in src/state.rs.  
4. **Logic:** The update() function matches on Message::KillProcess and calls self.kill\_selected\_parent().  
5. **Service:** This function, in turn, calls self.system\_manager.kill\_process(...) in src/system.rs.  
6. **System:** system.rs executes the actual system call using the sysinfo crate.  
7. **State Change:** The update() function modifies the TarnerMonitor state (e.g., self.selected\_process \= None and the process list is refreshed).  
8. **Re-render:** Iced detects the state change and calls view() to render a new UI reflecting the updated state.

### **B. Asynchronous Data Refresh Flow (Mitigating UI Freeze)**

This flow directly addresses the "UI Freeze" risk identified in the project proposal.

1. **Subscription:** On startup, the subscription() function in src/state.rs is called. It creates a background task: iced::time::every(Duration::from\_secs(1)).  
2. **Message:** This task emits a Message::RefreshTick every second, independently of the main UI thread.  
3. **Update:** The update() function receives this RefreshTick message.  
4. **Logic:** It calls self.refresh\_processes().  
5. **Service:** This function updates the SystemManager and fetches a new list of ProcessInfo objects.  
6. **State Change:** The self.processes vector in the TarnerMonitor state is replaced with the new data.  
7. **Re-render:** Iced detects the state change and calls view() to update the process list on-screen.

This model ensures that the main UI thread is *never* blocked by I/O or system calls, guaranteeing a responsive interface.

## **4\. Key Libraries and Dependencies**

* **iced:** The core GUI framework providing the MVU architecture, widgets, and asynchronous runtime (tokio feature).  
* **sysinfo:** The sole provider for all system-level information (processes, CPU, memory). Encapsulated entirely within src/system.rs.  
* **serde & toml:** Used for serializing (serde) the AppSettings struct into the .toml format for persistence.  
* **dirs:** A utility crate used to find the correct, cross-platform configuration directory to store the tarner\_monitor\_config.toml file.

## **5\. Persistence (Settings)**

Application settings (currently just the theme) are persisted via the AppSettings struct in src/state.rs.

* **On Load:** TarnerMonitor::new() calls AppSettings::load().  
* **Loading Logic:** AppSettings::load() attempts to read and parse tarner\_monitor\_config.toml from the user's config directory (via dirs::config\_dir()). If it fails (e.g., file not found), it returns AppSettings::default().  
* **On Save:** When the Message::ToggleTheme is handled in update(), the self.theme state is changed, a new AppSettings struct is created, and settings.save() is called.  
* **Saving Logic:** settings.save() serializes the struct to TOML and writes it to the config file.

## **6\. Directory Structure**

tarner-monitor/  
├── Cargo.toml      \# Defines dependencies (iced, sysinfo, serde, etc.)  
├── Cargo.lock      \# Locks dependency versions  
├── README.md  
├── ARCHITECTURE.md \# This file  
└── src/  
    ├── main.rs     \# Application entry point  
    ├── process.rs  \# Defines the ProcessInfo data model  
    ├── state.rs    \# Core state (Model) and logic (Update)  
    ├── system.rs   \# System interaction (sysinfo) wrapper  
    ├── test.rs     \# (Ignored, exploratory test file)  
    └── view.rs     \# UI rendering logic (View)  
