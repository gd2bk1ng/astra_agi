//! Astra Learning Crate Root
//!
//! Public API re-exporting learning modules.

pub mod autodiff;
pub mod trainer;

pub use autodiff::*;
pub use trainer::*;
