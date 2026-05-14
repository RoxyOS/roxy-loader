#![no_std]
#![no_main]

use uefi::{Status, entry, println};

#[entry]
fn main() -> Status {
    println!("hello world!!");

    Status::SUCCESS
}
