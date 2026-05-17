use anyhow::Result;
use roxy_loader_utils::build_image::build_image;

use crate::{run_command, utils::cargo_target_dir};

pub fn build_basic_kernel() -> Result<()> {
    run_command!("cargo build -p basic-kernel --target x86_64-unknown-none");
    build_image(
        cargo_target_dir()?
            .join("x86_64-unknown-none")
            .join("debug")
            .join("basic-kernel"),
    )?;
    Ok(())
}
