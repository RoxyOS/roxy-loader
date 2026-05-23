use std::path::PathBuf;

use anyhow::Result;

/// Returns the Cargo target directory for the current workspace.
pub fn cargo_target_dir() -> Result<PathBuf> {
    Ok(cargo_metadata()?.target_directory.into())
}

fn cargo_metadata() -> Result<cargo_metadata::Metadata> {
    Ok(cargo_metadata::MetadataCommand::new().exec()?)
}

// Evaluate workspace root at comptime.
pub(crate) fn static_workspace_root() -> PathBuf {
    let utils_dir: PathBuf = env!("CARGO_MANIFEST_DIR").into();

    utils_dir.join("..")
}
