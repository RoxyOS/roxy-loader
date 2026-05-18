#[repr(C)]
pub struct Framebuffer {
    pub ptr: *mut u8,
    pub size: usize,
    pub stride: usize,
}

#[cfg(test)]
mod tests {
    use super::Framebuffer;
    use core::{
        mem::{MaybeUninit, align_of, size_of},
        ptr::addr_of,
    };

    #[test]
    fn layout_is_stable() {
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
}
