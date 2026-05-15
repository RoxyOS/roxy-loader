use anyhow::{Ok, Result};
use clap::Parser;
use workspace_root::get_workspace_root;
use xshell::Shell;

use crate::{
    build::build,
    build_image::build_image,
    command::{Args, Command},
    run_basic_kernel::run_basic_kernel,
};

pub mod build;
pub mod build_image;
pub mod command;
pub mod run_basic_kernel;

fn main() -> Result<()> {
    chdir_to_workspace_root();

    let args = Args::parse();

    match args.command {
        Command::RunBasicKernel => run_basic_kernel()?,
        Command::Build => build()?,
        Command::BuildImage => build_image()?,
    }

    Ok(())
}

fn chdir_to_workspace_root() {
    let workspace_root = get_workspace_root();
    Shell::new().unwrap().change_dir(workspace_root);
}
