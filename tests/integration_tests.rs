// use tarner_monitor::process::ProcessInfo;
use tarner_monitor::state::{TarnerMonitor, Tab};
use tarner_monitor::system::SystemManager;
use std::thread;
use std::time::Duration;

// ========== Integration Test 1: Complete Monitoring Cycle ==========
#[test]
fn test_complete_monitoring_cycle() {
    println!("🔄 Starting complete monitoring cycle test...");
    
    let mut monitor = TarnerMonitor::new();
    
    // Initial state
    let initial_count = monitor.processes.len();
    println!("Initial process count: {}", initial_count);
    assert!(initial_count > 0, "No processes detected");
    
    // Get system info
    let cpu_cores = monitor.system_manager.cpu_cores;
    let total_memory = monitor.system_manager.total_memory;
    
    println!("CPU Cores: {}", cpu_cores);
    println!("Total Memory: {} MB", total_memory / 1024 / 1024);
    
    assert!(cpu_cores > 0);
    assert!(total_memory > 0);
    
    // Wait and refresh
    thread::sleep(Duration::from_secs(2));
    monitor.refresh_processes();
    
    let after_refresh_count = monitor.processes.len();
    println!("After refresh process count: {}", after_refresh_count);
    
    // Process count should be similar (within reasonable range)
    let difference = if after_refresh_count > initial_count {
        after_refresh_count - initial_count
    } else {
        initial_count - after_refresh_count
    };
    
    assert!(
        difference < 50,
        "Process count changed drastically: {} -> {}",
        initial_count,
        after_refresh_count
    );
    
    println!("✅ Complete monitoring cycle test passed!");
}

// ========== Integration Test 2: Process Search and Filter ==========
#[test]
fn test_process_search_and_filter() {
    println!("🔍 Testing process search and filter...");
    
    let mut monitor = TarnerMonitor::new();
    monitor.refresh_processes();
    
    let total_processes = monitor.processes.len();
    println!("Total processes: {}", total_processes);
    
    // Test empty search (should return all)
    monitor.search_str = String::new();
    let all_filtered = monitor.get_filtered();
    assert_eq!(all_filtered.len(), total_processes);
    println!("✓ Empty search returns all {} processes", all_filtered.len());
    
    // Test specific search
    if let Some(first_process) = monitor.processes.first() {
        let process_name = first_process.name.to_string_lossy().to_string();
        let search_term = process_name.chars().take(3).collect::<String>();
        
        monitor.search_str = search_term.clone();
        let filtered = monitor.get_filtered();
        
        println!("Searching for '{}': found {} processes", search_term, filtered.len());
        assert!(filtered.len() > 0, "Search should find at least one process");
        
        // Verify all filtered items contain search term
        for process in filtered {
            let name_lower = process.name.to_string_lossy().to_lowercase();
            assert!(
                name_lower.contains(&search_term.to_lowercase()),
                "Process '{}' doesn't contain search term '{}'",
                name_lower,
                search_term
            );
        }
        println!("✓ All filtered processes contain search term");
    }
    
    println!("✅ Process search and filter test passed!");
}

// ========== Integration Test 3: Sorting Functionality ==========
#[test]
fn test_sorting_functionality() {
    println!("📊 Testing sorting functionality...");
    
    let mut monitor = TarnerMonitor::new();
    monitor.refresh_processes();
    
    if monitor.processes.len() < 3 {
        println!("⚠️  Not enough processes to test sorting");
        return;
    }
    
    // Test CPU sorting
    monitor.current_sort = tarner_monitor::state::SortBy::CpuDesc;
    monitor.apply_sort();
    
    let first_cpu = monitor.processes[0].cpu_usage;
    let last_cpu = monitor.processes[monitor.processes.len() - 1].cpu_usage;
    
    assert!(
        first_cpu >= last_cpu,
        "CPU sorting failed: first ({}) < last ({})",
        first_cpu,
        last_cpu
    );
    println!("✓ CPU sorting works: {} >= {}", first_cpu, last_cpu);
    
    // Test Memory sorting
    monitor.current_sort = tarner_monitor::state::SortBy::MemDesc;
    monitor.apply_sort();
    
    let first_mem = monitor.processes[0].memory_usage;
    let last_mem = monitor.processes[monitor.processes.len() - 1].memory_usage;
    
    assert!(
        first_mem >= last_mem,
        "Memory sorting failed: first ({}) < last ({})",
        first_mem,
        last_mem
    );
    println!("✓ Memory sorting works: {} >= {}", first_mem, last_mem);
    
    // Test Alphabetical sorting
    monitor.current_sort = tarner_monitor::state::SortBy::AlphaAsc;
    monitor.apply_sort();
    
    let first_name = monitor.processes[0].name.to_string_lossy();
    let second_name = monitor.processes[1].name.to_string_lossy();
    
    assert!(
        first_name <= second_name,
        "Alphabetical sorting failed: '{}' > '{}'",
        first_name,
        second_name
    );
    println!("✓ Alphabetical sorting works: '{}' <= '{}'", first_name, second_name);
    
    println!("✅ Sorting functionality test passed!");
}

