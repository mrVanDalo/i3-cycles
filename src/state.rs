use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, VecDeque};

/// Data stored for each cycle: workspaces as a linked list with position tracking
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CycleData {
    /// Workspaces in this cycle, stored as a linked list (VecDeque)
    pub workspaces: VecDeque<String>,
    /// Current position in the workspace list
    pub position: usize,
    /// Cached count of workspaces (avoids recalculating)
    pub count: usize,
}

impl Default for CycleData {
    fn default() -> Self {
        Self {
            workspaces: VecDeque::new(),
            position: 0,
            count: 0,
        }
    }
}

impl CycleData {
    /// Create a new empty CycleData
    pub fn new() -> Self {
        Self::default()
    }

    /// Return the cached count of workspaces
    pub fn len(&self) -> usize {
        self.count
    }

    /// Check if there are no workspaces in this cycle
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Update the cached count (call after modifying workspaces)
    pub fn update_count(&mut self) {
        self.count = self.workspaces.len();
    }

    /// Add a workspace to the end of the list
    pub fn add_workspace(&mut self, workspace: String) {
        self.workspaces.push_back(workspace);
        self.update_count();
    }

    /// Remove a workspace from anywhere in the list
    pub fn remove_workspace(&mut self, workspace: &str) -> bool {
        let before_len = self.workspaces.len();
        self.workspaces.retain(|w| w != workspace);
        let removed = self.workspaces.len() < before_len;
        if removed {
            self.update_count();
            // Clamp position if it exceeds new length
            if self.position >= self.count && self.count > 0 {
                self.position = self.count - 1;
            } else if self.count == 0 {
                self.position = 0;
            }
        }
        removed
    }

