use anyhow::Result;
use workspace_root::get_workspace_root;

use crate::run_command;

pub fn build() -> Result<()> {
    run_command!("cargo build --target x86_64-unknown-uefi");
    Ok(())
}
