//! Feature tests for rolo - RSB MODULE_SPEC compliant
//!
//! Tests feature-specific functionality with feature gates

#[path = "features/width_features.rs"]
mod width_features;

// Re-export feature tests
pub use width_features::*;