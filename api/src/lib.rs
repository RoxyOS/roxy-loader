#![no_std]
#![deny(missing_docs)]

//! Kernel-facing types and macros for use with `roxy-loader`.

/// Boot-time information provided to the kernel.
pub mod bootinfo;
/// Framebuffer types provided to the kernel.
pub mod framebuffer;
/// Macros for defining the kernel entry point.
pub mod kernel_entry;
