use core::ptr::{copy_nonoverlapping, write_bytes};

use elfloader::{ElfLoader, ElfLoaderErr};
use uefi::boot::{AllocateType, MemoryType, allocate_pages};

pub struct RoxyElfLoader;

impl ElfLoader for RoxyElfLoader {
    fn allocate(
        &mut self,
        load_headers: elfloader::LoadableHeaders,
    ) -> Result<(), elfloader::ElfLoaderErr> {
        for header in load_headers {
            const PAGE_SIZE: u64 = 4096;

            let addr = header.virtual_addr();
            let size = header.mem_size();

            let page_base = addr & !(PAGE_SIZE - 1);
            let page_offset = addr - page_base;
            let alloc_size = page_offset + size;
            let pages = alloc_size.div_ceil(PAGE_SIZE) as usize;

            let allocated_ptr = allocate_pages(
                AllocateType::Address(page_base),
                MemoryType::LOADER_DATA,
                pages,
            )
            .expect("Failed to allocate page for kernel elf")
            .as_ptr();

            // Zero allocated pages
            unsafe {
                write_bytes(allocated_ptr, 0, pages * PAGE_SIZE as usize);
            }
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
