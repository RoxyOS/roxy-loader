use anyhow::Result;

use crate::{run_command, test_kernel::run_test_kernel};

pub fn test() -> Result<()> {
    run_command!(
        "cargo test -p roxy-loader --lib -p roxy-loader-api -p roxy-loader-utils -p xtask"
    );
    run_test_kernel()?;
    Ok(())
}
