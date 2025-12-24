// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\versioning.rs
//
//  Description: Tracks versions and changes to Ontology data.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-26
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

use std::collections::HashMap;
use chrono::{DateTime, Utc};

/// Represents a versioned snapshot of ontology data
#[derive(Debug, Clone)]
pub struct Version {
    pub id: usize,
    pub timestamp: DateTime<Utc>,
    pub description: String,
    pub changes: HashMap<String, String>, // e.g., serialized diffs or summaries
}

pub struct VersionManager {
    pub versions: Vec<Version>,
}

impl VersionManager {
    pub fn new() -> Self {
        VersionManager { versions: Vec::new() }
    }

    pub fn add_version(&mut self, description: &str, changes: HashMap<String, String>) {
        let id = self.versions.len() + 1;
        let version = Version {
            id,
            timestamp: Utc::now(),
            description: description.to_string(),
            changes,
        };
        self.versions.push(version);
    }

    pub fn get_latest(&self) -> Option<&Version> {
        self.versions.last()
    }

    pub fn get_version(&self, id: usize) -> Option<&Version> {
        self.versions.iter().find(|v| v.id == id)
    }
}
