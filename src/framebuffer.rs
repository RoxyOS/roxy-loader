use anyhow::Result;
use roxy_loader_api::framebuffer::{Framebuffer, PixelFormat};
use uefi::proto::console::gop::{GraphicsOutput, PixelFormat as UefiPixelFormat};

use crate::utils::get_and_open_protocol;

pub fn new_framebuffer() -> Result<Framebuffer> {
    let mut gop = get_and_open_protocol::<GraphicsOutput>()?;

    let mode = gop.current_mode_info();
    let mut fb = gop.frame_buffer();

    Ok(Framebuffer::new(
        fb.as_mut_ptr(),
        fb.size(),
        mode.stride(),
        mode.pixel_format(),
        mode.resolution(),
        4,
    ))
}
