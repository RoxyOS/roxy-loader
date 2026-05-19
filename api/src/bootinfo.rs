use crate::framebuffer::Framebuffer;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct BootInfo {
    pub framebuffer: Framebuffer,
}

#[cfg(test)]
mod tests {
    use super::BootInfo;
    use crate::framebuffer::Framebuffer;
    use core::{
        mem::{MaybeUninit, align_of, size_of},
        ptr::addr_of,
    };

    #[test]
    fn layout_is_stable() {
        assert_eq!(size_of::<BootInfo>(), size_of::<Framebuffer>());
        assert_eq!(align_of::<BootInfo>(), align_of::<Framebuffer>());

        let bootinfo = MaybeUninit::<BootInfo>::uninit();
        let base = bootinfo.as_ptr();

        unsafe {
            assert_eq!(addr_of!((*base).framebuffer) as usize - base as usize, 0);
        }
    }
}