// ========== Integration Test 4: Process Selection and Details ==========
#[test]
fn test_process_selection_and_details() {
    println!("🎯 Testing process selection and details...");
    
    let mut monitor = TarnerMonitor::new();
    monitor.refresh_processes();
    
    if monitor.processes.is_empty() {
        println!("⚠️  No processes available");
        return;
    }
    
    // Select first process
    let first_pid = monitor.processes[0].pid;
    monitor.selected_process = monitor.processes
        .iter()
        .find(|p| p.pid == first_pid)
        .cloned();
    
    assert!(monitor.selected_process.is_some());
    println!("✓ Process selected: {:?}", 
             monitor.selected_process.as_ref().unwrap().name.to_string_lossy());
    
    // Verify selected process details
    let selected = monitor.selected_process.as_ref().unwrap();
    assert!(selected.pid.as_u32() > 0);
    assert!(!selected.name.is_empty());
    assert!(selected.memory_usage >= 0);
    
    println!("  PID: {}", selected.pid.as_u32());
    println!("  CPU: {:.2}%", selected.cpu_usage);
    println!("  Memory: {} bytes", selected.memory_usage);
    
    // Test refresh with selection
    thread::sleep(Duration::from_millis(500));
    monitor.refresh_processes();
    
    // Selection should persist or be cleared if process ended
    if let Some(still_selected) = &monitor.selected_process {
        println!("✓ Selection persisted after refresh");
        assert_eq!(still_selected.pid, first_pid);
    } else {
        println!("✓ Selection cleared (process may have ended)");
    }
    
    println!("✅ Process selection and details test passed!");
}

// ========== Integration Test 5: Tab Navigation ==========
#[test]
fn test_tab_navigation() {
    println!("🔀 Testing tab navigation...");
    
    let mut monitor = TarnerMonitor::new();
    
    // Start on Processes tab
    assert_eq!(monitor.active_tab, Tab::Processes);
    println!("✓ Started on Processes tab");
    
    // Navigate to System tab
    monitor.active_tab = Tab::System;
    assert_eq!(monitor.active_tab, Tab::System);
    println!("✓ Navigated to System tab");
    
    // Navigate to Settings tab
    monitor.active_tab = Tab::Settings;
    assert_eq!(monitor.active_tab, Tab::Settings);
    println!("✓ Navigated to Settings tab");
    
    // Navigate back to Processes
    monitor.active_tab = Tab::Processes;
    assert_eq!(monitor.active_tab, Tab::Processes);
    println!("✓ Navigated back to Processes tab");
    
    println!("✅ Tab navigation test passed!");
}

// ========== Integration Test 6: System Information Retrieval ==========
#[test]
fn test_system_information_retrieval() {
    println!("💻 Testing system information retrieval...");
    
    let system_manager = SystemManager::new();
    
    // Verify OS info
    assert!(!system_manager.os_name.is_empty());
    assert!(!system_manager.os_version.is_empty());
    assert!(!system_manager.kernel_version.is_empty());
    assert!(!system_manager.hostname.is_empty());
    
    println!("✓ OS Name: {}", system_manager.os_name);
    println!("✓ OS Version: {}", system_manager.os_version);
    println!("✓ Kernel: {}", system_manager.kernel_version);
    println!("✓ Hostname: {}", system_manager.hostname);
    
    // Verify CPU info
    assert!(!system_manager.cpu_brand.is_empty());
    assert!(system_manager.cpu_cores > 0);
    
    println!("✓ CPU: {}", system_manager.cpu_brand);
    println!("✓ Cores: {}", system_manager.cpu_cores);
    
    // Verify memory info
    assert!(system_manager.total_memory > 0);
    let used_memory = system_manager.system.used_memory();
    assert!(used_memory > 0);
    assert!(used_memory <= system_manager.total_memory);
    
    println!("✓ Total Memory: {} MB", system_manager.total_memory / 1024 / 1024);
    println!("✓ Used Memory: {} MB", used_memory / 1024 / 1024);
    
    println!("✅ System information retrieval test passed!");
}