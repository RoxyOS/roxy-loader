use std::{path::PathBuf, process::ExitStatus};

use anyhow::Result;
use ovmf_prebuilt::{Arch, FileType};
use qemu_command_builder::{
    QemuInstanceForX86_64,
    args::drive::{Drive, DriveInterface},
    common::OnOff,
};
use workspace_root::get_workspace_root;

use crate::utils::{cargo_target_dir, run_qemu};

const QEMU_BINARY_NAME: &str = "qemu-system-x86_64";

pub type VMResult = Result<ExitStatus>;

pub fn run_vm(image: PathBuf) -> VMResult {
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

    let image_drive = Drive::builder()
        .format("raw".into())
        .file(image.into())
        .build();

    let drives = vec![code_drive, vars_drive, image_drive];

    let mut qemu_command = QemuInstanceForX86_64::builder()
        .qemu_binary(QEMU_BINARY_NAME.into())
        .drive(drives)
        .build();

    if std::fs::metadata("/dev/kvm").is_ok() {
        qemu_command.enable_kvm = Some(true);
    }

    run_qemu(qemu_command)
}

const OVMF_DIR: &str = "ovmf";

fn fetch_ovmf() -> Result<(PathBuf, PathBuf)> {
    println!("Downloading ovmf, this might take a long time.");

    let ovmf_dir = cargo_target_dir()?.join(OVMF_DIR);

    let ovmf = ovmf_prebuilt::Prebuilt::fetch(ovmf_prebuilt::Source::LATEST, ovmf_dir)?;

    let ovmf_code = ovmf.get_file(Arch::X64, FileType::Code);
    let ovmf_vars = ovmf.get_file(Arch::X64, FileType::Vars);

    Ok((ovmf_code, ovmf_vars))
}
