// This is the entry point of your Rust library.
// When adding new code to your project, note that only items used
// here will be transformed to their Dart equivalents.

use crate::domain_color;

// The convention for Rust identifiers is the snake_case,
// and they are automatically converted to camelCase on the Dart side.
pub fn rust_release_mode() -> bool {
    cfg!(not(debug_assertions))
}

#[inline(always)]
pub fn color_bmp(
    width: usize,
    height: usize,
    fun_str: String,
    options: domain_color::DCOptions,
) -> Vec<u8> {
    domain_color::color_bmp(width, height, &fun_str, options)
}
