#![no_std]
#![no_main]

extern crate alloc;

use alloc::boxed::Box;
use anyhow::Result;
use uefi::{Status, boot::exit_boot_services, entry, println};

use crate::{bootinfo::new_bootinfo, load_kernel::load_kernel};

mod allocation_info;
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
        let kernel_entry = load_kernel()?;

        let _ = exit_boot_services(None);

        kernel_entry(&*bootinfo);
    }

    Ok(())
}
