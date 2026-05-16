#![no_std]
#![no_main]

extern crate alloc;

use core::{mem::transmute, ptr::copy_nonoverlapping, time::Duration};

use alloc::ffi::c_str;
use anyhow::{Context, Result};
use core::result::Result::Ok;
use elfloader::{ElfBinary, ElfLoader, ElfLoaderErr};
use uefi::{
    Status,
    boot::{self, AllocateType, MemoryType, allocate_pages, exit_boot_services},
    entry, println,
    proto::media::fs,
};

use crate::{elf_loader::RoxyElfLoader, utils::read_file};
use uefi::cstr16;

mod elf_loader;
mod utils;

#[entry]
fn main() -> Status {
    match run() {
        Ok(()) => Status::SUCCESS,
        Err(error) => {
            println!("{error}");
            loop {}
        }
    }
}

fn run() -> Result<()> {
    let kernel_file = read_file(cstr16!("\\KERNEL")).context("Failed to read kernel file")?;
    let kernel_elf = ElfBinary::new(&kernel_file)
        .ok()
        .context("Failed to parse kernel elf")?;

    kernel_elf
        .load(&mut RoxyElfLoader)
        .expect("Failed to load kernel elf");

    unsafe {
        exit_boot_services(None);

        let kernel_entry: extern "C" fn() = transmute(kernel_elf.entry_point());

        kernel_entry();
    }

    Ok(())
}
