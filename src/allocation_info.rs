use core::ptr::write_bytes;
use uefi::boot::{AllocateType, MemoryType, allocate_pages};

pub const PAGE_SIZE: u64 = 4096;

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AllocationInfo {
    pub page_base: u64,
    pub page_offset: u64,
    pub alloc_size: u64,
    pub pages: usize,
}

impl AllocationInfo {
    #[must_use]
    pub fn from_region(addr: u64, size: u64) -> Self {
        let page_base = addr & !(PAGE_SIZE - 1);
        let page_offset = addr - page_base;
        let alloc_size = page_offset + size;
        let pages = alloc_size.div_ceil(PAGE_SIZE) as usize;

        Self {
            page_base,
            page_offset,
            alloc_size,
            pages,
        }
    }

    pub fn alloc(self) -> uefi::Result<*mut u8> {
        Ok(allocate_pages(
            AllocateType::Address(self.page_base),
            MemoryType::LOADER_DATA,
            self.pages,
        )?
        .as_ptr())
    }

    pub fn alloc_zeroed(self) -> uefi::Result<*mut u8> {
        let ptr = self.alloc()?;

        // The loader expects a fully zeroed backing region before segment contents are copied in.
        unsafe {
            write_bytes(ptr, 0, self.pages * PAGE_SIZE as usize);
        }

        Ok(ptr)
    }
}

#[cfg(test)]
mod tests {
    use super::{AllocationInfo, PAGE_SIZE};

    #[test]
    fn aligned_single_page_allocation() {
        assert_eq!(
            AllocationInfo::from_region(0x2000, 1),
            AllocationInfo {
                page_base: 0x2000,
                page_offset: 0,
                alloc_size: 1,
                pages: 1,
            }
        );
    }

    #[test]
    fn unaligned_address_accounts_for_offset() {
        assert_eq!(
            AllocationInfo::from_region(0x2123, 0x100),
            AllocationInfo {
                page_base: 0x2000,
                page_offset: 0x123,
                alloc_size: 0x223,
                pages: 1,
            }
        );
    }

    #[test]
    fn crossing_page_boundary_allocates_two_pages() {
        assert_eq!(
            AllocationInfo::from_region(PAGE_SIZE - 1, 2),
            AllocationInfo {
                page_base: 0,
                page_offset: PAGE_SIZE - 1,
                alloc_size: PAGE_SIZE + 1,
                pages: 2,
            }
        );
    }

    #[test]
    fn exact_multiple_of_page_size_does_not_round_up_extra() {
        assert_eq!(
            AllocationInfo::from_region(0x4000, PAGE_SIZE * 2),
            AllocationInfo {
                page_base: 0x4000,
                page_offset: 0,
                alloc_size: PAGE_SIZE * 2,
                pages: 2,
            }
        );
    }

    #[test]
    fn zero_sized_region_allocates_zero_pages() {
        assert_eq!(
            AllocationInfo::from_region(0x4000, 0),
            AllocationInfo {
                page_base: 0x4000,
                page_offset: 0,
                alloc_size: 0,
                pages: 0,
            }
        );
    }
}
