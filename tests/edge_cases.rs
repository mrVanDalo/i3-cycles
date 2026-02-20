use i3_cycles::state::{CycleData, Cycles};
use std::time::Instant;

// ============================================================================
// EDGE-01: Special Character Workspace Names
// ============================================================================

#[test]
fn test_special_character_workspace_names_emoji() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("emoji_test".to_string());

    // Test emoji workspace names
    cycles.toggle("🚀".to_string());
    cycles.toggle("💻".to_string());
    cycles.toggle("🎯".to_string());

    let _snapshot = insta::Settings::new().bind_to_scope();

    // Verify toggle operations work
    let next1 = cycles.next();
    insta::assert_json_snapshot!("edge_01_emoji_first", next1);

    let next2 = cycles.next();
    insta::assert_json_snapshot!("edge_01_emoji_second", next2);

    // Verify serialization preserves emoji
    let json = cycles.to_json();
    let deserialized: Cycles = serde_json::from_str(&json).unwrap();
    assert!(deserialized
        .cycle_data
        .get("emoji_test")
        .unwrap()
        .contains("🚀"));
    assert!(deserialized
        .cycle_data
        .get("emoji_test")
        .unwrap()
        .contains("💻"));
}

#[test]
fn test_special_character_workspace_names_unicode() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("unicode_test".to_string());

    // Test unicode workspace names from various languages
    cycles.toggle("日本語".to_string()); // Japanese
    cycles.toggle("café".to_string()); // French with accent
    cycles.toggle("naïve".to_string()); // with diaeresis
    cycles.toggle("中文".to_string()); // Chinese
    cycles.toggle("العربية".to_string()); // Arabic
    cycles.toggle("Ελληνικά".to_string()); // Greek

    let _snapshot = insta::Settings::new().bind_to_scope();

    // Verify operations work with unicode
    let next1 = cycles.next();
    insta::assert_json_snapshot!("edge_01_unicode_first", next1);

    // Verify all workspaces present
    let cycle_data = cycles.cycle_data.get("unicode_test").unwrap();
    assert_eq!(cycle_data.len(), 6);
}

#[test]
fn test_special_character_workspace_names_symbols() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("symbols_test".to_string());

    // Test various symbols commonly used in workspace names
    cycles.toggle("work@home".to_string());
    cycles.toggle("test#1".to_string());
    cycles.toggle("project[v2]".to_string());
    cycles.toggle("a/b/c".to_string());
    cycles.toggle("space here".to_string());
    cycles.toggle("dots...here".to_string());

    let _snapshot = insta::Settings::new().bind_to_scope();

    // Verify operations with symbols
    let next1 = cycles.next();
    insta::assert_json_snapshot!("edge_01_symbols_first", next1);

    // Test removal with exact match (symbols must match exactly)
    let removed = cycles.toggle("work@home".to_string());
    assert!(!removed); // returns false when removing

    let still_has = cycles
        .cycle_data
        .get("symbols_test")
        .unwrap()
        .contains("work@home");
    assert!(!still_has);
}

#[test]
fn test_special_character_workspace_names_control_chars() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("control_test".to_string());

    // Test workspace names with characters that need JSON escaping
    cycles.toggle("with\ttab".to_string());
    cycles.toggle("with\nnewline".to_string());
    cycles.toggle("with\\backslash".to_string());
    cycles.toggle("with\"quote\"".to_string());

    let _snapshot = insta::Settings::new().bind_to_scope();

    // Verify serialization handles escaping
    let json = cycles.to_json();
    insta::assert_snapshot!("edge_01_control_json", json);

    // Verify deserialization preserves original values
    let deserialized: Cycles = serde_json::from_str(&json).unwrap();
    let cycle_data = deserialized.cycle_data.get("control_test").unwrap();
    assert!(cycle_data.contains("with\ttab"));
    assert!(cycle_data.contains("with\nnewline"));
}

// ============================================================================
// EDGE-02: Very Long Workspace Names
// ============================================================================

