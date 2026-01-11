// ============================================================================
//                 ASTRA AGI • NARRATIVE MEMORY SYSTEM (NMS)
//        Persistent Identity, Experiential Recall & Self‑Reflective Context
// ----------------------------------------------------------------------------
//   Architectural Role:
//       Implements Astra’s long‑term narrative memory, enabling the system to
//       maintain a persistent sense of identity, recall past experiences, and
//       construct coherent self‑stories. This module supports introspection,
//       trust‑building, contextual continuity, and learning from prior actions.
//       It forms the backbone of Astra’s autobiographical and experiential memory.
//
//   Core Functions:
//       • Store chronologically ordered narrative events
//       • Maintain persistent identity through experiential continuity
//       • Support retrieval of recent or context‑relevant memories
//       • Enable reflective reasoning and self‑storytelling
//       • Provide structured metadata for advanced cognitive processing
//
//   File:        /src/memory/narrative_memory.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-25
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================


use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a single narrative event or memory.
#[derive(Debug, Clone)]
pub struct NarrativeEvent {
    pub timestamp: u64,       // Unix timestamp
    pub event_type: String,   // E.g., "task_started", "belief_updated"
    pub description: String,  // Human-readable description
    pub metadata: Option<String>, // Optional JSON or structured data
}

/// Narrative memory storing a chronological sequence of events.
pub struct NarrativeMemory {
    pub events: VecDeque<NarrativeEvent>,
    pub max_capacity: usize, // Limits memory size to avoid unbounded growth
}

impl NarrativeMemory {
    /// Creates a new NarrativeMemory with specified capacity.
    pub fn new(max_capacity: usize) -> Self {
        NarrativeMemory {
            events: VecDeque::with_capacity(max_capacity),
            max_capacity,
        }
    }

    /// Adds a new event to the narrative memory.
    pub fn add_event(&mut self, event_type: impl Into<String>, description: impl Into<String>, metadata: Option<String>) {
        let now = current_unix_timestamp();
        let event = NarrativeEvent {
            timestamp: now,
            event_type: event_type.into(),
            description: description.into(),
            metadata,
        };

        if self.events.len() == self.max_capacity {
            self.events.pop_front(); // Remove oldest event
        }
        self.events.push_back(event);
    }

    /// Retrieves the most recent N events.
    pub fn recent_events(&self, count: usize) -> Vec<&NarrativeEvent> {
        self.events.iter().rev().take(count).collect()
    }
}

/// Helper function to get current unix timestamp in seconds.
fn current_unix_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_retrieve_events() {
        let mut memory = NarrativeMemory::new(5);
        memory.add_event("task_started", "Started processing task A", None);
        memory.add_event("belief_updated", "Updated confidence in fact X", Some("{\"confidence\":0.9}".to_string()));

        let recent = memory.recent_events(2);
        assert_eq!(recent.len(), 2);
        assert_eq!(recent[0].event_type, "belief_updated");
        assert_eq!(recent[1].event_type, "task_started");
    }

    #[test]
    fn test_capacity_limit() {
        let mut memory = NarrativeMemory::new(3);
        for i in 0..5 {
            memory.add_event("event", format!("Event {}", i), None);
        }
        assert_eq!(memory.events.len(), 3);
        assert_eq!(memory.events.front().unwrap().description, "Event 2");
    }
}
