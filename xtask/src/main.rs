use anyhow::Result;
use clap::Parser;

use crate::{
    build_basic_kernel::build_basic_kernel,
    c_api::generate_c_api,
    command::{Args, Command},
    run_basic_kernel::run_basic_kernel,
    test::test,
    utils::chdir_to_workspace_root,
};

pub mod build_basic_kernel;
pub mod c_api;
pub mod command;
pub mod run_basic_kernel;
pub mod run_vm;
pub mod test;
pub mod test_kernel;
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
        Command::CApi => generate_c_api()?,
        Command::Test => test()?,
    }

    Ok(())
}
