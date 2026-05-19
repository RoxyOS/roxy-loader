use cargo_emit::rustc_link_arg;

fn main() {
    rustc_link_arg!("-no-pie");
}
