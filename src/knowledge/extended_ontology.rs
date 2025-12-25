// =============================================================================
//  Astra AGI - Extended Ontology Module (EOM)
//  File: extended_ontology.rs
//
//  Description:
//  Provides a dynamic and versioned ontology core with contextual and epistemic metadata.
//  Enables Astra to represent evolving knowledge, handle multiple perspectives,
//  and reason about the certainty and provenance of facts.
//
//  This module supports Astra's living knowledge base, essential for adaptive reasoning.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-26
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

use std::collections::{HashMap, HashSet};
use std::time::{SystemTime, UNIX_EPOCH};

/// Unique identifier for ontology entities and concepts.
pub type EntityId = u64;

/// Represents the confidence level of a statement or fact.
/// Range: 0.0 (no confidence) to 1.0 (absolute certainty).
pub type Confidence = f32;

/// Represents the source or provenance of a piece of knowledge.
#[derive(Debug, Clone)]
pub struct Provenance {
    pub source_name: String,
    pub timestamp: u64, // Unix timestamp
    pub notes: Option<String>,
}

impl Provenance {
    pub fn new(source_name: impl Into<String>, notes: Option<String>) -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Self {
            source_name: source_name.into(),
            timestamp: now,
            notes,
        }
    }
}

/// Represents a single fact or statement in the ontology.
#[derive(Debug, Clone)]
pub struct Fact {
    pub subject: EntityId,
    pub predicate: String,
    pub object: String,
    pub confidence: Confidence,
    pub provenance: Provenance,
}

/// Represents a version of the ontology.
/// Supports immutable snapshots for rollback and branching.
#[derive(Debug, Clone)]
pub struct OntologyVersion {
    pub version_id: u64,
    pub timestamp: u64,
    pub facts: Vec<Fact>,
    pub parent_version: Option<u64>, // For version lineage
}

/// Contextual view of ontology facts.
/// Allows filtering or overriding facts based on context (e.g., user, environment).
#[derive(Debug, Clone)]
pub struct OntologyContext {
    pub context_id: u64,
    pub name: String,
    pub active_facts: HashSet<usize>, // Indexes into OntologyVersion.facts for active facts
    pub metadata: HashMap<String, String>, // Context-specific metadata
}

/// The main ontology manager that holds versions, contexts, and provides APIs for querying.
pub struct OntologyManager {
    versions: HashMap<u64, OntologyVersion>,
    contexts: HashMap<u64, OntologyContext>,
    current_version: u64,
    next_version_id: u64,
    next_context_id: u64,
}

impl OntologyManager {
    /// Creates a new OntologyManager with an empty initial version.
    pub fn new() -> Self {
        let initial_version = OntologyVersion {
            version_id: 0,
            timestamp: current_unix_timestamp(),
            facts: Vec::new(),
            parent_version: None,
        };

        let mut versions = HashMap::new();
        versions.insert(0, initial_version);

        OntologyManager {
            versions,
            contexts: HashMap::new(),
            current_version: 0,
            next_version_id: 1,
            next_context_id: 1,
        }
    }

    /// Adds a new fact to the current ontology version.
    /// Returns the index of the fact within the version.
    pub fn add_fact(&mut self, fact: Fact) -> usize {
        let current_version = self.versions.get_mut(&self.current_version).unwrap();
        current_version.facts.push(fact);
        current_version.facts.len() - 1
    }

    /// Creates a new version based on the current one (snapshot).
    /// Returns the new version ID.
    pub fn create_version(&mut self) -> u64 {
        let parent_version = self.current_version;
        let parent = self.versions.get(&parent_version).unwrap();

        let new_version = OntologyVersion {
            version_id: self.next_version_id,
            timestamp: current_unix_timestamp(),
            facts: parent.facts.clone(),
            parent_version: Some(parent_version),
        };

        self.versions.insert(self.next_version_id, new_version);
        self.current_version = self.next_version_id;
        self.next_version_id += 1;

        self.current_version
    }

    /// Switches the active version to the specified version ID.
    /// Returns error if the version does not exist.
    pub fn switch_version(&mut self, version_id: u64) -> Result<(), String> {
        if self.versions.contains_key(&version_id) {
            self.current_version = version_id;
            Ok(())
        } else {
            Err(format!("Version {} does not exist", version_id))
        }
    }

    /// Creates a new context with a name and optional metadata.
    pub fn create_context(&mut self, name: impl Into<String>, metadata: Option<HashMap<String, String>>) -> u64 {
        let id = self.next_context_id;
        self.next_context_id += 1;

        let context = OntologyContext {
            context_id: id,
            name: name.into(),
            active_facts: HashSet::new(),
            metadata: metadata.unwrap_or_default(),
        };

        self.contexts.insert(id, context);
        id
    }

    /// Adds a fact index to a context's active facts.
    pub fn add_fact_to_context(&mut self, context_id: u64, fact_index: usize) -> Result<(), String> {
        if let Some(context) = self.contexts.get_mut(&context_id) {
            context.active_facts.insert(fact_index);
            Ok(())
        } else {
            Err(format!("Context {} not found", context_id))
        }
    }

    /// Queries facts in the current version filtered by context if provided.
    pub fn query_facts(&self, context_id: Option<u64>) -> Vec<&Fact> {
        let version = self.versions.get(&self.current_version).unwrap();

        match context_id {
            Some(cid) => {
                if let Some(context) = self.contexts.get(&cid) {
                    context.active_facts.iter()
                        .filter_map(|&idx| version.facts.get(idx))
                        .collect()
                } else {
                    Vec::new()
                }
            }
            None => version.facts.iter().collect(),
        }
    }

    /// Gets the current ontology version ID.
    pub fn current_version(&self) -> u64 {
        self.current_version
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
    fn test_versioning_and_context() {
        let mut manager = OntologyManager::new();

        let fact = Fact {
            subject: 1,
            predicate: "is_a".to_string(),
            object: "Human".to_string(),
            confidence: 0.95,
            provenance: Provenance::new("InitialData", None),
        };

        let idx = manager.add_fact(fact);
        let context_id = manager.create_context("Default", None);
        manager.add_fact_to_context(context_id, idx).unwrap();

        let facts = manager.query_facts(Some(context_id));
        assert_eq!(facts.len(), 1);

        let new_version_id = manager.create_version();
        assert_eq!(manager.current_version(), new_version_id);

        // Switch back to initial version
        manager.switch_version(0).unwrap();
        assert_eq!(manager.current_version(), 0);
    }
}
