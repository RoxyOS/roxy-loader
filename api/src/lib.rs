#![no_std]

#[repr(C)]
pub struct Framebuffer {
    pub ptr: *mut u8,
    pub size: usize,
    pub stride: usize,
}

#[repr(C)]
pub struct BootInfo {
    pub framebuffer: Framebuffer,
}