#[test]
fn test_very_long_workspace_names() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("long_names".to_string());

    // Test with progressively longer names
    let name_100 = "a".repeat(100);
    let name_1000 = "b".repeat(1000);
    let name_10000 = "c".repeat(10000);

    // Measure time for 100 char name
    let start = Instant::now();
    cycles.toggle(name_100.clone());
    let time_100 = start.elapsed();
    assert!(
        time_100.as_millis() < 100,
        "100 char name toggle took too long"
    );

    // Measure time for 1000 char name
    let start = Instant::now();
    cycles.toggle(name_1000.clone());
    let time_1000 = start.elapsed();
    assert!(
        time_1000.as_millis() < 100,
        "1000 char name toggle took too long"
    );

    // Measure time for 10000 char name
    let start = Instant::now();
    cycles.toggle(name_10000.clone());
    let time_10000 = start.elapsed();
    // Very long names might take longer, but should complete without crash
    assert!(
        time_10000.as_secs() < 5,
        "10000 char name toggle took too long"
    );

    // Verify all names are stored correctly
    let cycle_data = cycles.cycle_data.get("long_names").unwrap();
    assert!(cycle_data.contains(&name_100));
    assert!(cycle_data.contains(&name_1000));
    assert!(cycle_data.contains(&name_10000));

    // Verify serialization preserves full names
    let json = cycles.to_json();
    assert!(json.contains(&name_100));
    assert!(json.contains(&name_1000));

    // Verify deserialization
    let deserialized: Cycles = serde_json::from_str(&json).unwrap();
    let cycle_data = deserialized.cycle_data.get("long_names").unwrap();
    assert!(cycle_data.contains(&name_100));
    assert!(cycle_data.contains(&name_1000));

    // Test next() with long names
    let start = Instant::now();
    let next_result = cycles.next();
    let next_time = start.elapsed();
    assert!(
        next_time.as_millis() < 100,
        "next() with long names took too long"
    );
    assert!(next_result.is_some());
}

#[test]
fn test_long_name_position_tracking() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("long_pos".to_string());

    // Add workspaces with long names and cycle through them
    for i in 0..5 {
        cycles.toggle(format!("workspace_{}_{}", i, "x".repeat(100)));
    }

    // Cycle through all workspaces
    for i in 0..5 {
        let start = Instant::now();
        let next = cycles.next();
        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 50,
            "Position tracking with long names too slow"
        );
        assert!(next.is_some());
    }

    // Verify position wraps correctly
    let cycle_data = cycles.cycle_data.get("long_pos").unwrap();
    assert_eq!(cycle_data.position, 0); // After 5 next() calls on 5 workspaces
}

// ============================================================================
// EDGE-03: Many Workspaces Performance
// ============================================================================

#[test]
fn test_many_workspaces_performance_100() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("stress_100".to_string());

    // Add 100 workspaces
    let start = Instant::now();
    for i in 0..100 {
        cycles.toggle(format!("workspace_{}", i));
    }
    let add_time = start.elapsed();
    assert!(
        add_time.as_millis() < 500,
        "Adding 100 workspaces took too long"
    );

    // Test toggle on first, middle, last
    let start = Instant::now();
    cycles.toggle("workspace_0".to_string()); // remove first
    cycles.toggle("workspace_50".to_string()); // remove middle
    cycles.toggle("workspace_99".to_string()); // remove last
    let toggle_time = start.elapsed();
    assert!(
        toggle_time.as_millis() < 100,
        "Toggle operations took too long"
    );

    // Test next() performance
    let start = Instant::now();
    for _ in 0..10 {
        cycles.next();
    }
    let next_time = start.elapsed();
    assert!(
        next_time.as_millis() < 50,
        "next() operations took too long"
    );

    // Test list_cycles
    let start = Instant::now();
    let _cycles_list = cycles.list_cycles();
    let list_time = start.elapsed();
    assert!(list_time.as_millis() < 10, "list_cycles took too long");

    // Verify position tracking correct
    let cycle_data = cycles.cycle_data.get("stress_100").unwrap();
    assert_eq!(cycle_data.len(), 97); // 100 - 3 removed
}

