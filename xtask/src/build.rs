use std::path::PathBuf;

use anyhow::Result;
use workspace_root::get_workspace_root;

use crate::{run_command, utils::cargo_target_dir};

pub fn build() -> Result<()> {
    run_command!("cargo build --target x86_64-unknown-uefi");
    Ok(())
}

pub fn bootloader_efi_file() -> Result<PathBuf> {
    Ok(cargo_target_dir()?
        .join("x86_64-unknown-uefi")
        .join("debug")
        .join("roxy-loader.efi"))
}
