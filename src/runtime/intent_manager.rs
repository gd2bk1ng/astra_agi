// =============================================================================
//  Astra AGI - Intent Management System (IMS)
//  File: intent_manager.rs
//
//  Description:
//  Manages the lifecycle of intents—Astra's goals and tasks.
//  Extended to accept and store rich task metadata such as ethical importance,
//  enabling affective and value-based prioritization.
//
//  This enhancement allows Astra to reason about tasks with nuanced context,
//  aligning behavior with human values and ethical considerations.
//
//  Author:      Alex Roussinov & Ecosia AI
//  Created:     2025-12-25
//  Updated:     2026-01-02
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::time::{Duration, Instant};

/// Unique identifier for an Intent.
pub type IntentId = u64;

/// Represents the current state of an intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentState {
    Pending,
    Active,
    Completed,
    Cancelled,
}

/// Core data structure representing an Intent.
#[derive(Debug, Clone)]
pub struct Intent {
    pub id: IntentId,
    pub description: String,
    pub priority: u32,               // Higher = more urgent
    pub created_at: Instant,
    pub deadline: Option<Instant>,  // Optional deadline for completion
    pub duration: Option<Duration>, // Estimated time to complete
    pub state: IntentState,
    pub metadata: HashMap<String, String>, // Flexible key-value for extensibility
}

impl Intent {
    /// Creates a new Intent with mandatory fields.
    pub fn new(id: IntentId, description: impl Into<String>, priority: u32) -> Self {
        Intent {
            id,
            description: description.into(),
            priority,
            created_at: Instant::now(),
            deadline: None,
            duration: None,
            state: IntentState::Pending,
            metadata: HashMap::new(),
        }
    }

    /// Checks if the intent is overdue based on current time.
    pub fn is_overdue(&self) -> bool {
        if let Some(deadline) = self.deadline {
            Instant::now() > deadline && self.state != IntentState::Completed
        } else {
            false
        }
    }
}

/// Wrapper to allow priority queue ordering by Intent priority and deadline.
#[derive(Debug, Clone)]
struct IntentWrapper(Intent);

impl PartialEq for IntentWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

impl Eq for IntentWrapper {}

impl PartialOrd for IntentWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for IntentWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.priority.cmp(&self.0.priority)
            .then_with(|| {
                match (self.0.deadline, other.0.deadline) {
                    (Some(a), Some(b)) => a.cmp(&b),
                    (Some(_), None) => Ordering::Less,
                    (None, Some(_)) => Ordering::Greater,
                    (None, None) => Ordering::Equal,
                }
            })
            .then_with(|| self.0.created_at.cmp(&other.0.created_at))
    }
}

/// Manages all intents, providing APIs for creation, update, scheduling, and querying.
pub struct IntentManager {
    intents: HashMap<IntentId, Intent>,
    priority_queue: BinaryHeap<IntentWrapper>,
    next_id: IntentId,
}

impl IntentManager {
    /// Creates a new IntentManager instance.
    pub fn new() -> Self {
        IntentManager {
            intents: HashMap::new(),
            priority_queue: BinaryHeap::new(),
            next_id: 1,
        }
    }

    /// Creates and adds a new intent with optional metadata, returning its unique ID.
    pub fn create_intent_with_metadata(&mut self, description: impl Into<String>, priority: u32, metadata: Option<HashMap<String, String>>) -> IntentId {
        let id = self.next_id;
        self.next_id += 1;

        let mut intent = Intent::new(id, description, priority);
        if let Some(meta) = metadata {
            intent.metadata = meta;
        }

        self.priority_queue.push(IntentWrapper(intent.clone()));
        self.intents.insert(id, intent);
        id
    }

    /// Updates an existing intent's priority, deadline, or state.
    pub fn update_intent(&mut self, id: IntentId, priority: Option<u32>, deadline: Option<Option<Instant>>, state: Option<IntentState>) -> Result<(), String> {
        if let Some(intent) = self.intents.get_mut(&id) {
            if let Some(p) = priority {
                intent.priority = p;
            }
            if let Some(d) = deadline {
                intent.deadline = d;
            }
            if let Some(s) = state {
                intent.state = s;
            }
            self.rebuild_priority_queue();
            Ok(())
        } else {
            Err(format!("Intent ID {} not found", id))
        }
    }

    /// Marks an intent as completed.
    pub fn complete_intent(&mut self, id: IntentId) -> Result<(), String> {
        self.update_intent(id, None, None, Some(IntentState::Completed))
    }

    /// Cancels an intent.
    pub fn cancel_intent(&mut self, id: IntentId) -> Result<(), String> {
        self.update_intent(id, None, None, Some(IntentState::Cancelled))
    }

    /// Returns the next highest priority pending or active intent, if any.
    pub fn next_intent(&mut self) -> Option<Intent> {
        while let Some(IntentWrapper(intent)) = self.priority_queue.pop() {
            if intent.state == IntentState::Pending || intent.state == IntentState::Active {
                return Some(intent);
            }
        }
        None
    }

    /// Returns a reference to an intent by ID.
    pub fn get_intent(&self, id: IntentId) -> Option<&Intent> {
        self.intents.get(&id)
    }

    /// Returns all intents in the system.
    pub fn all_intents(&self) -> Vec<&Intent> {
        self.intents.values().collect()
    }

    /// Rebuilds the priority queue from the intents map.
    fn rebuild_priority_queue(&mut self) {
        self.priority_queue.clear();
        for intent in self.intents.values() {
            self.priority_queue.push(IntentWrapper(intent.clone()));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intent_creation_and_metadata() {
        let mut im = IntentManager::new();
        let mut metadata = HashMap::new();
        metadata.insert("ethical_importance".to_string(), "high".to_string());
        let id = im.create_intent_with_metadata("Complete report", 10, Some(metadata));
        let intent = im.get_intent(id).expect("Intent should exist");
        assert_eq!(intent.description, "Complete report");
        assert_eq!(intent.priority, 10);
        assert_eq!(intent.state, IntentState::Pending);
        assert_eq!(intent.metadata.get("ethical_importance").unwrap(), "high");
    }
}
