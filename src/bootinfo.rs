use anyhow::Result;
use roxy_loader_api::bootinfo::BootInfo;

use crate::framebuffer::new_framebuffer;

pub fn new_bootinfo() -> Result<BootInfo> {
    Ok(BootInfo {
        framebuffer: new_framebuffer()?,
    })
}
