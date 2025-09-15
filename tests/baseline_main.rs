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

#[path = "baseline/column_mode.rs"]
mod column_mode;

#[path = "baseline/table_mode.rs"]
mod table_mode;

#[path = "baseline/integration_with_test_data.rs"]
mod integration_with_test_data;

#[path = "baseline/terminal_width_detection.rs"]
mod terminal_width_detection;

// Re-export baseline tests
pub use core_functionality::*;
pub use default_behavior::*;
pub use error_handling::*;
pub use column_mode::*;
pub use table_mode::*;
pub use integration_with_test_data::*;
pub use terminal_width_detection::*;