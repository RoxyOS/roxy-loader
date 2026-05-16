#[derive(Debug, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug, Clone, Copy)]
pub enum Command {
    #[command(
        about = "Build and run the basic kernel, used to check the bootloader starts correctly"
    )]
    RunBasicKernel,
    #[command(about = "Build the basic kernel")]
    BuildBasicKernel,
    #[command(about = "Build the bootloader EFI binary")]
    Build,
    #[command(about = "Build the bootloader disk image")]
    BuildImage,
}
