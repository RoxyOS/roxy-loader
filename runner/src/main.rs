use std::{
    env::var,
    fmt::Display,
    io::{self, ErrorKind, Stdout},
    path::PathBuf,
    process::{Command, ExitStatus, Stdio},
};

use ovmf_prebuilt::{Arch, FileType};
use qemu_command_builder::{
    QemuInstanceBase, QemuInstanceForX86_64,
    args::drive::{Drive, DriveBuilder, DriveInterface},
    common::OnOff,
    to_command::ToCommand,
};
use thiserror::Error;

use crate::error::*;
use crate::qemu::run_qemu;

mod error;
mod qemu;

const QEMU_BINARY_NAME: &str = "qemu-system-x86_64";
const OVMF_DIR: &str = "../target/ovmf";

fn main() {
    match run() {
        Ok(exit_status) => println!("Qemu exited with exit status {exit_status}"),
        Err(error) => println!("Error: {error}"),
    }
}

fn run() -> Result<ExitStatus> {
    println!("Downloading ovmf, this might take a long time.");

    let ovmf = ovmf_prebuilt::Prebuilt::fetch(ovmf_prebuilt::Source::LATEST, OVMF_DIR)?;

    let ovmf_code = ovmf.get_file(Arch::X64, FileType::Code);
    let ovmf_vars = ovmf.get_file(Arch::X64, FileType::Vars);

    let code_drive = Drive::builder()
        .interface(DriveInterface::Pflash)
        .format("raw".into())
        .unit(0)
        .file(ovmf_code.into())
        .read_only(OnOff::On)
        .build();
    let vars_drive = Drive::builder()
        .interface(DriveInterface::Pflash)
        .format("raw".into())
        .unit(1)
        .file(ovmf_vars.into())
        .read_only(OnOff::On)
        .build();

    run_qemu(
        QemuInstanceForX86_64::builder()
            .qemu_binary(QEMU_BINARY_NAME.into())
            .enable_kvm(true)
            .drive(vec![code_drive, vars_drive])
            .build(),
    )
}