#[test]
fn test_many_workspaces_performance_1000() {
    let mut cycles = Cycles::new();
    cycles.select_cycle("stress_1000".to_string());

    // Add 1000 workspaces
    let start = Instant::now();
    for i in 0..1000 {
        cycles.toggle(format!("ws_{:04}", i));
    }
    let add_time = start.elapsed();
    assert!(
        add_time.as_millis() < 2000,
        "Adding 1000 workspaces took too long"
    );

    // Test toggle on first, middle, last
    let start = Instant::now();
    cycles.toggle("ws_0000".to_string()); // remove first
    cycles.toggle("ws_0500".to_string()); // remove middle
    cycles.toggle("ws_0999".to_string()); // remove last
    let toggle_time = start.elapsed();
    assert!(
        toggle_time.as_millis() < 500,
        "Toggle operations took too long"
    );

    // Test next() performance (should be O(1) with VecDeque)
    let start = Instant::now();
    for _ in 0..100 {
        cycles.next();
    }
    let next_time = start.elapsed();
    assert!(
        next_time.as_millis() < 100,
        "100 next() operations took too long"
    );

    // Test state serialization
    let start = Instant::now();
    let json = cycles.to_json();
    let json_time = start.elapsed();
    assert!(
        json_time.as_millis() < 500,
        "JSON serialization took too long"
    );

    // Verify deserialization works
    let start = Instant::now();
    let deserialized: Cycles = serde_json::from_str(&json).unwrap();
    let deserialize_time = start.elapsed();
    assert!(
        deserialize_time.as_millis() < 500,
        "JSON deserialization took too long"
    );

    // Verify data integrity
    let cycle_data = deserialized.cycle_data.get("stress_1000").unwrap();
    assert_eq!(cycle_data.len(), 997); // 1000 - 3 removed
}

// ============================================================================
// EDGE-04: Many Cycles Performance
// ============================================================================

#[test]
fn test_many_cycles_performance_100() {
    let mut cycles = Cycles::new();

    // Create 100 cycles with 10 workspaces each
    let start = Instant::now();
    for i in 0..100 {
        let cycle_name = format!("cycle_{:03}", i);
        cycles.select_cycle(cycle_name.clone());

        for j in 0..10 {
            cycles.toggle(format!("ws_{:03}_{}", i, j));
        }
    }
    let create_time = start.elapsed();
    assert!(
        create_time.as_millis() < 3000,
        "Creating 100 cycles took too long"
    );

    // Test select_cycle performance (first, middle, last)
    let start = Instant::now();
    cycles.select_cycle("cycle_000".to_string());
    cycles.select_cycle("cycle_050".to_string());
    cycles.select_cycle("cycle_099".to_string());
    let select_time = start.elapsed();
    assert!(select_time.as_millis() < 100, "select_cycle took too long");

    // Test list_cycles sorting
    let start = Instant::now();
    let cycle_list = cycles.list_cycles();
    let list_time = start.elapsed();
    assert!(list_time.as_millis() < 50, "list_cycles took too long");
    assert_eq!(cycle_list.len(), 100); // default gets cleaned up when empty, only created cycles remain

    // Verify cycles are sorted
    for i in 1..cycle_list.len() {
        assert!(cycle_list[i - 1] <= cycle_list[i], "Cycles not sorted");
    }

    // Test position preservation
    cycles.select_cycle("cycle_050".to_string());
    cycles.next(); // move position
    let pos_before = cycles.cycle_data.get("cycle_050").unwrap().position;

    cycles.select_cycle("cycle_025".to_string());
    cycles.select_cycle("cycle_050".to_string());
    let pos_after = cycles.cycle_data.get("cycle_050").unwrap().position;
    assert_eq!(pos_before, pos_after, "Position not preserved");
}

