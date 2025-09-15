//! Baseline functionality tests - no features required
//!
//! These tests validate core rolo functionality that works without
//! any optional features enabled. This ensures the default build works.

#[path = "baseline/core_functionality.rs"]
mod core_functionality;

#[path = "baseline/default_behavior.rs"]
mod default_behavior;

#[path = "baseline/error_handling.rs"]
mod error_handling;

// Re-export baseline tests
pub use core_functionality::*;
pub use default_behavior::*;
pub use error_handling::*;