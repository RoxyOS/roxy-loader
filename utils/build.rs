use cargo_emit::rustc_env;

fn main() {
    let Some(roxyloader_artifact) = cargo_bin_file::bin_path("roxy-loader") else {
        panic!("Roxyloader artifact not found")
    };
    let roxyloader_artifact = roxyloader_artifact.to_str().unwrap();
    rustc_env!("ROXYLOADER_ARTIFACT", "{}", roxyloader_artifact);
}
