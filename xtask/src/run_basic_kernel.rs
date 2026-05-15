use anyhow::Result;

use crate::{
    build_image::{bootloader_image, build_image},
    run_vm::{VMResult, run_vm},
};

pub fn run_basic_kernel() -> Result<()> {
    build_image()?;
    let result = run_vm(bootloader_image()?);
    display_vm_result(result);
    Ok(())
}

fn display_vm_result(result: VMResult) {
    match result {
        Ok(exit_status) => println!("Qemu exited with status {exit_status}"),
        Err(error) => println!("{error}"),
    }
}
