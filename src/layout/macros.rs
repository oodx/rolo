//! Layout module-owned macros per MODULE_SPEC

/// Macro for creating layout configurations
#[macro_export]
macro_rules! layout_config {
    ($width:expr) => {
        $crate::layout::utils::LayoutConfig {
            width: $width,
            gap: 2,
            padding: 1,
        }
    };
    ($width:expr, gap: $gap:expr) => {
        $crate::layout::utils::LayoutConfig {
            width: $width,
            gap: $gap,
            padding: 1,
        }
    };
    ($width:expr, gap: $gap:expr, padding: $padding:expr) => {
        $crate::layout::utils::LayoutConfig {
            width: $width,
            gap: $gap,
            padding: $padding,
        }
    };
}