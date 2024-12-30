//! This file provides support for DPI.

use crate::get_dpi_scale;

/// Calculate the platform-independent pixel value from the given value.
pub fn px(x: f32) -> i32 {
    (x * get_dpi_scale()) as i32
}

/// Calculate the platform-independent pixel value from the given em value.
pub fn em(x: f32) -> i32 {
    px(x * 16.0)
}
