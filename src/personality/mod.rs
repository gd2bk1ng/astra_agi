// =============================================================================
//  Astra AGI - Personality Module Root
//  File: mod.rs
//
//  Description:
//  Root module for personality and affective systems.
//  Exposes personality traits and interactive response capabilities.
//
//  Author:      Alex Roussinov
//  Created:     2025-12-25
//  Updated:     2025-12-26
//
//  This file is dual licensed under the MIT and Apache 2.0 licenses.
// =============================================================================

pub mod personality;
pub mod humor;

pub use humor::Humor;
use crate::personality::humor::Humor;

let humor = Humor::new();
let joke = humor.tell_joke();

pub mod humor;

