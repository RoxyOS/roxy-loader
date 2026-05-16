#![no_std]
#![no_main]

extern crate alloc;

use core::{ptr::copy_nonoverlapping, time::Duration};

use elfloader::{ElfLoader, ElfLoaderErr};
use uefi::{
    Status,
    boot::{self, AllocateType, MemoryType, allocate_pages, exit_boot_services},
    entry, println,
    proto::media::fs,
};

mod elf_loader;
mod utils;

#[entry]
fn main() -> Status {
    println!("hello world!!");
    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}
