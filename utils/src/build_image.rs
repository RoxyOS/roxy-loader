use std::{
    env,
    fs::File,
    io::{self, Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::Result;
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};

use crate::utils::cargo_target_dir;

pub fn build_image(kernel_binary: PathBuf) -> Result<PathBuf> {
    const IMAGE_SIZE: u64 = 64 * 1024 * 1024;

    let mut image = open_image()?;

    // truncate
    image.set_len(IMAGE_SIZE)?;

    fatfs::format_volume(&mut image, FormatVolumeOptions::new())?;

    image.seek(SeekFrom::Start(0))?;

    let fs = FileSystem::new(image, FsOptions::new())?;

    let root_dir = fs.root_dir();
    root_dir.create_dir("EFI")?;
    let efi_dir = root_dir.open_dir("EFI")?;
    efi_dir.create_dir("BOOT")?;
    let boot_dir = efi_dir.open_dir("BOOT")?;

    // Copies roxyloader artifact
    let mut src = File::open(roxyloader_artifact())?;
    let mut dst = boot_dir.create_file("BOOTX64.EFI")?;
    io::copy(&mut src, &mut dst)?;

    // Installs kernel binary
    let mut dst = fs.root_dir().create_file("KERNEL")?;
    let mut src = File::open(kernel_binary)?;
    io::copy(&mut src, &mut dst)?;

    default_image_path()
}

pub fn default_image_path() -> Result<PathBuf> {
    const IMAGE_NAME: &str = "image.img";
    Ok(cargo_target_dir()?.join(IMAGE_NAME))
}

fn open_image() -> Result<File> {
    Ok(File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(default_image_path()?)?)
}

fn roxyloader_artifact() -> PathBuf {
    env!("ROXYLOADER_ARTIFACT").into()
}
