use std::{
    env::var,
    fmt::Display,
    io::{self, ErrorKind, Stdout},
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
};

use image_builder::build_bootloader_image;
use ovmf_prebuilt::{Arch, FileType};
use qemu_command_builder::{
    QemuInstanceBase, QemuInstanceForX86_64,
    args::drive::{Drive, DriveBuilder, DriveInterface},
    common::OnOff,
    to_command::ToCommand,
};
use thiserror::Error;
use workspace_root::get_workspace_root;

use crate::error::*;
use crate::qemu::run_qemu;

mod error;
mod qemu;

const QEMU_BINARY_NAME: &str = "qemu-system-x86_64";
const OVMF_DIR: &str = "../target/ovmf";
const BOOTLOADER_EFI_FILE_PATH: &str = "target/x86_64-unknown-uefi/debug/roxy-loader.efi";
const BOOTLOADER_IMAGE_PATH: &str = "target/image.img";

fn main() {
    match run() {
        Ok(exit_status) => println!("Qemu exited with exit status {exit_status}"),
        Err(error) => println!("Error: {error}"),
    }
}

fn run() -> Result<ExitStatus> {
    let bootloader_efi_file = get_workspace_root().join(BOOTLOADER_EFI_FILE_PATH);
    let bootloader_image_path = get_workspace_root().join(BOOTLOADER_IMAGE_PATH);

    build_bootloader_image(bootloader_efi_file, bootloader_image_path.clone()).unwrap();

    println!("Downloading ovmf, this might take a long time.");
    let (ovmf_code, ovmf_vars) = fetch_ovmf()?;

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
    let bootloader_drive = Drive::builder()
        .format("raw".into())
        .file(bootloader_image_path.into())
        .build();

    let drives = vec![code_drive, vars_drive, bootloader_drive];

    let qemu_command = QemuInstanceForX86_64::builder()
        .qemu_binary(QEMU_BINARY_NAME.into())
        .enable_kvm(true)
        .drive(drives)
        .build();

    run_qemu(qemu_command)
}

fn fetch_ovmf() -> Result<(PathBuf, PathBuf)> {
    let ovmf = ovmf_prebuilt::Prebuilt::fetch(ovmf_prebuilt::Source::LATEST, OVMF_DIR)?;

    let ovmf_code = ovmf.get_file(Arch::X64, FileType::Code);
    let ovmf_vars = ovmf.get_file(Arch::X64, FileType::Vars);

    Ok((ovmf_code, ovmf_vars))
}
