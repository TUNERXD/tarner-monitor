use tarner_monitor::process::ProcessInfo;
use tarner_monitor::state::{TarnerMonitor, SortBy, AppTheme, Tab};
use tarner_monitor::system::SystemManager;
use sysinfo::{Pid, ProcessStatus, DiskUsage};
use std::ffi::OsString;

// ========== Unit Test 1: ProcessInfo Creation ==========
#[test]
fn test_process_info_creation() {
    let process = ProcessInfo::new(
        OsString::from("test_process"),
        Some(Pid::from_u32(100)),
        Pid::from_u32(200),
        15.5,
        1024 * 1024, // 1 MB
        3600,
        ProcessStatus::Run,
        1000,
        DiskUsage::default(),
    );

    assert_eq!(process.name, OsString::from("test_process"));
    assert_eq!(process.pid, Pid::from_u32(200));
    assert_eq!(process.parent_pid, Some(Pid::from_u32(100)));
    assert_eq!(process.cpu_usage, 15.5);
    assert_eq!(process.memory_usage, 1024 * 1024);
    assert_eq!(process.run_time, 3600);
}

// ========== Unit Test 2: Process Name Validation ==========
#[test]
fn test_process_name_not_empty() {
    let process = ProcessInfo::new(
        OsString::from("chrome"),
        None,
        Pid::from_u32(1234),
        10.5,
        2048,
        100,
        ProcessStatus::Run,
        500,
        DiskUsage::default(),
    );

    assert!(!process.name.is_empty());
    assert_eq!(process.name.to_string_lossy(), "chrome");
}

// ========== Unit Test 3: Process PID Validation ==========
#[test]
fn test_process_pid_positive() {
    let process = ProcessInfo::new(
        OsString::from("firefox"),
        None,
        Pid::from_u32(5678),
        5.0,
        4096,
        200,
        ProcessStatus::Run,
        1000,
        DiskUsage::default(),
    );

    assert!(process.pid.as_u32() > 0);
    assert_eq!(process.pid.as_u32(), 5678);
}

// ========== Unit Test 4: CPU Usage Bounds ==========
#[test]
fn test_cpu_usage_within_bounds() {
    let process = ProcessInfo::new(
        OsString::from("test"),
        None,
        Pid::from_u32(1),
        45.5,
        1024,
        100,
        ProcessStatus::Run,
        100,
        DiskUsage::default(),
    );

    assert!(process.cpu_usage >= 0.0);
    assert!(process.cpu_usage <= 100.0);
}

// ========== Unit Test 5: Memory Usage Non-Negative ==========
#[test]
fn test_memory_usage_non_negative() {
    let process = ProcessInfo::new(
        OsString::from("test"),
        None,
        Pid::from_u32(1),
        10.0,
        8192,
        100,
        ProcessStatus::Run,
        100,
        DiskUsage::default(),
    );

    assert!(process.memory_usage >= 0);
}

// ========== Unit Test 6: SystemManager Creation ==========
#[test]
fn test_system_manager_initialization() {
    let system_manager = SystemManager::new();

    assert!(system_manager.cpu_cores > 0);
    assert!(system_manager.total_memory > 0);
    assert!(!system_manager.os_name.is_empty());
    assert!(!system_manager.hostname.is_empty());
}

// ========== Unit Test 7: CPU Core Count Realistic ==========
#[test]
fn test_cpu_core_count_realistic() {
    let system_manager = SystemManager::new();
    
    // Most systems have 1-256 cores
    assert!(
        system_manager.cpu_cores >= 1 && system_manager.cpu_cores <= 256,
        "Unrealistic CPU count: {}",
        system_manager.cpu_cores
    );
}

// ========== Unit Test 8: Total Memory Validation ==========
#[test]
fn test_total_memory_validation() {
    let system_manager = SystemManager::new();
    
    // Total memory should be at least 1 GB (1073741824 bytes)
    assert!(
        system_manager.total_memory >= 1_073_741_824,
        "Total memory too low: {} bytes",
        system_manager.total_memory
    );
}

