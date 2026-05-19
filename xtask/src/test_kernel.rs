use std::{path::PathBuf, process::ExitStatus};

use anyhow::{Context, Result, bail};
use escargot::CargoBuild;
use roxy_loader_utils::build_image::build_image;

use crate::run_vm::{TEST_FAILURE_EXIT_STATUS, TEST_SUCCESS_EXIT_STATUS, run_test_vm};

pub fn run_test_kernel() -> Result<()> {
    let artifact = build_test_kernel()?;
    build_image(artifact)?;

    let status = run_test_vm()?;
    process_qemu_status(status)
}

fn build_test_kernel() -> Result<PathBuf> {
    let artifacts = CargoBuild::new()
        .package("test-kernel")
        .tests()
        .target("x86_64-unknown-none")
        .current_release()
        .run_tests()
        .context("Failed to build test kernel")?;

    for artifact in artifacts {
        let artifact = artifact.context("Failed to read test kernel artifact")?;
        if artifact.kind() == "bin" && artifact.name() == "test-kernel" {
            return Ok(artifact.path().to_path_buf());
        }
    }

    bail!("No test kernel artifact found")
}

fn process_qemu_status(status: ExitStatus) -> Result<()> {
    match status.code() {
        Some(TEST_SUCCESS_EXIT_STATUS) => Ok(()),
        Some(TEST_FAILURE_EXIT_STATUS) => bail!("Kernel test failed"),
        Some(code) => bail!("test-kernel exited with unexpected status {code}"),
        None => bail!("test-kernel did not exit cleanly"),
    }
}
