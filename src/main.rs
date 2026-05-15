#![no_std]
#![no_main]

use core::time::Duration;

use uefi::{Status, boot, entry, println};

#[entry]
fn main() -> Status {
    println!("hello world!!");
    boot::stall(Duration::from_secs(10));

    Status::SUCCESS
}