    /// Check if workspace exists in this cycle
    pub fn contains(&self, workspace: &str) -> bool {
        self.workspaces.iter().any(|w| w == workspace)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cycles {
    /// Named cycles, each with its own position tracking
    pub cycle_data: BTreeMap<String, CycleData>,
    /// The currently active cycle name
    pub active_cycle: String,
}

impl Default for Cycles {
    fn default() -> Self {
        Self::new()
    }
}

impl Cycles {
    pub fn new() -> Self {
        let mut cycle_data = BTreeMap::new();
        cycle_data.insert("default".to_string(), CycleData::new());

        Self {
            cycle_data,
            active_cycle: "default".to_string(),
        }
    }

    /// Get mutable reference to active cycle's data
    fn get_active_cycle_data(&mut self) -> &mut CycleData {
        self.cycle_data
            .get_mut(&self.active_cycle)
            .expect("Active cycle must exist")
    }

    /// Get immutable reference to active cycle's data
    fn get_active_cycle_data_readonly(&self) -> &CycleData {
        self.cycle_data
            .get(&self.active_cycle)
            .expect("Active cycle must exist")
    }

    /// Get the next workspace in the active cycle using linked list rotation.
    /// Returns None if cycle has fewer than 2 workspaces.
    /// Rotates: pops from front, pushes to back, returns the element.
    pub fn next(&mut self) -> Option<String> {
        let cycle = self.get_active_cycle_data();

        // Need at least 2 workspaces to cycle
        if cycle.len() < 2 {
            return None;
        }

        // Pop from front, push to back (rotation)
        let workspace = cycle.workspaces.pop_front()?;
        let workspace_clone = workspace.clone();
        cycle.workspaces.push_back(workspace);

        // Update position (wraps around)
        cycle.position = (cycle.position + 1) % cycle.len();

        Some(workspace_clone)
    }

    /// Toggle a workspace in the active cycle
    pub fn toggle(&mut self, workspace: String) -> bool {
        let cycle = self.get_active_cycle_data();

        if cycle.contains(&workspace) {
            cycle.remove_workspace(&workspace);
            false // removed
        } else {
            cycle.add_workspace(workspace);
            true // added
        }
    }

    /// List all cycle names
    pub fn list_cycles(&self) -> Vec<String> {
        let mut names: Vec<_> = self.cycle_data.keys().cloned().collect();
        names.sort();
        names
    }

    /// Select or create a cycle
    /// Returns true if created, false if selected
    pub fn select_cycle(&mut self, name: String) -> bool {
        if self.active_cycle == name {
            return false; // already selected
        }

        let old_cycle_name = self.active_cycle.clone();

        // Delete previous cycle if it's empty
        if let Some(old_data) = self.cycle_data.get(&old_cycle_name) {
            if old_data.is_empty() {
                self.cycle_data.remove(&old_cycle_name);
            }
        }

        // Create cycle if it doesn't exist (starts at position 0)
        let created = !self.cycle_data.contains_key(&name);
        if created {
            self.cycle_data.insert(name.clone(), CycleData::new());
        }

        self.active_cycle = name;
        // Position is automatically restored because it's stored in CycleData
        // New cycles have position = 0 from CycleData::new()

        created
    }

    /// Get current state as JSON for debugging
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_cycles() {
        let cycles = Cycles::new();
        assert_eq!(cycles.active_cycle, "default");
        assert_eq!(cycles.list_cycles(), vec!["default"]);
    }

    #[test]
    fn test_toggle_workspace() {
        let mut cycles = Cycles::new();

        let added = cycles.toggle("1".to_string());
        assert!(added);

        let removed = cycles.toggle("1".to_string());
        assert!(!removed);
    }

    #[test]
    fn test_next_with_insufficient_workspaces() {
        let mut cycles = Cycles::new();
        cycles.toggle("1".to_string());

        assert_eq!(cycles.next(), None);
    }

    #[test]
    fn test_next_with_multiple_workspaces() {
        let mut cycles = Cycles::new();
        cycles.toggle("1".to_string());
        cycles.toggle("2".to_string());
        cycles.toggle("3".to_string());

        let snapshot = insta::Settings::new().bind_to_scope();

        let next1 = cycles.next();
        insta::assert_json_snapshot!("next_first", next1);

        let next2 = cycles.next();
        insta::assert_json_snapshot!("next_second", next2);

        let next3 = cycles.next();
        insta::assert_json_snapshot!("next_third", next3);

        let next4 = cycles.next();
        insta::assert_json_snapshot!("next_wraps", next4);
    }

    #[test]
    fn test_select_cycle() {
        let mut cycles = Cycles::new();

        let created = cycles.select_cycle("work".to_string());
        assert!(created);
        assert_eq!(cycles.active_cycle, "work");

        let selected = cycles.select_cycle("work".to_string());
        assert!(!selected);
    }

    #[test]
    fn test_empty_cycle_deleted_on_switch() {
        let mut cycles = Cycles::new();

        cycles.select_cycle("work".to_string());
        assert!(!cycles.list_cycles().contains(&"default".to_string()));

        cycles.toggle("1".to_string());
        cycles.select_cycle("personal".to_string());
        assert!(cycles.list_cycles().contains(&"work".to_string()));
    }

    #[test]
    fn test_state_serialization() {
        let mut cycles = Cycles::new();
        cycles.toggle("1".to_string());
        cycles.toggle("2".to_string());

        insta::assert_snapshot!("state_json", cycles.to_json());
    }

    #[test]
    fn test_position_preserved_on_cycle_switch() {
        // Fulfill TEST-01: cycle switch preserves position in source cycle
        let mut cycles = Cycles::new();

        // Set up "work" cycle with position at 1
        cycles.select_cycle("work".to_string());
        cycles.toggle("1".to_string());
        cycles.toggle("2".to_string());
        cycles.toggle("3".to_string());
        cycles.next(); // position = 1, returns "1", deque now ["2", "3", "1"]

        // Switch to personal - work position should be preserved
        cycles.select_cycle("personal".to_string());

        // Fulfill TEST-02: cycle switch restores correct position in target cycle
        // Personal is new, so position should be 0
        assert_eq!(cycles.cycle_data.get("personal").unwrap().position, 0);

        // Switch back to work - position should still be 1
        cycles.select_cycle("work".to_string());
        assert_eq!(cycles.cycle_data.get("work").unwrap().position, 1);

        // Next should return "2" (position 1 in ["2", "3", "1"])
        assert_eq!(cycles.next(), Some("2".to_string()));
    }

    #[test]
    fn test_rapid_cycle_switching() {
        // Fulfill TEST-03: rapid cycle switching A→B→A→C→A
        let mut cycles = Cycles::new();

        // Set up A (work) with position at 2
        cycles.select_cycle("work".to_string());
        cycles.toggle("w1".to_string());
        cycles.toggle("w2".to_string());
        cycles.toggle("w3".to_string());
        cycles.next(); // 0 → 1
        cycles.next(); // 1 → 2

        // Set up B (personal) with position at 1
        cycles.select_cycle("personal".to_string());
        cycles.toggle("p1".to_string());
        cycles.toggle("p2".to_string());
        cycles.next(); // 0 → 1

        // Set up C (misc) with position at 0
        cycles.select_cycle("misc".to_string());
        cycles.toggle("m1".to_string());
        // position stays 0 (need 2 workspaces for next())

        // Rapid switching: A → B → A → C → A
        cycles.select_cycle("work".to_string());
        assert_eq!(cycles.cycle_data.get("work").unwrap().position, 2);

        cycles.select_cycle("personal".to_string());
        assert_eq!(cycles.cycle_data.get("personal").unwrap().position, 1);

        cycles.select_cycle("work".to_string());
        assert_eq!(cycles.cycle_data.get("work").unwrap().position, 2);

        cycles.select_cycle("misc".to_string());
        assert_eq!(cycles.cycle_data.get("misc").unwrap().position, 0);

        cycles.select_cycle("work".to_string());
        assert_eq!(cycles.cycle_data.get("work").unwrap().position, 2);
    }

    #[test]
    fn test_position_with_empty_cycles() {
        // Fulfill TEST-04: position with empty cycle switches
        let mut cycles = Cycles::new();

        // Create and set up "work" with position
        cycles.select_cycle("work".to_string());
        cycles.toggle("1".to_string());
        cycles.toggle("2".to_string());
        cycles.next(); // position = 1

        // Create empty "empty_cycle"
        cycles.select_cycle("empty_cycle".to_string());
        // Position should be 0 for new cycle
        assert_eq!(cycles.cycle_data.get("empty_cycle").unwrap().position, 0);

        // Switch to work
        cycles.select_cycle("work".to_string());
        assert_eq!(cycles.cycle_data.get("work").unwrap().position, 1);

        // Now switch back to empty_cycle
        // empty_cycle should still be empty with position 0
        cycles.select_cycle("empty_cycle".to_string());
        assert_eq!(cycles.cycle_data.get("empty_cycle").unwrap().position, 0);
        assert!(cycles.cycle_data.get("empty_cycle").unwrap().is_empty());
    }

    #[test]
    fn test_position_after_workspace_toggle() {
        // Fulfill TEST-05: position after workspace toggle in each cycle
        let mut cycles = Cycles::new();

        // Set up work cycle
        cycles.select_cycle("work".to_string());
        cycles.toggle("1".to_string());
        cycles.toggle("2".to_string());
        cycles.toggle("3".to_string());
        cycles.next(); // position = 1, returns "1"
        cycles.next(); // position = 2, returns "2"
                       // Deque is now ["3", "1", "2"], position = 2

        // Remove "3" (which is at the front)
        // After removal, position should clamp
        cycles.toggle("3".to_string()); // removes "3"
                                        // Now only ["1", "2"] remain, position was 2 but should clamp to 1
        let work_data = cycles.cycle_data.get("work").unwrap();
        assert_eq!(work_data.position, 1); // clamped from 2 to 1

        // Add a new workspace "4"
        cycles.toggle("4".to_string()); // adds to back
                                        // Deque: ["1", "2", "4"], position stays 1
        let work_data = cycles.cycle_data.get("work").unwrap();
        assert_eq!(work_data.position, 1);

        // Switch to another cycle and back
        cycles.select_cycle("personal".to_string());
        cycles.select_cycle("work".to_string());

        // Position should be preserved at 1
        let work_data = cycles.cycle_data.get("work").unwrap();
        assert_eq!(work_data.position, 1);
    }

    #[test]
    fn test_position_wraparound_independent() {
        // Fulfill TEST-06: position wraparound within each cycle independently
        let mut cycles = Cycles::new();

        // Set up work with 3 workspaces
        cycles.select_cycle("work".to_string());
        cycles.toggle("w1".to_string());
        cycles.toggle("w2".to_string());
        cycles.toggle("w3".to_string());

        // Set up personal with 2 workspaces
        cycles.select_cycle("personal".to_string());
        cycles.toggle("p1".to_string());
        cycles.toggle("p2".to_string());

        // Advance personal through full cycle (wraps at 2)
        cycles.next(); // position: 0→1, returns "p1"
        cycles.next(); // position: 1→0, returns "p2" (wrapped)
        assert_eq!(cycles.cycle_data.get("personal").unwrap().position, 0);

        // Switch to work, advance through full cycle (wraps at 3)
        cycles.select_cycle("work".to_string());
        cycles.next(); // position: 0→1
        cycles.next(); // position: 1→2
        cycles.next(); // position: 2→0 (wrapped)
        assert_eq!(cycles.cycle_data.get("work").unwrap().position, 0);

        // Switch back to personal, position should still be 0
        cycles.select_cycle("personal".to_string());
        assert_eq!(cycles.cycle_data.get("personal").unwrap().position, 0);
    }

    #[test]
    fn test_position_new_cycle_starts_at_zero() {
        // Verify POS-03: New cycles start at position 0
        let mut cycles = Cycles::new();

        // Create a new cycle
        cycles.select_cycle("new_cycle".to_string());

        // Verify position starts at 0
        assert_eq!(cycles.cycle_data.get("new_cycle").unwrap().position, 0);

        // Add some workspaces, position should still be 0
        cycles.toggle("a".to_string());
        cycles.toggle("b".to_string());
        assert_eq!(cycles.cycle_data.get("new_cycle").unwrap().position, 0);
    }

    // === Workspace Ordering Tests (ORD-01 to ORD-05) ===

    #[test]
    fn test_numeric_workspace_order() {
        // ORD-01: Workspaces added as "1", "2", "10" cycle in that exact order
        // (not sorted as 1, 10, 2)
        let mut cycles = Cycles::new();

        // Add workspaces in order: "1", "2", "10"
        cycles.toggle("1".to_string());
        cycles.toggle("2".to_string());
        cycles.toggle("10".to_string());

        let _snapshot = insta::Settings::new().bind_to_scope();

        // Call next() 4 times (including wrap)
        let next1 = cycles.next();
        insta::assert_json_snapshot!("ord_01_first", next1);

        let next2 = cycles.next();
        insta::assert_json_snapshot!("ord_01_second", next2);

        let next3 = cycles.next();
        insta::assert_json_snapshot!("ord_01_third", next3);

        let next4 = cycles.next();
        insta::assert_json_snapshot!("ord_01_wraps", next4);
    }

    #[test]
    fn test_named_workspace_order() {
        // ORD-02: Named workspaces "alpha", "beta", "gamma" cycle in insertion order
        let mut cycles = Cycles::new();

        // Add workspaces in order: "alpha", "beta", "gamma"
        cycles.toggle("alpha".to_string());
        cycles.toggle("beta".to_string());
        cycles.toggle("gamma".to_string());

        let _snapshot = insta::Settings::new().bind_to_scope();

        // Call next() 4 times (including wrap)
        let next1 = cycles.next();
        insta::assert_json_snapshot!("ord_02_first", next1);

        let next2 = cycles.next();
        insta::assert_json_snapshot!("ord_02_second", next2);

        let next3 = cycles.next();
        insta::assert_json_snapshot!("ord_02_third", next3);

        let next4 = cycles.next();
        insta::assert_json_snapshot!("ord_02_wraps", next4);
    }

    #[test]
    fn test_mixed_workspace_order() {
        // ORD-03: Mixed workspaces "1", "alpha", "2", "beta" interleave as added
        let mut cycles = Cycles::new();

        // Add workspaces in order: "1", "alpha", "2", "beta"
        cycles.toggle("1".to_string());
        cycles.toggle("alpha".to_string());
        cycles.toggle("2".to_string());
        cycles.toggle("beta".to_string());

        let _snapshot = insta::Settings::new().bind_to_scope();

        // Call next() 5 times (including wrap)
        let next1 = cycles.next();
        insta::assert_json_snapshot!("ord_03_first", next1);

        let next2 = cycles.next();
        insta::assert_json_snapshot!("ord_03_second", next2);

        let next3 = cycles.next();
        insta::assert_json_snapshot!("ord_03_third", next3);

        let next4 = cycles.next();
        insta::assert_json_snapshot!("ord_03_fourth", next4);

        let next5 = cycles.next();
        insta::assert_json_snapshot!("ord_03_wraps", next5);
    }

    #[test]
    fn test_single_workspace_no_cycle() {
        // ORD-04: Single workspace "solo" returns None when next() called
        let mut cycles = Cycles::new();

        // Add only one workspace
        cycles.toggle("solo".to_string());

        // Single workspace should return None (can't cycle)
        assert_eq!(cycles.next(), None);
    }

    #[test]
    fn test_empty_cycle_no_next() {
        // ORD-05: Empty cycle returns None immediately on next()
        let mut cycles = Cycles::new();

        // Empty cycle should return None
        assert_eq!(cycles.next(), None);
    }
}
