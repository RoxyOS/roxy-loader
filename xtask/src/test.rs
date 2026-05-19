use anyhow::Result;

use crate::{run_command, test_kernel::run_test_kernel};

const HOST_TEST_PACKAGES: &str = "-p roxy-loader --lib -p roxy-loader-api -p roxy-loader-utils -p xtask";
const HOST_DOCTEST_PACKAGES: &str = "-p roxy-loader-api -p roxy-loader-utils";

pub fn test() -> Result<()> {
    run_command!(concat!("cargo test ", HOST_TEST_PACKAGES));
    run_command!(concat!("cargo test --doc ", HOST_DOCTEST_PACKAGES));
    run_test_kernel()?;
    Ok(())
}
