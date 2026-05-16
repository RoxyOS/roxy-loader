use core::ptr::copy_nonoverlapping;

use elfloader::{ElfLoader, ElfLoaderErr};
use uefi::boot::{AllocateType, MemoryType, allocate_pages};

pub struct RoxyElfLoader;

impl ElfLoader for RoxyElfLoader {
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
