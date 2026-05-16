use std::{fs::File, io};

use anyhow::{Ok, Result};
use fatfs::{FileSystem, FsOptions};

use crate::{
    build_image::{bootloader_image, build_image},
    run_command,
    utils::cargo_target_dir,
};

pub fn build_basic_kernel() -> Result<()> {
    run_command!("cargo build -p basic-kernel --target x86_64-unknown-none");
    install_to_image()?;
    Ok(())
}

fn install_to_image() -> Result<()> {
    build_image()?;

    let bootloader_image_file = File::options()
        .read(true)
        .write(true)
        .open(bootloader_image()?)?;
    let fs = FileSystem::new(bootloader_image_file, FsOptions::new())?;

    let kernel_path = cargo_target_dir()?
        .join("x86_64-unknown-none")
        .join("debug")
        .join("basic-kernel");

    let mut kernel_dst_file = fs.root_dir().create_file("KERNEL")?;
    let mut kernel_src_file = File::open(kernel_path)?;

    io::copy(&mut kernel_src_file, &mut kernel_dst_file)?;

    Ok(())
}
