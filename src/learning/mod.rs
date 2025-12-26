// =============================================================================
//  Astra AGI - Astra Learning Crate
//  File: mod.rs
//
//  Description: Adaptive learning algorithms and training infrastructure.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-23
//  Updated:     2025-12-26
//
//  //  This file is dual licensed under the MIT and Apache 2.0 licenses.
//  Please see the root level LICENSE-MIT and LICENSE-APACHE files for details.
// =============================================================================

pub mod autodiff;
pub mod trainer;

pub use autodiff::*;
pub use trainer::*;
