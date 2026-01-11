// ============================================================================
//                     ASTRA AGI • ONTOLOGY VERSIONING MODULE
//        Snapshot Tracking, Change History & Knowledge Evolution Metadata
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, responsible for maintaining a
//       chronological record of ontology evolution. This module captures
//       versioned snapshots, descriptive metadata, and structured change logs,
//       enabling rollback, auditing, temporal reasoning, and historical
//       introspection across Astra’s dynamic knowledge base.
//
//   Core Functions:
//       • Create immutable version snapshots with timestamps and descriptions
//       • Record structured change metadata for each version
//       • Retrieve specific versions or the latest snapshot
//       • Support higher‑level reasoning about ontology evolution over time
//
//   File:        /src/knowledge/versioning.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-26
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

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
