use anyhow::Result;
use roxy_loader_api::framebuffer::Framebuffer;
use uefi::{boot::get_handle_for_protocol, proto::console::gop::GraphicsOutput};

use crate::utils::get_and_open_protocol;

pub fn new_framebuffer() -> Result<Framebuffer> {
    let mut gop = get_and_open_protocol::<GraphicsOutput>()?;

    let mode = gop.current_mode_info();
    let mut fb = gop.frame_buffer();

    Ok(Framebuffer {
        ptr: fb.as_mut_ptr(),
        size: fb.size(),
        stride: mode.stride(),
    })
}
