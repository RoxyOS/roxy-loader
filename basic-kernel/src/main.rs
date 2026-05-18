#![no_std]
#![no_main]

use core::panic::PanicInfo;

use roxy_loader_api::{bootinfo::BootInfo, kernel_entry};

kernel_entry!(kernel_main);

fn kernel_main(bootinfo: &BootInfo) -> ! {
    unsafe {
        let ptr = bootinfo.framebuffer.ptr;

        for i in 0..bootinfo.framebuffer.size {
            ptr.add(i).write_volatile(69);
        }

        loop {}
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}
