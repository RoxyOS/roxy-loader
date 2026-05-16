#![no_std]
#![no_main]

use core::{ptr::copy_nonoverlapping, time::Duration};

use elfloader::{ElfLoader, ElfLoaderErr};
use uefi::{
    Status,
    boot::{self, AllocateType, MemoryType, allocate_pages, exit_boot_services},
    entry, println,
    proto::media::fs,
};

#[entry]
fn main() -> Status {
    println!("hello world!!");
    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}

struct RoxyLoader;

impl ElfLoader for RoxyLoader {
    fn allocate(
        &mut self,
        load_headers: elfloader::LoadableHeaders,
    ) -> Result<(), elfloader::ElfLoaderErr> {
        for header in load_headers {
            let addr = header.physical_addr();
            let size = header.mem_size();
            let pages = size.div_ceil(4096) as usize;

            allocate_pages(AllocateType::Address(addr), MemoryType::LOADER_DATA, pages)
                .map_err(|_| ElfLoaderErr::OutOfMemory)?;
        }

        Ok(())
    }

    fn load(
        &mut self,
        _flags: elfloader::Flags,
        base: elfloader::VAddr,
        region: &[u8],
    ) -> Result<(), ElfLoaderErr> {
        unsafe {
            copy_nonoverlapping(region.as_ptr(), base as *mut u8, region.len());
        }

        Ok(())
    }

    fn relocate(&mut self, _entry: elfloader::RelocationEntry) -> Result<(), ElfLoaderErr> {
        panic!("Relocate is not supported.")
    }
}
