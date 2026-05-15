use std::path::PathBuf;

use anyhow::Result;
use cargo_metadata::MetadataCommand;

#[macro_export]
macro_rules! run_command {
    ($cmd:literal) => {
        let sh = $crate::private::xshell::Shell::new()?;
        $crate::private::xshell::cmd!(sh, $cmd).run()?;
    };
}

pub fn cargo_target_dir() -> Result<PathBuf> {
    Ok(cargo_metadata()?.target_directory.into())
}

fn cargo_metadata() -> Result<cargo_metadata::Metadata> {
    Ok(cargo_metadata::MetadataCommand::new().exec()?)
}
