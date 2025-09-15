//! Sanity tests for rolo - RSB MODULE_SPEC compliant
//!
//! Tests basic functionality without features, visible output for debugging

#[path = "sanity/baseline.rs"]
mod sanity_baseline;

#[path = "sanity/column_mode.rs"]
mod column_mode;

// Re-export sanity tests
pub use sanity_baseline::*;
pub use column_mode::*;