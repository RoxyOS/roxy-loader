use core::mem::transmute;

use anyhow::{Context, Result, bail};
use elfloader::ElfBinary;
use roxy_loader_api::BootInfo;
use uefi::{boot::exit_boot_services, cstr16};

use crate::{elf_loader::RoxyElfLoader, utils::read_file};

pub type KernelEntry = extern "sysv64" fn(*const BootInfo);

pub fn load_kernel() -> Result<KernelEntry> {
    let kernel_file = read_file(cstr16!("\\KERNEL")).context("Failed to read kernel file")?;
    let kernel_elf = ElfBinary::new(&kernel_file)
        .ok()
        .context("Failed to parse kernel elf")?;

    if kernel_elf.is_pie() {
        bail!("PIE kernels are not supported.");
    }

    kernel_elf
        .load(&mut RoxyElfLoader)
        .expect("Failed to load kernel elf");

    unsafe { Ok(transmute(kernel_elf.entry_point())) }
}
