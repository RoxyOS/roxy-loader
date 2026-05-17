use crate::{
    build_basic_kernel::build_basic_kernel,
    run_vm::{VMResult, run_vm},
};
use anyhow::Result;
use roxy_loader_utils::build_image::default_image_path;

pub fn run_basic_kernel() -> Result<()> {
    build_basic_kernel()?;
    let result = run_vm(default_image_path()?);
    display_vm_result(result);
    Ok(())
}

fn display_vm_result(result: VMResult) {
    match result {
        Ok(exit_status) => println!("Qemu exited with status {exit_status}"),
        Err(error) => println!("{error}"),
    }
}
