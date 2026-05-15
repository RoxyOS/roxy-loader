use std::{
    io::{self, ErrorKind},
    process::{Command, ExitStatus},
};

use qemu_command_builder::{QemuInstanceForX86_64, to_command::ToCommand};

use crate::error::*;

pub fn run_qemu(qemu_command: QemuInstanceForX86_64) -> Result<ExitStatus> {
    let argv = qemu_command.to_command();
    let [program, args @ ..] = argv.as_slice() else {
        return Err(Error::IoError(io::Error::new(
            ErrorKind::InvalidInput,
            "empty qemu command",
        )));
    };

    Ok(Command::new(program)
        .args(args)
        .stdout(io::stdout())
        .spawn()?
        .wait()?)
}
