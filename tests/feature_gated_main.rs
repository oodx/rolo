//! Feature-gated tests - requires specific features to be enabled
//!
//! These tests validate functionality that only works when specific
//! optional features are enabled.

#[cfg(feature = "width-boxy")]
#[path = "feature_gated/width_boxy.rs"]
mod width_boxy;

#[cfg(feature = "visual")]
#[path = "feature_gated/visual.rs"]
mod visual;

#[cfg(feature = "csv-support")]
#[path = "feature_gated/csv_support.rs"]
mod csv_support;

#[cfg(feature = "json-support")]
#[path = "feature_gated/json_support.rs"]
mod json_support;

// Re-export feature-gated tests
#[cfg(feature = "width-boxy")]
pub use width_boxy::*;

#[cfg(feature = "visual")]
pub use visual::*;

#[cfg(feature = "csv-support")]
pub use csv_support::*;

#[cfg(feature = "json-support")]
pub use json_support::*;