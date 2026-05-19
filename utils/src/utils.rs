use std::path::PathBuf;

use anyhow::Result;

/// Returns the Cargo target directory for the current workspace.
pub fn cargo_target_dir() -> Result<PathBuf> {
    Ok(cargo_metadata()?.target_directory.into())
}

fn cargo_metadata() -> Result<cargo_metadata::Metadata> {
    Ok(cargo_metadata::MetadataCommand::new().exec()?)
}
