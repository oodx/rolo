//! UAT (User Acceptance Testing) for rolo - RSB HOWTO_TEST compliant
//!
//! Executive-level validation of user-facing functionality

#[path = "uat/acceptance_baseline.rs"]
mod acceptance_baseline;

#[path = "uat/width_acceptance.rs"]
mod width_acceptance;

#[path = "uat/prelude_acceptance.rs"]
mod prelude_acceptance;

#[path = "uat/cli_acceptance.rs"]
mod cli_acceptance;

// Re-export UAT tests
pub use acceptance_baseline::*;
pub use width_acceptance::*;
pub use prelude_acceptance::*;
pub use cli_acceptance::*;