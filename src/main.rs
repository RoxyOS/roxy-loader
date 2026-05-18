#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use uefi::allocator::Allocator;
use uefi::{Status, boot::exit_boot_services, entry, println};

use roxy_loader::{bootinfo::new_bootinfo, load_kernel::load_kernel};

#[global_allocator]
static GLOBAL_ALLOCATOR: Allocator = Allocator;

#[entry]
fn main() -> Status {
    match run_main() {
        Ok(()) => Status::SUCCESS,
        Err(error) => {
            println!("{error}");
            loop {}
        }
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}

fn run_main() -> anyhow::Result<()> {
    let bootinfo = alloc::boxed::Box::leak(alloc::boxed::Box::new(new_bootinfo()?));

    unsafe {
        let kernel_entry = load_kernel()?;

        let _ = exit_boot_services(None);

        kernel_entry(&*bootinfo);
    }

    Ok(())
}
