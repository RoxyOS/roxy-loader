#![no_std]

//! Kernel-facing types and macros for use with `roxy-loader`.
//!
//! If you want to use `roxy-loader`, see the [Github README](https://github.com/RoxyOS/roxy-loader).
//! These docs are not intended to be guides.

/// Boot-time information provided to the kernel.
pub mod bootinfo;
/// Framebuffer types provided to the kernel.
pub mod framebuffer;
/// Macros for defining the kernel entry point.
pub mod kernel_entry;
