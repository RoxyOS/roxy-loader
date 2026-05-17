use alloc::vec::Vec;
use anyhow::{Result, anyhow};
use uefi::{
    boot::{
        ScopedProtocol, get_handle_for_protocol, get_image_file_system, image_handle,
        open_protocol_exclusive,
    },
    cstr16,
    fs::{self, FileSystem, Path},
    proto::Protocol,
};

pub fn read_file(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    Ok(FileSystem::new(get_image_file_system(image_handle())?).read(path)?)
}

pub fn get_and_open_protocol<T: Protocol>() -> Result<ScopedProtocol<T>> {
    let handle = get_handle_for_protocol::<T>()?;
    open_protocol_exclusive::<T>(handle).map_err(|err| anyhow!("{err}"))
}
