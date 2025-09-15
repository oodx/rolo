//! Feature tests for rolo - RSB MODULE_SPEC compliant
//!
//! Tests feature-specific functionality with feature gates

#[path = "features/width_features.rs"]
mod width_features;

#[path = "features/cli_features.rs"]
mod cli_features;

// Re-export feature tests
pub use width_features::*;
pub use cli_features::*;