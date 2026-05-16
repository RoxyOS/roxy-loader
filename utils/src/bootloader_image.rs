use std::{
    fs::File,
    io::{self, Seek, SeekFrom},
    path::PathBuf,
};

use anyhow::Result;
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};

const BOOTLOADER_IMAGE_SIZE: u64 = 64 * 1024 * 1024;

pub fn build_bootloader_image(
    bootloader_efi_file: PathBuf,
    bootloader_image_path: PathBuf,
) -> Result<()> {
    let mut bootloader_image = File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(bootloader_image_path)?;

    bootloader_image.set_len(BOOTLOADER_IMAGE_SIZE)?;

    fatfs::format_volume(&mut bootloader_image, FormatVolumeOptions::new())?;

    bootloader_image.seek(SeekFrom::Start(0))?;

    let fs = FileSystem::new(bootloader_image, FsOptions::new())?;

    let root_dir = fs.root_dir();
    root_dir.create_dir("EFI")?;
    let efi_dir = root_dir.open_dir("EFI")?;
    efi_dir.create_dir("BOOT")?;
    let boot_dir = efi_dir.open_dir("BOOT")?;

    let mut src = File::open(bootloader_efi_file)?;
    let mut dst = boot_dir.create_file("BOOTX64.EFI")?;
    io::copy(&mut src, &mut dst)?;

    Ok(())
}
