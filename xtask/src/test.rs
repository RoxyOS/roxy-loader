use anyhow::Result;

use crate::run_command;

pub fn test() -> Result<()> {
    run_command!("cargo test -p roxy-loader-api -p roxy-loader-utils -p xtask");
    Ok(())
}
