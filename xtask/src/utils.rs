use std::{
    io,
    path::PathBuf,
    process::{Command, ExitStatus},
};

use anyhow::Result;
use qemu_command_builder::{QemuInstanceForX86_64, to_command::ToCommand};
use workspace_root::get_workspace_root;
use xshell::Shell;

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

pub fn run_qemu(qemu_command: QemuInstanceForX86_64) -> Result<ExitStatus> {
    let argv = qemu_command.to_command();
    let [program, args @ ..] = argv.as_slice() else {
        anyhow::bail!("run_qemu: empty qemu command")
    };

    Ok(Command::new(program)
        .args(args)
        .stdout(io::stdout())
        .spawn()?
        .wait()?)
}

pub fn chdir_to_workspace_root() -> Result<()> {
    let workspace_root = get_workspace_root();
    Shell::new()?.change_dir(workspace_root);
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn publishable_workspace_crates_have_the_same_version() -> Result<()> {
        let metadata = cargo_metadata()?;
        let workspace_packages = metadata.workspace_packages();
        let versions = workspace_packages
            .iter()
            .filter(|package| package.publish.as_ref().is_none_or(|publish| !publish.is_empty()))
            .fold(BTreeMap::new(), |mut versions, package| {
                versions
                    .entry(package.version.to_string())
                    .or_insert_with(Vec::new)
                    .push(package.name.to_string());
                versions
            });

        assert_eq!(
            versions.len(),
            1,
            "publishable workspace crates must share one version: {versions:#?}"
        );

        Ok(())
    }
}
