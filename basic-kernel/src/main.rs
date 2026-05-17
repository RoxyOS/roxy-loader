#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

use roxy_loader_api::BootInfo;

#[unsafe(no_mangle)]
extern "sysv64" fn _start(bootinfo: *const BootInfo) {
    unsafe {
        let bootinfo = &*bootinfo;

        let ptr = bootinfo.framebuffer.ptr;

        for i in 0..1000000 {
            ptr.add(i).write_volatile(69);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    loop {}
}
