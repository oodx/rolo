//! Layout module orchestrator - Column, Table, and List formatting
//! Follows RSB MODULE_SPEC patterns

mod helpers; // Internal implementation details per MODULE_SPEC

pub mod error;
pub mod utils;

// Module-owned macros
pub mod macros;

// Layout mode implementations
mod column;
mod table;
mod list;

// Re-export public APIs
pub use column::*;
pub use table::*;
pub use list::*;
pub use utils::*;