#[test]
fn test_many_cycles_performance_1000() {
    let mut cycles = Cycles::new();

    // Create 1000 cycles with 5 workspaces each
    let start = Instant::now();
    for i in 0..1000 {
        let cycle_name = format!("cycle_{:04}", i);
        cycles.select_cycle(cycle_name.clone());

        for j in 0..5 {
            cycles.toggle(format!("ws_{:04}_{}", i, j));
        }
    }
    let create_time = start.elapsed();
    assert!(
        create_time.as_secs() < 30,
        "Creating 1000 cycles took too long"
    );

    // Test select_cycle performance
    let start = Instant::now();
    cycles.select_cycle("cycle_0000".to_string());
    cycles.select_cycle("cycle_0500".to_string());
    cycles.select_cycle("cycle_0999".to_string());
    let select_time = start.elapsed();
    assert!(
        select_time.as_millis() < 500,
        "select_cycle with 1000 cycles took too long"
    );

    // Test list_cycles
    let start = Instant::now();
    let cycle_list = cycles.list_cycles();
    let list_time = start.elapsed();
    assert!(
        list_time.as_millis() < 500,
        "list_cycles with 1000 cycles took too long"
    );
    assert_eq!(cycle_list.len(), 1000); // default gets cleaned up when empty, only created cycles remain

    // Verify all cycles have independent position tracking
    cycles.select_cycle("cycle_0100".to_string());
    for _ in 0..3 {
        cycles.next();
    }
    let pos_100 = cycles.cycle_data.get("cycle_0100").unwrap().position;
    assert_eq!(pos_100, 3 % 5); // position should be 3 mod 5

    cycles.select_cycle("cycle_0200".to_string());
    for _ in 0..2 {
        cycles.next();
    }
    let pos_200 = cycles.cycle_data.get("cycle_0200").unwrap().position;
    assert_eq!(pos_200, 2 % 5); // position should be 2 mod 5

    // Switch back to verify positions preserved independently
    cycles.select_cycle("cycle_0100".to_string());
    assert_eq!(
        cycles.cycle_data.get("cycle_0100").unwrap().position,
        pos_100
    );
}

// ============================================================================
// EDGE-05: Cycle Name Edge Cases
// ============================================================================

#[test]
fn test_cycle_name_collisions() {
    let mut cycles = Cycles::new();

    // Create a cycle with workspaces
    cycles.select_cycle("work".to_string());
    cycles.toggle("ws1".to_string());
    cycles.toggle("ws2".to_string());

    // Create a different cycle
    cycles.select_cycle("personal".to_string());
    cycles.toggle("home1".to_string());

    // Switch back to work - data should be preserved
    let created = cycles.select_cycle("work".to_string());
    assert!(
        !created,
        "Selecting existing cycle should not create new one"
    );
    assert_eq!(cycles.active_cycle, "work");

    let work_data = cycles.cycle_data.get("work").unwrap();
    assert!(work_data.contains("ws1"));
    assert!(work_data.contains("ws2"));
    assert_eq!(work_data.len(), 2);

    // Verify personal cycle still exists
    assert!(cycles.cycle_data.contains_key("personal"));
}

#[test]
fn test_cycle_name_special_characters() {
    let mut cycles = Cycles::new();

    // Test unicode cycle names
    cycles.select_cycle("🔄".to_string());
    cycles.toggle("ws1".to_string());

    cycles.select_cycle("工作".to_string()); // Chinese for "work"
    cycles.toggle("ws2".to_string());

    cycles.select_cycle("日本語".to_string()); // Japanese
    cycles.toggle("ws3".to_string());

    // Test cycle names with spaces
    cycles.select_cycle("my cycle".to_string());
    cycles.toggle("ws4".to_string());

    // Test cycle names with symbols
    cycles.select_cycle("work@home".to_string());
    cycles.toggle("ws5".to_string());

    cycles.select_cycle("test#1".to_string());
    cycles.toggle("ws6".to_string());

    cycles.select_cycle("a/b/c".to_string());
    cycles.toggle("ws7".to_string());

    // Test very long cycle name
    let long_name = "cycle_".to_string() + &"x".repeat(100);
    cycles.select_cycle(long_name.clone());
    cycles.toggle("ws8".to_string());

    // Verify all cycles exist
    let cycle_list = cycles.list_cycles();
    assert!(cycle_list.contains(&"🔄".to_string()));
    assert!(cycle_list.contains(&"工作".to_string()));
    assert!(cycle_list.contains(&"日本語".to_string()));
    assert!(cycle_list.contains(&"my cycle".to_string()));
    assert!(cycle_list.contains(&"work@home".to_string()));
    assert!(cycle_list.contains(&"test#1".to_string()));
    assert!(cycle_list.contains(&"a/b/c".to_string()));
    assert!(cycle_list.contains(&long_name));

    // Verify sorted order (unicode comparison)
    for i in 1..cycle_list.len() {
        assert!(
            cycle_list[i - 1] <= cycle_list[i],
            "Cycles not sorted correctly"
        );
    }

    // Verify serialization handles special cycle names
    let json = cycles.to_json();
    let deserialized: Cycles = serde_json::from_str(&json).unwrap();
    assert!(deserialized.cycle_data.contains_key("🔄"));
    assert!(deserialized.cycle_data.contains_key("工作"));
}

