#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[unsafe(no_mangle)]
extern "sysv64" fn _start() {
    loop {}
}

#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    loop {}
}
