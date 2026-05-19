/// A framebuffer provided to the kernel at boot time.
///
/// Kernels can use this value to find the framebuffer memory and understand its
/// basic layout.
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Framebuffer {
    ptr: usize,
    /// The total size of the framebuffer in bytes.
    pub size: usize,
    /// The number of pixels in each row of the framebuffer.
    pub stride: usize,
}

impl Framebuffer {
    /// Creates a framebuffer value.
    pub fn new(ptr: *mut u8, size: usize, stride: usize) -> Self {
        Self {
            ptr: ptr as usize,
            size,
            stride,
        }
    }

    /// Returns a pointer to the framebuffer memory.
    pub fn ptr(&self) -> *mut u8 {
        self.ptr as *mut u8
    }
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
