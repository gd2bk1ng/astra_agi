// =============================================================================
//  Astra AGI
//  File: astra_agi\src\knowledge\storage.rs
//
//  Description: Abstracts persistent storage for Ontology using sled key-value store.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-26
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

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
