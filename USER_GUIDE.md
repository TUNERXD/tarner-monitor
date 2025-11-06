# Tarner Monitor User Guide

First run Tarner Monitor:

```bash
cargo run 
```

You'll see:
1. The Processes tab (default view)
2. A list of running processes
3. Real-time updates every second
4. Dark theme (default)

## Configuration Files
Tarner Monitor automatically creates configuration directories:

**macOS:**
- Settings: `~/Library/Application Support/tarner_monitor_config.toml`
- Logs: `~/Library/Application Support/TarnerMonitor/tarner-monitor.log`

**Linux:**
- Settings: `~/.config/tarner_monitor_config.toml`
- Logs: `~/.config/TarnerMonitor/tarner-monitor.log`

**Windows:**
- Settings: `%APPDATA%\tarner_monitor_config.toml`
- Logs: `%APPDATA%\TarnerMonitor\tarner-monitor.log`

---

## User Interface Overview
The application has three main tabs:

1. Processes Tab
- View all running processes
- Search, sort, and manage processes
- See detailed information for selected processes
- Terminate processes

2. System Tab
- View operating system information
- Check CPU and memory statistics
- Monitor system resources

3. Settings Tab
- Toggle between Light and Dark themes
- Export process data to CSV
- View application event logs
- Reload logs

---

## Process Management

**Viewing Processes**
Process List Columns
- Process Name: Application or service name
- PID: Process ID (unique identifier)
- CPU%: CPU usage per core (e.g., 100% = 1 full core)
- Memory%: Percentage of total system memory

**Selecting a Process**
1. Click on any process in the list
2. The row highlights in blue
3. Details appear in the lower panel

**Process Details**
When a process is selected, you'll see:
| Field | Description |
|-------|-------------|
| **Name** | Full process name |
| **Status** | Run, Sleep, Idle, etc. |
| **PID** | Process ID |
| **Parent PID** | Parent process ID (or N/A) |
| **CPU %** | CPU usage per core |
| **Accumulated CPU Time** | Total CPU time in milliseconds |
| **Memory (bytes)** | Memory usage in bytes |
| **Memory %** | Percentage of total memory |
| **Read Bytes** | New/Total disk read bytes |
| **Written Bytes** | New/Total disk write bytes |
| **Runtime** | Process uptime in hours |

**Searching Processes**
The search feature helps you find specific processes quickly.

How to Search:
1. Click in the search box (top-left)
2. Type part of the process name
3. The list updates instantly

**Search Features:**
- Case-insensitive: "chrome" matches "Chrome"
- Partial matching: "chr" matches "chrome", "chromium", etc.
- Real-time: Updates as you type
- Clear search: Delete text to see all processes

**Sorting Processes**
Click the sort buttons to organize the process list:

Name Sorting
- First Click: A → Z (ascending)
- Second Click: Z → A (descending)

CPU Sorting
- First Click: Low → High CPU usage
- Second Click: High → Low CPU usage

Memory Sorting
- First Click: Low → High memory usage
- Second Click: High → Low memory usage

**Visual Indicator**: The current sort is applied immediately to the list.

*Terminating Processes*
Tarner Monitor allows you to terminate processes safely with confirmation.

Method 1: Using the Button
1. Select a process from the list
2. Click End Task (Del) button
3. Confirmation dialog appears
4. Click Yes, End Task or Cancel

Method 2: Using Keyboard Shortcut
1. Select a process
2. Press the Delete key
3. Confirm in the dialog

**Success/Failure Feedback:**
- Green Toast: "Successfully killed parent of [name]"
- Red Toast: "Failed to kill parent of [name]"

---

## Automatic Refresh
Processes update automatically every 1 second.

**What Updates:**
- Process list
- CPU usage
- Memory usage
- Disk I/O
- Runtime

**Selection Behavior:**
- If your selected process is still running: Details update
- If your selected process terminated: Selection clears

---

## System Information
Switch to the System tab to view hardware and OS details.

**Information Displayed**
Operating System
- OS: Operating system name (e.g., "macOS", "Linux", "Windows")
- OS Version: Version number (e.g., "14.2.1", "22.04")
- Kernel Version: Kernel version string
- Hostname: Computer name

CPU
- CPU: Brand and model (e.g., "Apple M1", "Intel Core i7-9700K")
- Logical Cores: Number of logical CPU cores

Memory
- Total Memory: Total RAM in MB
- Used Memory: Currently used RAM in MB


## Settings and Configuration
The Settings tab provides customization and data management.

**Theme Toggle**
Switch between Light and Dark themes.

## Export to CSV
Export current process data to a CSV file.

**CSV Contents:**
The exported file includes all visible processes with columns:
- PID
- Name
- Parent PID
- Status
- CPU % (per core)
- Memory % (of total)
- Memory (bytes)
- Disk Read (bytes)
- Disk Write (bytes)
- Runtime (seconds)

## Event Logs
View application activity logs with color-coded severity levels.

**Log Display:**
- Red: Errors (failed operations)
- Yellow: Warnings (kill requests)
- Gray: Info (normal operations)

**Log Events Include:**
- Application startup
- Process selection
- Theme changes
- Tab switches
- Kill requests (success/failure)
- Export operations (success/failure)
- Sort changes

---

## Glossary

- PID: Process ID - unique number assigned to each running process
- Parent PID: PID of the process that created this process
- CPU Usage: Percentage of CPU time used by the process
- Memory Usage: Amount of RAM used by the process
- Disk I/O: Input/Output operations (read/write) to disk
- Runtime: How long the process has been running
- Process Status: Current state (Run, Sleep, Idle, etc.)
- Toast: Temporary notification message that auto-hides
- MVU: Model-View-Update - the architecture pattern used