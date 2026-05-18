use std::{fs::metadata, path::PathBuf, process::ExitStatus};

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
    let qemu_command = build_qemu_command(image, ovmf_code, ovmf_vars, kvm_available());
    run_qemu(qemu_command)
}

fn build_qemu_command(
    image: PathBuf,
    ovmf_code: PathBuf,
    ovmf_vars: PathBuf,
    enable_kvm: bool,
) -> QemuInstanceForX86_64 {
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

    if enable_kvm {
        qemu_command.enable_kvm = Some(true);
    }

    qemu_command
}

fn kvm_available() -> bool {
    metadata("/dev/kvm").is_ok()
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

#[cfg(test)]
mod tests {
    use super::*;
    use qemu_command_builder::to_command::ToCommand;

    #[test]
    fn build_qemu_command_includes_expected_drives() {
        let command = build_qemu_command(
            PathBuf::from("/tmp/image.img"),
            PathBuf::from("/tmp/OVMF_CODE.fd"),
            PathBuf::from("/tmp/OVMF_VARS.fd"),
            false,
        );

        let argv = command.to_command();
        let joined = argv.join(" ");

        assert_eq!(argv.first().map(String::as_str), Some(QEMU_BINARY_NAME));
        assert!(joined.contains("/tmp/OVMF_CODE.fd"));
        assert!(joined.contains("/tmp/OVMF_VARS.fd"));
        assert!(joined.contains("/tmp/image.img"));
        assert!(joined.contains("if=pflash"));
        assert!(!joined.contains("-enable-kvm"));
    }

    #[test]
    fn build_qemu_command_enables_kvm_when_requested() {
        let command = build_qemu_command(
            PathBuf::from("/tmp/image.img"),
            PathBuf::from("/tmp/OVMF_CODE.fd"),
            PathBuf::from("/tmp/OVMF_VARS.fd"),
            true,
        );

        let argv = command.to_command();
        assert!(argv.iter().any(|arg| arg == "-enable-kvm"));
    }
}
