//! Helpers for producing bootable disk images for `roxy-loader`.
//!
//! The functions in this module are intended for host-side build tools that
//! need to prepare a disk image containing the loader and a kernel binary.

use std::{
    fs::File,
    io::{self, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use anyhow::Result;
use cargo_artifact_dependency::ArtifactDependencyBuilder;
use fatfs::{FileSystem, FormatVolumeOptions, FsOptions};

use crate::utils::{cargo_target_dir, static_workspace_root};

const ROXY_LOADER_ARTIFACT_VERSION: &str = "0.1.5";

/// Builds a bootable disk image for a kernel artifact.
pub fn build_image(kernel_binary: PathBuf) -> Result<PathBuf> {
    let image_path = default_image_path()?;
    build_image_from_paths(&image_path, &roxyloader_artifact()?, &kernel_binary)?;
    Ok(image_path)
}

/// Builds a bootable disk image from explicit paths.
///
/// Use this when you want full control over the image path, loader path, or
/// kernel path.
///
/// # Examples
///
/// ```
/// use roxy_loader_utils::build_image::build_image_from_paths;
///
/// let temp = std::env::temp_dir().join(format!(
///     "roxy-loader-doc-{}",
///     std::process::id()
/// ));
/// std::fs::create_dir_all(&temp)?;
///
/// let image = temp.join("image.img");
/// let loader = temp.join("loader.efi");
/// let kernel = temp.join("kernel.bin");
///
/// std::fs::write(&loader, b"loader")?;
/// std::fs::write(&kernel, b"kernel")?;
///
/// build_image_from_paths(&image, &loader, &kernel)?;
///
/// assert!(image.exists());
///
/// std::fs::remove_dir_all(&temp)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn build_image_from_paths(
    image_path: &Path,
    roxyloader_artifact: &Path,
    kernel_binary: &Path,
) -> Result<()> {
    const IMAGE_SIZE: u64 = 64 * 1024 * 1024;

    let mut image = open_image(image_path)?;

    // truncate
    image.set_len(IMAGE_SIZE)?;

    fatfs::format_volume(&mut image, FormatVolumeOptions::new())?;

    image.seek(SeekFrom::Start(0))?;

    let fs = FileSystem::new(image, FsOptions::new())?;

    let root_dir = fs.root_dir();
    root_dir.create_dir("EFI")?;
    let efi_dir = root_dir.open_dir("EFI")?;
    efi_dir.create_dir("BOOT")?;
    let boot_dir = efi_dir.open_dir("BOOT")?;

    // Copies roxyloader artifact
    let mut src = File::open(roxyloader_artifact)?;
    let mut dst = boot_dir.create_file("BOOTX64.EFI")?;
    io::copy(&mut src, &mut dst)?;

    // Installs kernel binary
    let mut dst = fs.root_dir().create_file("KERNEL")?;
    let mut src = File::open(kernel_binary)?;
    io::copy(&mut src, &mut dst)?;

    Ok(())
}

/// Returns the default output path used by [`build_image`].
pub fn default_image_path() -> Result<PathBuf> {
    const IMAGE_NAME: &str = "image.img";
    Ok(cargo_target_dir()?.join(IMAGE_NAME))
}

fn open_image(path: &Path) -> Result<File> {
    Ok(File::options()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)?)
}

fn roxyloader_artifact() -> Result<PathBuf> {
    Ok(ArtifactDependencyBuilder::default()
        .crate_name("roxy-loader")
        // Use staticly evaluated workspace root to prevent it from getting the
        // crate user's workspace root
        .path(static_workspace_root())
        .version(ROXY_LOADER_ARTIFACT_VERSION)
        .target("x86_64-unknown-uefi")
        .build()?
        .resolve()?)
}

#[cfg(test)]
mod tests {
    use workspace_root::get_workspace_root;

    use super::*;
    use std::{
        io::Read,
        sync::atomic::{AtomicU64, Ordering},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn test_temp_dir() -> Result<std::path::PathBuf> {
        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let unique = COUNTER.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!(
            "roxy-loader-test-{}-{}",
            std::process::id(),
            SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos() + unique as u128
        ));

        std::fs::create_dir_all(&dir)?;
        Ok(dir)
    }

    #[test]
    fn roxyloader_artifact_version_matches_package_version() -> Result<()> {
        let metadata = cargo_metadata::MetadataCommand::new()
            .manifest_path(get_workspace_root().join("Cargo.toml"))
            .exec()?;
        let package = metadata
            .packages
            .iter()
            .find(|package| package.name == "roxy-loader")
            .expect("workspace should contain the roxy-loader package");

        assert_eq!(
            ROXY_LOADER_ARTIFACT_VERSION,
            package.version.to_string(),
            "update ROXY_LOADER_ARTIFACT_VERSION when bumping roxy-loader"
        );

        Ok(())
    }

    #[test]
    fn roxyloader_artifact_resolves_to_valid_file() -> Result<()> {
        let artifact_path = roxyloader_artifact()?;
        let metadata = std::fs::metadata(&artifact_path)?;

        assert!(
            metadata.is_file(),
            "roxy-loader artifact path should be a file: {}",
            artifact_path.display()
        );
        assert!(
            metadata.len() > 0,
            "roxy-loader artifact should not be empty: {}",
            artifact_path.display()
        );

        Ok(())
    }

    #[test]
    fn build_image_contains_loader_and_kernel_payloads() -> Result<()> {
        let dir = test_temp_dir()?;
        let image_path = dir.join("image.img");
        let loader_path = dir.join("loader.efi");
        let kernel_path = dir.join("kernel.bin");

        std::fs::write(&loader_path, b"loader-bytes")?;
        std::fs::write(&kernel_path, b"kernel-bytes")?;

        build_image_from_paths(&image_path, &loader_path, &kernel_path)?;

        let image = File::options().read(true).write(true).open(&image_path)?;
        let fs = FileSystem::new(image, FsOptions::new())?;

        let root = fs.root_dir();
        let efi_dir = root.open_dir("EFI")?;
        let boot_dir = efi_dir.open_dir("BOOT")?;

        let mut loader_file = boot_dir.open_file("BOOTX64.EFI")?;
        let mut loader_bytes = Vec::new();
        loader_file.read_to_end(&mut loader_bytes)?;
        assert_eq!(loader_bytes, b"loader-bytes");

        let mut kernel_file = root.open_file("KERNEL")?;
        let mut kernel_bytes = Vec::new();
        kernel_file.read_to_end(&mut kernel_bytes)?;
        assert_eq!(kernel_bytes, b"kernel-bytes");

        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }

    #[test]
    fn build_image_truncates_existing_image_contents() -> Result<()> {
        let dir = test_temp_dir()?;
        let image_path = dir.join("image.img");
        let loader_path = dir.join("loader.efi");
        let kernel_path = dir.join("kernel.bin");

        std::fs::write(&loader_path, b"a")?;
        std::fs::write(&kernel_path, b"b")?;
        std::fs::write(&image_path, b"stale-bytes")?;

        build_image_from_paths(&image_path, &loader_path, &kernel_path)?;

        let metadata = std::fs::metadata(&image_path)?;
        assert_eq!(metadata.len(), 64 * 1024 * 1024);

        std::fs::remove_dir_all(&dir)?;
        Ok(())
    }
}
