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
    #[command(about = "Run host-side unit tests")]
    Test,
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_run_basic_kernel_command() {
        let args = Args::parse_from(["xtask", "run-basic-kernel"]);
        assert!(matches!(args.command, Command::RunBasicKernel));
    }

    #[test]
    fn parses_build_basic_kernel_command() {
        let args = Args::parse_from(["xtask", "build-basic-kernel"]);
        assert!(matches!(args.command, Command::BuildBasicKernel));
    }

    #[test]
    fn parses_xtest_command() {
        let args = Args::parse_from(["xtask", "test"]);
        assert!(matches!(args.command, Command::Test));
    }
}
