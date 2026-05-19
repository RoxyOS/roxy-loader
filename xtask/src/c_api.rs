use std::path::Path;

use anyhow::Result;

pub fn build_c_api() -> Result<String> {
    let crate_dir = Path::new("api");

    let c_api = cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_config(
            cbindgen::Config::from_file(crate_dir.join("cbindgen.toml"))
                .map_err(anyhow::Error::msg)?,
        )
        .generate()?;

    let mut bytes = Vec::new();
    c_api.write(&mut bytes);
    Ok(String::from_utf8(bytes)?)
}

pub fn generate_c_api() -> Result<()> {
    let output = Path::new("c_api/roxy_loader.h");

    std::fs::create_dir_all(output.parent().expect("c_api output has parent"))?;
    std::fs::write(output, build_c_api()?)?;

    Ok(())
}