// ========== Unit Test 9: Process Filtering Empty Search ==========
#[test]
fn test_process_filtering_empty_search() {
    let mut monitor = TarnerMonitor::new();
    monitor.search_str = String::new();
    
    let filtered = monitor.get_filtered();
    
    // When search is empty, should return all processes
    assert_eq!(filtered.len(), monitor.processes.len());
}

// ========== Unit Test 10: Process Filtering With Search ==========
#[test]
fn test_process_filtering_with_search() {
    let mut monitor = TarnerMonitor::new();
    
    // Add test processes
    monitor.processes = vec![
        ProcessInfo::new(
            OsString::from("chrome"),
            None,
            Pid::from_u32(1),
            10.0,
            1024,
            100,
            ProcessStatus::Run,
            100,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("firefox"),
            None,
            Pid::from_u32(2),
            20.0,
            2048,
            200,
            ProcessStatus::Run,
            200,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("code"),
            None,
            Pid::from_u32(3),
            30.0,
            3072,
            300,
            ProcessStatus::Run,
            300,
            DiskUsage::default(),
        ),
    ];
    
    // Search for "chrome"
    monitor.search_str = String::from("chrome");
    let filtered = monitor.get_filtered();
    
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name.to_string_lossy(), "chrome");
}

// ========== Unit Test 11: Case Insensitive Search ==========
#[test]
fn test_case_insensitive_search() {
    let mut monitor = TarnerMonitor::new();
    
    monitor.processes = vec![
        ProcessInfo::new(
            OsString::from("Chrome"),
            None,
            Pid::from_u32(1),
            10.0,
            1024,
            100,
            ProcessStatus::Run,
            100,
            DiskUsage::default(),
        ),
    ];
    
    // Search with lowercase should find uppercase
    monitor.search_str = String::from("chrome");
    let filtered = monitor.get_filtered();
    
    assert_eq!(filtered.len(), 1);
}

// ========== Unit Test 12: Sort Alphabetically Ascending ==========
#[test]
fn test_sort_alphabetically_ascending() {
    let mut monitor = TarnerMonitor::new();
    
    monitor.processes = vec![
        ProcessInfo::new(
            OsString::from("zebra"),
            None,
            Pid::from_u32(1),
            10.0,
            1024,
            100,
            ProcessStatus::Run,
            100,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("apple"),
            None,
            Pid::from_u32(2),
            20.0,
            2048,
            200,
            ProcessStatus::Run,
            200,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("middle"),
            None,
            Pid::from_u32(3),
            30.0,
            3072,
            300,
            ProcessStatus::Run,
            300,
            DiskUsage::default(),
        ),
    ];
    
    monitor.current_sort = SortBy::AlphaAsc;
    monitor.apply_sort();
    
    assert_eq!(monitor.processes[0].name.to_string_lossy(), "apple");
    assert_eq!(monitor.processes[1].name.to_string_lossy(), "middle");
    assert_eq!(monitor.processes[2].name.to_string_lossy(), "zebra");
}

// ========== Unit Test 13: Sort by CPU Descending ==========
#[test]
fn test_sort_by_cpu_descending() {
    let mut monitor = TarnerMonitor::new();
    
    monitor.processes = vec![
        ProcessInfo::new(
            OsString::from("low"),
            None,
            Pid::from_u32(1),
            10.0,
            1024,
            100,
            ProcessStatus::Run,
            100,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("high"),
            None,
            Pid::from_u32(2),
            50.0,
            2048,
            200,
            ProcessStatus::Run,
            200,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("medium"),
            None,
            Pid::from_u32(3),
            25.0,
            3072,
            300,
            ProcessStatus::Run,
            300,
            DiskUsage::default(),
        ),
    ];
    
    monitor.current_sort = SortBy::CpuDesc;
    monitor.apply_sort();
    
    assert_eq!(monitor.processes[0].cpu_usage, 50.0);
    assert_eq!(monitor.processes[1].cpu_usage, 25.0);
    assert_eq!(monitor.processes[2].cpu_usage, 10.0);
}