#[test]
fn test_cycle_name_case_sensitivity() {
    let mut cycles = Cycles::new();

    // Case-sensitive cycle names (BTreeMap behavior)
    cycles.select_cycle("Work".to_string());
    cycles.toggle("ws1".to_string());

    cycles.select_cycle("work".to_string());
    cycles.toggle("ws2".to_string());

    cycles.select_cycle("WORK".to_string());
    cycles.toggle("ws3".to_string());

    // All three should exist as separate cycles
    let cycle_list = cycles.list_cycles();
    assert!(cycle_list.contains(&"Work".to_string()));
    assert!(cycle_list.contains(&"work".to_string()));
    assert!(cycle_list.contains(&"WORK".to_string()));

    // Verify they're truly separate
    assert_eq!(cycles.cycle_data.get("Work").unwrap().len(), 1);
    assert_eq!(cycles.cycle_data.get("work").unwrap().len(), 1);
    assert_eq!(cycles.cycle_data.get("WORK").unwrap().len(), 1);

    // Verify sorted order: "WORK" < "Work" < "work" (ASCII/Unicode order)
    let pos_work = cycle_list.iter().position(|c| c == "work").unwrap();
    let pos_Work = cycle_list.iter().position(|c| c == "Work").unwrap();
    let pos_WORK = cycle_list.iter().position(|c| c == "WORK").unwrap();
    assert!(
        pos_WORK < pos_Work && pos_Work < pos_work,
        "Case-sensitive sort order incorrect"
    );
}

#[test]
fn test_cycle_name_edge_cases() {
    let mut cycles = Cycles::new();

    // Test cycle name that matches a workspace name
    cycles.select_cycle("workspace".to_string());
    cycles.toggle("workspace".to_string()); // same string as cycle name
    assert!(cycles
        .cycle_data
        .get("workspace")
        .unwrap()
        .contains("workspace"));

    // Test cycle name with JSON special characters
    cycles.select_cycle("with\"quotes\"".to_string());
    cycles.toggle("ws1".to_string());

    cycles.select_cycle("with\\backslash".to_string());
    cycles.toggle("ws2".to_string());

    // Verify JSON serialization handles escaping
    let json = cycles.to_json();
    insta::assert_snapshot!("edge_05_cycle_json", json);

    // Verify deserialization
    let deserialized: Cycles = serde_json::from_str(&json).unwrap();
    assert!(deserialized.cycle_data.contains_key("with\"quotes\""));
    assert!(deserialized.cycle_data.contains_key("with\\backslash"));
}

#[test]
fn test_empty_cycle_cleanup() {
    let mut cycles = Cycles::new();

    // Create and leave empty
    cycles.select_cycle("to_be_deleted".to_string());
    // No workspaces added

    // Switch to another cycle
    cycles.select_cycle("keeper".to_string());
    cycles.toggle("ws1".to_string());

    // Empty cycle should be removed
    assert!(!cycles.cycle_data.contains_key("to_be_deleted"));
    assert!(cycles.cycle_data.contains_key("keeper"));

    // Named cycles with workspaces persist
    cycles.select_cycle("work".to_string());
    cycles.toggle("ws2".to_string());
    cycles.select_cycle("personal".to_string());
    // work should still exist
    assert!(cycles.cycle_data.contains_key("work"));
}
