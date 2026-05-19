use cargo_emit::rustc_env;

fn main() {
    for (key, value) in std::env::vars() {
        if key.starts_with("CARGO_BIN_FILE_") {
            eprintln!("{key}={value}");
        }
    }

    eprintln!(
        "HOST={}",
        std::env::var("HOST").unwrap_or_else(|_| "<unset>".to_string())
    );
    eprintln!(
        "TARGET={}",
        std::env::var("TARGET").unwrap_or_else(|_| "<unset>".to_string())
    );

    let Some(roxyloader_artifact) = cargo_bin_file::bin_path("roxy-loader") else {
        panic!("Roxyloader artifact not found")
    };
    let roxyloader_artifact = roxyloader_artifact.to_str().unwrap();
    rustc_env!("ROXYLOADER_ARTIFACT", "{}", roxyloader_artifact);
}
