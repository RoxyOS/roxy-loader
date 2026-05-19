use std::{fs, path::Path};

use anyhow::Result;

fn build_c_api() -> Result<String> {
    let crate_dir = Path::new(env!("CARGO_MANIFEST_DIR"));

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

#[test]
fn generated_c_api_is_up_to_date() -> Result<()> {
    let expected = build_c_api()?;
    let actual = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("api crate has workspace root parent")
            .join("c_api/roxy_loader.h"),
    )?;

    assert_eq!(
        actual, expected,
        "c_api/roxy_loader.h is out of date; run `cargo run -p xtask c-api`"
    );

    Ok(())
}
