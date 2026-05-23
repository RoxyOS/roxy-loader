use uefi::proto::console::gop::PixelFormat as UefiPixelFormat;

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
    /// Pixel format of the framebuffer.
    pub pixel_format: PixelFormat,
    /// Width of the framebuffer in pixels.
    pub width: usize,
    /// Height of the framebuffer in pixels.
    pub height: usize,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum PixelFormat {
    Rgb,
    Bgr,
    Bitmask,
    BltOnly,
}

impl From<UefiPixelFormat> for PixelFormat {
    fn from(value: UefiPixelFormat) -> Self {
        match value {
            UefiPixelFormat::Rgb => Self::Rgb,
            UefiPixelFormat::Bgr => Self::Bgr,
            UefiPixelFormat::Bitmask => Self::Bitmask,
            UefiPixelFormat::BltOnly => Self::BltOnly,
        }
    }
}

type Resolution = (usize, usize);

impl Framebuffer {
    /// Bytes each pixel will take on the framebuffer.
    pub fn bytes_per_pixel(&self) -> usize {
        self.size / (self.stride * self.height)
    }

    /// Creates a framebuffer value.
    pub fn new(
        ptr: *mut u8,
        size: usize,
        stride: usize,
        pixel_format: impl Into<PixelFormat>,
        resolution: Resolution,
    ) -> Self {
        let (width, height) = resolution;

        Self {
            ptr: ptr as usize,
            size,
            stride,
            width,
            height,
            pixel_format: pixel_format.into(),
        }
    }

    /// Returns a pointer to the framebuffer memory.
    pub fn ptr(&self) -> *mut u8 {
        self.ptr as *mut u8
    }
}

#[cfg(test)]
mod tests {
    use super::{Framebuffer, PixelFormat};
    use core::{
        mem::{MaybeUninit, align_of, size_of},
        ptr::addr_of,
    };

    #[test]
    fn layout_is_stable() {
        assert_eq!(size_of::<Framebuffer>(), 48);
        assert_eq!(align_of::<Framebuffer>(), align_of::<usize>());
        assert_eq!(size_of::<PixelFormat>(), 4);
        assert_eq!(align_of::<PixelFormat>(), 4);

        let framebuffer = MaybeUninit::<Framebuffer>::uninit();
        let base = framebuffer.as_ptr();

        // ABI stability matters because the loader and kernel share this struct across a raw boundary.
        unsafe {
            assert_eq!(addr_of!((*base).ptr) as usize - base as usize, 0);
            assert_eq!(
                addr_of!((*base).size) as usize - base as usize,
                size_of::<usize>()
            );
            assert_eq!(
                addr_of!((*base).stride) as usize - base as usize,
                size_of::<usize>() * 2
            );
            assert_eq!(
                addr_of!((*base).pixel_format) as usize - base as usize,
                size_of::<usize>() * 3
            );
            assert_eq!(
                addr_of!((*base).width) as usize - base as usize,
                size_of::<usize>() * 4
            );
            assert_eq!(
                addr_of!((*base).height) as usize - base as usize,
                size_of::<usize>() * 5
            );
        }
    }
}
