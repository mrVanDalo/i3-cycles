use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Cycles {
    /// Named cycles, each containing a set of workspace names
    cycles: BTreeMap<String, BTreeSet<String>>,
    /// The currently active cycle name
    active_cycle: String,
    /// Current position in the active cycle (workspace index)
    current_position: usize,
}

impl Default for Cycles {
    fn default() -> Self {
        Self::new()
    }
}

impl Cycles {
    pub fn new() -> Self {
        let mut cycles = BTreeMap::new();
        cycles.insert("default".to_string(), BTreeSet::new());

        Self {
            cycles,
            active_cycle: "default".to_string(),
            current_position: 0,
        }
    }

    /// Get the next workspace in the active cycle, or None if < 2 workspaces
    pub fn next(&mut self) -> Option<String> {
        let workspaces = self.cycles.get(&self.active_cycle)?;

        if workspaces.len() < 2 {
            return None;
        }

        let mut sorted: Vec<_> = workspaces.iter().cloned().collect();
        sorted.sort();

        self.current_position = (self.current_position + 1) % sorted.len();
        Some(sorted[self.current_position].clone())
    }

    /// Toggle a workspace in the active cycle
    pub fn toggle(&mut self, workspace: String) -> bool {
        let workspaces = self.cycles.get_mut(&self.active_cycle)
            .expect("Active cycle must exist");

        if workspaces.contains(&workspace) {
            workspaces.remove(&workspace);
            false // removed
        } else {
            workspaces.insert(workspace);
            true // added
        }
    }

    /// List all cycle names
    pub fn list_cycles(&self) -> Vec<String> {
        let mut names: Vec<_> = self.cycles.keys().cloned().collect();
        names.sort();
        names
    }

    /// Select or create a cycle
    /// Returns true if created, false if selected
    pub fn select_cycle(&mut self, name: String) -> bool {
        let created = !self.cycles.contains_key(&name);

        // Delete previous cycle if it's empty
        if self.active_cycle != name {
            if let Some(workspaces) = self.cycles.get(&self.active_cycle) {
                if workspaces.is_empty() {
                    self.cycles.remove(&self.active_cycle);
                }
            }
        }

        // Create cycle if it doesn't exist
        if created {
            self.cycles.insert(name.clone(), BTreeSet::new());
        }

        self.active_cycle = name;
        self.current_position = 0;

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
}
