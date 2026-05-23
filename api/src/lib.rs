#![no_std]

//! Kernel-facing types and macros for use with `roxy-loader`.
//!
//! If you want to use `roxy-loader`, see the [Github README](https://github.com/RoxyOS/roxy-loader).
//! These docs are not intended to be guides.

pub mod bootinfo;
pub mod framebuffer;
pub mod kernel_entry;
