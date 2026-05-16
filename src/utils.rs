use alloc::vec::Vec;
use anyhow::Result;
use uefi::{
    boot::{get_image_file_system, image_handle},
    cstr16,
    fs::{self, FileSystem, Path},
};

pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    Ok(FileSystem::new(get_image_file_system(image_handle())?).read(path)?)
}
