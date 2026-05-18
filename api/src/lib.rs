#![no_std]

pub mod kernel_entry;

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

#[cfg(test)]
mod tests {
    use super::*;
    use core::{
        mem::{MaybeUninit, align_of, size_of},
        ptr::addr_of,
    };

    #[test]
    fn framebuffer_layout_is_stable() {
        assert_eq!(
            size_of::<Framebuffer>(),
            size_of::<(*mut u8, usize, usize)>()
        );
        assert_eq!(align_of::<Framebuffer>(), align_of::<usize>());

        let framebuffer = MaybeUninit::<Framebuffer>::uninit();
        let base = framebuffer.as_ptr();

        // ABI stability matters because the loader and kernel share this struct across a raw boundary.
        unsafe {
            assert_eq!(addr_of!((*base).ptr) as usize - base as usize, 0);
            assert_eq!(
                addr_of!((*base).size) as usize - base as usize,
                size_of::<*mut u8>()
            );
            assert_eq!(
                addr_of!((*base).stride) as usize - base as usize,
                size_of::<*mut u8>() + size_of::<usize>()
            );
        }
    }

    #[test]
    fn bootinfo_layout_is_stable() {
        assert_eq!(size_of::<BootInfo>(), size_of::<Framebuffer>());
        assert_eq!(align_of::<BootInfo>(), align_of::<Framebuffer>());

        let bootinfo = MaybeUninit::<BootInfo>::uninit();
        let base = bootinfo.as_ptr();

        unsafe {
            assert_eq!(addr_of!((*base).framebuffer) as usize - base as usize, 0);
        }
    }
}
