use std::path::PathBuf;

use anyhow::Result;
use workspace_root::get_workspace_root;

use crate::run_command;
use crate::test::test;

pub fn bump_version(new_version: &str) -> Result<()> {
    let workspace_root = get_workspace_root();

    update_text_file(
        workspace_root.join("Cargo.toml"),
        &[("version = \"", "\"")],
        new_version,
    )?;
    update_text_file(
        workspace_root.join("api/Cargo.toml"),
        &[("version = \"", "\"")],
        new_version,
    )?;
    update_text_file(
        workspace_root.join("utils/Cargo.toml"),
        &[("version = \"", "\"")],
        new_version,
    )?;
    update_text_file(
        workspace_root.join("utils/src/build_image.rs"),
        &[("const ROXY_LOADER_ARTIFACT_VERSION: &str = \"", "\";")],
        new_version,
    )?;

    // Check if bumping the version caused any issues
    run_command!("cargo metadata --format-version=1");
    test()?;
    run_command!("cargo test -p roxy-loader-utils --features local-dev");

    Ok(())
}

fn update_text_file(path: PathBuf, patterns: &[(&str, &str)], version: &str) -> Result<()> {
    let mut contents = std::fs::read_to_string(&path)?;

    for (start, end) in patterns {
        contents = replace_once(&contents, start, end, version)?;
    }

    std::fs::write(path, contents)?;
    Ok(())
}

fn replace_once(contents: &str, start: &str, end: &str, version: &str) -> Result<String> {
    let start_index = contents
        .find(start)
        .ok_or_else(|| anyhow::anyhow!("pattern not found: {start:?}"))?
        + start.len();
    let end_index = contents[start_index..]
        .find(end)
        .map(|offset| start_index + offset)
        .ok_or_else(|| anyhow::anyhow!("end pattern not found after {start:?}: {end:?}"))?;

    let mut out = String::with_capacity(contents.len() - (end_index - start_index) + version.len());
    out.push_str(&contents[..start_index]);
    out.push_str(version);
    out.push_str(&contents[end_index..]);
    Ok(out)
}
