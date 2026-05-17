use anyhow::Result;
use roxy_loader_api::Framebuffer;
use uefi::{boot::get_handle_for_protocol, proto::console::gop::GraphicsOutput};

use crate::utils::get_and_open_protocol;

pub fn new_framebuffer() -> Result<roxy_loader_api::Framebuffer> {
    let mut gop = get_and_open_protocol::<GraphicsOutput>()?;

    let mode = gop.current_mode_info();
    let mut fb = gop.frame_buffer();

    Ok(roxy_loader_api::Framebuffer {
        ptr: fb.as_mut_ptr(),
        size: fb.size(),
        stride: mode.stride(),
    })
}
