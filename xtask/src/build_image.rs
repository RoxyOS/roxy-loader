use anyhow::{Ok, Result};
use image_builder::build_bootloader_image;
use workspace_root::get_workspace_root;

use crate::{build::bootloader_efi_file, utils::cargo_target_dir};

const IMAGE_PATH: &str = "image.img";

pub fn build_image() -> Result<()> {
    let bootloader_image = cargo_target_dir()?.join(IMAGE_PATH);
    build_bootloader_image(bootloader_efi_file()?, bootloader_image)?;
    Ok(())
}
