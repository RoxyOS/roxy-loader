use core::mem::transmute;

use anyhow::{Context, Result};
use roxy_loader_api::bootinfo::BootInfo;
use uefi::cstr16;

use crate::{
    elf_loader::RoxyElfLoader,
    elf_processing::{kernel_entry_point, parse_kernel_elf, validate_kernel_elf},
    utils::read_file,
};

pub type KernelEntry = extern "sysv64" fn(*const BootInfo);

pub fn load_kernel() -> Result<KernelEntry> {
    let kernel_file = read_file(cstr16!("\\KERNEL")).context("Failed to read kernel file")?;
    let kernel_elf = parse_kernel_elf(&kernel_file)?;
    validate_kernel_elf(&kernel_elf)?;

    kernel_elf
        .load(&mut RoxyElfLoader)
        .expect("Failed to load kernel elf");

    unsafe { Ok(transmute(kernel_entry_point(&kernel_elf))) }
}
