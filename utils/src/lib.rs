#![deny(missing_docs)]

//! Host-side utilities for working with `roxy-loader`.
//!
//! If you want to use `roxy-loader`, see the [Github README](https://github.com/RoxyOS/roxy-loader).
//! These docs are not intended to be guides.

/// Utilities for building bootable disk images.
pub mod build_image;
/// Utilities for discovering Cargo output paths.
pub mod utils;
