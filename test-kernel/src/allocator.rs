use core::ptr::addr_of_mut;

use buddy_system_allocator::LockedHeap;

const HEAP_SIZE: usize = 64 * 1024;
const HEAP_ORDER: usize = 16;

#[global_allocator]
static ALLOCATOR: LockedHeap<HEAP_ORDER> = LockedHeap::empty();

static mut HEAP_SPACE: [u8; HEAP_SIZE] = [0; HEAP_SIZE];

pub fn init_heap() {
    unsafe {
        ALLOCATOR
            .lock()
            .init(addr_of_mut!(HEAP_SPACE) as *mut u8 as usize, HEAP_SIZE);
    }
}
