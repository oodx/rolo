//! Visual UAT (User Acceptance Testing) for rolo - Executive visual validation
//!
//! These tests demonstrate actual text formatting output for executive review

#[path = "visual_uat/column_formatting.rs"]
mod column_formatting;

#[path = "visual_uat/table_formatting.rs"]
mod table_formatting;

#[path = "visual_uat/list_formatting.rs"]
mod list_formatting;

#[path = "visual_uat/width_integration.rs"]
mod width_integration;

#[path = "visual_uat/column_mode_uat.rs"]
mod column_mode_uat;

// Re-export visual UAT tests
pub use column_formatting::*;
pub use table_formatting::*;
pub use list_formatting::*;
pub use width_integration::*;
pub use column_mode_uat::*;