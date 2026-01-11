// ============================================================================
//                        ASTRA AGI • STORAGE ABSTRACTION LAYER
//        Persistent Key‑Value Backend for Ontology & Knowledge Structures
// ---------------------------------------------------------------------------
//   Architectural Role:
//       Component of Astra’s Knowledge Layer, providing a clean abstraction
//       over persistent storage for ontology state, indexes, and metadata.
//       This module decouples the ontology logic from the underlying database
//       engine, enabling flexible backends while offering a robust default
//       implementation using the sled embedded key‑value store.
//
//   Core Functions:
//       • Define a generic Storage trait for save/load operations
//       • Provide a sled‑based implementation with automatic persistence
//       • Support binary serialization of ontology state for durability
//       • Serve as the persistence backbone for the Knowledge subsystem
//
//   File:        /src/knowledge/storage.rs
//   Author:      Alex Roussinov
//   Created:     2025-12-26
//   Updated:     2026-01-11
//
//   License:
//       Dual-licensed under the MIT and Apache 2.0 licenses.
//       See LICENSE-MIT and LICENSE-APACHE in the repository root for details.
// ============================================================================

use sled::{Db, IVec};
use std::path::Path;
use anyhow::{Result, Context};

/// Trait defining storage interface
pub trait Storage {
    fn save(&self, key: &str, value: &[u8]) -> Result<()>;
    fn load(&self, key: &str) -> Result<Option<Vec<u8>>>;
}

/// Sled-based storage implementation
pub struct SledStorage {
    db: Db,
}

impl SledStorage {
    /// Opens or creates a sled database at the specified path
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let db = sled::open(path).context("Failed to open sled database")?;
        Ok(SledStorage { db })
    }
}

impl Storage for SledStorage {
    fn save(&self, key: &str, value: &[u8]) -> Result<()> {
        self.db.insert(key, value)?;
        self.db.flush()?;
        Ok(())
    }

    fn load(&self, key: &str) -> Result<Option<Vec<u8>>> {
        match self.db.get(key)? {
            Some(ivec) => Ok(Some(ivec.to_vec())),
            None => Ok(None),
        }
    }
}
