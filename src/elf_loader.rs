use core::ptr::copy_nonoverlapping;

use elfloader::{ElfLoader, ElfLoaderErr};

use crate::allocation_info::AllocationInfo;

pub struct RoxyElfLoader;

impl ElfLoader for RoxyElfLoader {
    fn allocate(
        &mut self,
        load_headers: elfloader::LoadableHeaders,
    ) -> Result<(), elfloader::ElfLoaderErr> {
        for header in load_headers {
            let allocation = AllocationInfo::from_region(header.virtual_addr(), header.mem_size());
            allocation
                .alloc_zeroed()
                .expect("Failed to allocate page for kernel elf");
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
