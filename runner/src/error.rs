use std::io::{self, ErrorKind};

use thiserror::Error;

pub type Result<T = ()> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Qemu is not installed")]
    NoQemu,

    #[error("Failed to build bootloader image {0}")]
    BootloaderImage(#[from] anyhow::Error),

    #[error("Error when fetching ovmf: {0}")]
    OvmfFetchError(#[from] ovmf_prebuilt::Error),

    #[error("{0}")]
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            ErrorKind::NotFound => Self::NoQemu,
            _ => Error::IoError(err),
        }
    }
}
