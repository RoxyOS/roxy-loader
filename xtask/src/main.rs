use anyhow::{Ok, Result};
use clap::Parser;
use workspace_root::get_workspace_root;
use xshell::Shell;

use crate::{
    build_basic_kernel::build_basic_kernel,
    command::{Args, Command},
    run_basic_kernel::run_basic_kernel,
};

pub mod build_basic_kernel;
pub mod command;
pub mod run_basic_kernel;
pub mod run_vm;
pub mod utils;

mod private {
    pub use xshell;
}

fn main() -> Result<()> {
    chdir_to_workspace_root()?;

    let args = Args::parse();

    match args.command {
        Command::RunBasicKernel => run_basic_kernel()?,
        Command::BuildBasicKernel => build_basic_kernel()?,
    }

    Ok(())
}

fn chdir_to_workspace_root() -> Result<()> {
    let workspace_root = get_workspace_root();
    Shell::new()?.change_dir(workspace_root);
    Ok(())
}
