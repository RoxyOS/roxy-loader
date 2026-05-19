use std::{fs::metadata, path::PathBuf, process::ExitStatus};

use anyhow::Result;
use ovmf_prebuilt::{Arch, FileType};
use qemu_command_builder::{
    QemuInstanceForX86_64,
    args::drive::{Drive, DriveInterface},
    args::{device::Device, display::QemuDisplay, serial::SpecialDevice},
    common::OnOff,
};
use roxy_loader_utils::build_image::default_image_path;

use crate::utils::{cargo_target_dir, run_qemu};

const QEMU_BINARY_NAME: &str = "qemu-system-x86_64";
const TEST_EXIT_DEVICE_PORT: &str = "0xf4";
const TEST_EXIT_DEVICE_SIZE: &str = "0x04";
pub const TEST_SUCCESS_EXIT_STATUS: i32 = 33;
pub const TEST_FAILURE_EXIT_STATUS: i32 = 35;

pub type VMResult = Result<ExitStatus>;

pub fn run_vm() -> VMResult {
    let image = default_image_path()?;
    let (ovmf_code, ovmf_vars) = fetch_ovmf()?;
    let qemu_command = build_qemu_command(image, ovmf_code, ovmf_vars, kvm_available(), false);
    run_qemu(qemu_command)
}

pub fn run_test_vm() -> VMResult {
    let image = default_image_path()?;
    let (ovmf_code, ovmf_vars) = fetch_ovmf()?;
    let qemu_command = build_qemu_command(image, ovmf_code, ovmf_vars, kvm_available(), true);
    run_qemu(qemu_command)
}

fn build_qemu_command(
    image: PathBuf,
    ovmf_code: PathBuf,
    ovmf_vars: PathBuf,
    enable_kvm: bool,
    test_mode: bool,
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
        .enable_kvm(enable_kvm)
        .build();

    if test_mode {
        let mut exit_device = Device::new("isa-debug-exit");
        exit_device.add_prop("iobase", TEST_EXIT_DEVICE_PORT);
        exit_device.add_prop("iosize", TEST_EXIT_DEVICE_SIZE);

        qemu_command.device = Some(vec![exit_device]);
        qemu_command.serial = Some(SpecialDevice::Stdio);
        qemu_command.display = Some(QemuDisplay::None);
        qemu_command.monitor = Some(SpecialDevice::None);
        qemu_command.no_reboot = Some(true);
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
            false,
        );

        let argv = command.to_command();
        assert!(argv.iter().any(|arg| arg == "-enable-kvm"));
    }

    #[test]
    fn build_qemu_command_configures_test_mode() {
        let command = build_qemu_command(
            PathBuf::from("/tmp/image.img"),
            PathBuf::from("/tmp/OVMF_CODE.fd"),
            PathBuf::from("/tmp/OVMF_VARS.fd"),
            false,
            true,
        );

        let argv = command.to_command();
        let joined = argv.join(" ");

        assert!(joined.contains("isa-debug-exit"));
        assert!(joined.contains("iobase=0xf4"));
        assert!(joined.contains("-serial stdio"));
        assert!(joined.contains("-display none"));
        assert!(joined.contains("-monitor none"));
        assert!(!argv.iter().any(|arg| arg == "-nographic"));
        assert!(argv.iter().any(|arg| arg == "-no-reboot"));
    }
}
