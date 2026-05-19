use anyhow::Result;

use crate::{run_command, test_kernel::run_test_kernel};

pub fn test() -> Result<()> {
    run_command!("cargo test -p roxy-loader --lib");
    run_command!("cargo test -p roxy-loader-api -p roxy-loader-utils -p xtask");
    run_command!("cargo test --doc -p roxy-loader-api -p roxy-loader-utils");
    run_test_kernel()?;
    Ok(())
}
