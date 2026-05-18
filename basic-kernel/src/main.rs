#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use roxy_loader_api::{BootInfo, kernel_entry};

kernel_entry!(kernel_main);

fn kernel_main(bootinfo: &BootInfo) -> ! {
    unsafe {
        let ptr = bootinfo.framebuffer.ptr;

        for i in 0..1000000 {
            ptr.add(i).write_volatile(69);
        }

        loop {}
    }
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    loop {}
}
