use clap::Parser;

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

fn main() {
    let args = Args::parse();

    match args.command {
        Command::RunBasicKernel => run_basic_kernel(),
        Command::Build => build(),
        Command::BuildImage => build_image(),
    }
}
