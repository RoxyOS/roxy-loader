use anyhow::{Ok, Result};

use crate::run_command;

pub fn build_basic_kernel() -> Result<()> {
    run_command!("cargo build -p basic-kernel --target x86_64-unknown-none");
    Ok(())
}
