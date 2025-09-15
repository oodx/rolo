//! CLI module orchestrator - Command line interface with RSB patterns
//! Follows RSB MODULE_SPEC patterns

mod helpers; // Internal implementation details per MODULE_SPEC

pub mod error;
pub mod utils;
pub mod dispatch;

// Re-export public APIs
pub use utils::*;
pub use dispatch::*;