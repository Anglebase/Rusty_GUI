//! This file provides support for DPI.

use crate::get_dpi_scale;

/// Calculate the platform-independent pixel value from the given value.
/// It will return current screen's scale factor relative to 96 DPI.
pub fn px(x: f32) -> i32 {
    (x * get_dpi_scale()) as i32
}

/// Calculate the platform-independent pixel value from the given em value.
/// It is equivalent to calling `px(x * 16.0)`.
pub fn em(x: f32) -> i32 {
    px(x * 16.0)
}