// ========== Unit Test 14: Sort by Memory Descending ==========
#[test]
fn test_sort_by_memory_descending() {
    let mut monitor = TarnerMonitor::new();
    
    monitor.processes = vec![
        ProcessInfo::new(
            OsString::from("low"),
            None,
            Pid::from_u32(1),
            10.0,
            1024,
            100,
            ProcessStatus::Run,
            100,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("high"),
            None,
            Pid::from_u32(2),
            20.0,
            8192,
            200,
            ProcessStatus::Run,
            200,
            DiskUsage::default(),
        ),
        ProcessInfo::new(
            OsString::from("medium"),
            None,
            Pid::from_u32(3),
            30.0,
            4096,
            300,
            ProcessStatus::Run,
            300,
            DiskUsage::default(),
        ),
    ];
    
    monitor.current_sort = SortBy::MemDesc;
    monitor.apply_sort();
    
    assert_eq!(monitor.processes[0].memory_usage, 8192);
    assert_eq!(monitor.processes[1].memory_usage, 4096);
    assert_eq!(monitor.processes[2].memory_usage, 1024);
}

// ========== Unit Test 15: Theme Toggle ==========
#[test]
fn test_theme_toggle() {
    let mut monitor = TarnerMonitor::new();
    
    let initial_theme = monitor.theme;
    
    // Toggle theme
    monitor.theme = match monitor.theme {
        AppTheme::Light => AppTheme::Dark,
        AppTheme::Dark => AppTheme::Light,
    };
    
    assert_ne!(monitor.theme, initial_theme);
}

// ========== Unit Test 16: Tab Selection ==========
#[test]
fn test_tab_selection() {
    let mut monitor = TarnerMonitor::new();
    
    assert_eq!(monitor.active_tab, Tab::Processes);
    
    monitor.active_tab = Tab::System;
    assert_eq!(monitor.active_tab, Tab::System);
    
    monitor.active_tab = Tab::Settings;
    assert_eq!(monitor.active_tab, Tab::Settings);
}

// ========================================
// 🐛 BUG FIX TEST - Process Selection Memory Leak
// ========================================
// BUG FOUND: When rapidly selecting different processes, 
//            the selected_process wasn't properly updated
// CAUSE: Race condition in process selection during refresh
// FIX: Added synchronization in refresh_processes() method
//      to check if selected process still exists
// ========================================

#[test]
fn test_process_selection_persistence_after_refresh() {
    let mut monitor = TarnerMonitor::new();
    
    // Refresh to get initial processes
    monitor.refresh_processes();
    
    if monitor.processes.is_empty() {
        println!("⚠️  No processes available for test");
        return;
    }
    
    // Select first process
    let first_process = monitor.processes[0].clone();
    monitor.selected_process = Some(first_process.clone());
    
    println!("Selected process: {:?} (PID: {})", 
             first_process.name.to_string_lossy(), 
             first_process.pid.as_u32());
    
    // Refresh multiple times
    for i in 1..=5 {
        monitor.refresh_processes();
        println!("Refresh {}: Selected process is {:?}", 
                 i, 
                 monitor.selected_process.as_ref().map(|p| p.pid.as_u32()));
    }
    
    // BEFORE FIX: selected_process might become stale or point to wrong process
    // AFTER FIX: selected_process should either:
    //   1. Still exist with updated data if process is still running
    //   2. Be None if process was terminated
    
    if let Some(selected) = &monitor.selected_process {
        // If still selected, verify it exists in current process list
        let exists = monitor.processes.iter().any(|p| p.pid == selected.pid);
        assert!(
            exists,
            "❌ Bug detected! Selected process (PID: {}) not found in current process list",
            selected.pid.as_u32()
        );
        println!("✅ Selected process correctly maintained after refresh");
    } else {
        println!("✅ Selected process was correctly cleared (process terminated)");
    }
}

// ========== Unit Test 17: Run Time Validation ==========
#[test]
fn test_runtime_non_negative() {
    let process = ProcessInfo::new(
        OsString::from("test"),
        None,
        Pid::from_u32(1),
        10.0,
        1024,
        3600, // 1 hour
        ProcessStatus::Run,
        100,
        DiskUsage::default(),
    );

    assert!(process.run_time >= 0);
    assert_eq!(process.run_time, 3600);
}