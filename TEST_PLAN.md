# Tarner Monitor Test Plan

## Testing Overview
Tarner Monitor follows a comprehensive testing approach that includes:
- Unit Tests: Testing individual components in isolation
- Integration Tests: Testing component interactions and workflows
- Manual Tests: UI/UX validation that can't be automated

**Testing Framework**
- Test Runner: Cargo's built-in test framework
- Assertions: Standard Rust `assert!`, `assert_eq!`, `assert_ne!`
- Mocking: Minimal (real system calls for integration tests)

---

## Test Strategy

**What We Test**
Data Models - ProcessInfo creation and validation  
System Information - SystemManager initialization  
Filtering Logic - Process search and filtering  
Sorting Logic - All sort modes (Alpha, CPU, Memory)  
State Management - Theme, tabs, selection  
Bug Fixes - Regression tests for fixed bugs  
Complete Workflows - End-to-end process monitoring  
Search and Filter - Real-time filtering behavior  
System Info Retrieval - Live system data accuracy  

---

## Unit Tests
Location: `tests/unit_tests.rs` (17 tests)

---

## Integration Tests
Location: `tests/integration_tests.rs` (6 tests)

---

## Manual Testing Checklist
These tests cannot be automated and require human interaction.

---

## Test Results

```bash
$ cargo test

running 23 tests
test unit_tests::test_case_insensitive_search ... ok
test unit_tests::test_cpu_core_count_realistic ... ok
test unit_tests::test_cpu_usage_within_bounds ... ok
test unit_tests::test_memory_usage_non_negative ... ok
test unit_tests::test_process_filtering_empty_search ... ok
test unit_tests::test_process_filtering_with_search ... ok
test unit_tests::test_process_info_creation ... ok
test unit_tests::test_process_name_not_empty ... ok
test unit_tests::test_process_pid_positive ... ok
test unit_tests::test_process_selection_persistence_after_refresh ... ok
test unit_tests::test_runtime_non_negative ... ok
test unit_tests::test_sort_alphabetically_ascending ... ok
test unit_tests::test_sort_by_cpu_descending ... ok
test unit_tests::test_sort_by_memory_descending ... ok
test unit_tests::test_system_manager_initialization ... ok
test unit_tests::test_tab_selection ... ok
test unit_tests::test_theme_toggle ... ok
test unit_tests::test_total_memory_validation ... ok
test integration_tests::test_complete_monitoring_cycle ... ok
test integration_tests::test_process_search_and_filter ... ok
test integration_tests::test_process_selection_and_details ... ok
test integration_tests::test_sorting_functionality ... ok
test integration_tests::test_system_information_retrieval ... ok
test integration_tests::test_tab_navigation ... ok

test result: ok. 23 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```