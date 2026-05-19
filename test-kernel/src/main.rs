#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os_test_framework::run_tests)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;

use roxy_loader_api::{bootinfo::BootInfo, kernel_entry};
use x86_64::instructions::hlt;

use crate::{allocator::init_heap, platform::TestPlatform};

mod allocator;
mod platform;
mod serial;
mod tests;

kernel_entry!(kernel_main);

fn kernel_main(_bootinfo: &BootInfo) -> ! {
    init_heap();
    os_test_framework::init_platform(TestPlatform);
    test_main();
    hlt_loop()
}

fn hlt_loop() -> ! {
    loop {
        hlt();
    }
}

os_test_framework::forward_panic!();
