//! Width calculation module orchestrator - ANSI/Unicode width handling
//! Follows RSB MODULE_SPEC patterns

mod helpers;

pub mod error;
pub mod utils;

// Boxy adapter (feature-gated)
#[cfg(feature = "width-boxy")]
pub mod width_boxy_adapter;

// Re-export public APIs
pub use utils::*;