#![no_std]
#![no_main]

extern crate alloc;

use core::{mem::transmute, ptr::copy_nonoverlapping, time::Duration};

use alloc::{boxed::Box, ffi::c_str};
use anyhow::{Context, Result, bail};
use core::result::Result::Ok;
use elfloader::{ElfBinary, ElfLoader, ElfLoaderErr};
use roxy_loader_api::BootInfo;
use uefi::{
    Status,
    boot::{self, AllocateType, MemoryType, allocate_pages, exit_boot_services},
    entry, println,
    proto::media::fs,
};

use crate::{
    bootinfo::new_bootinfo, elf_loader::RoxyElfLoader, load_kernel::load_kernel, utils::read_file,
};
use uefi::cstr16;

mod bootinfo;
mod elf_loader;
mod framebuffer;
mod load_kernel;
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
    let bootinfo = Box::leak(Box::new(new_bootinfo()?));

    unsafe {
        exit_boot_services(None);

        let kernel_entry = load_kernel()?;

        kernel_entry(&*bootinfo);
    }

    Ok(())
}
