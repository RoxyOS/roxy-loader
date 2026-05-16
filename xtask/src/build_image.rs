use std::path::PathBuf;

use anyhow::{Ok, Result};
use roxy_loader_utils::bootloader_image::build_bootloader_image;
use workspace_root::get_workspace_root;

use crate::{
    build::{bootloader_efi_file, build},
    utils::cargo_target_dir,
};

const IMAGE_PATH: &str = "image.img";

pub fn build_image() -> Result<()> {
    build()?;
    build_bootloader_image(bootloader_efi_file()?, bootloader_image()?)?;
    Ok(())
}

pub fn bootloader_image() -> Result<PathBuf> {
    Ok(cargo_target_dir()?.join(IMAGE_PATH))
}
