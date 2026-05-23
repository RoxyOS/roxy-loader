use std::{thread::sleep, time::Duration};

use anyhow::Result;
use xshell::{Shell, cmd};

use crate::test::test;

pub fn publish() -> Result<()> {
    test()?;

    run_publish("roxy-loader-api", &[])?;
    wait_for_publish();
    run_publish("roxy-loader", &["--no-verify"])?;
    wait_for_publish();
    run_publish("roxy-loader-utils", &[])?;

    Ok(())
}

fn run_publish(package: &str, extra_args: &[&str]) -> Result<()> {
    let sh = Shell::new()?;
    let command = match extra_args {
        [] => cmd!(sh, "cargo publish -p {package}"),
        [arg] => cmd!(sh, "cargo publish -p {package} {arg}"),
        _ => unreachable!("publish only uses zero or one extra arg"),
    };
    command.run()?;
    Ok(())
}

fn wait_for_publish() {
    println!("waiting 10 seconds...");
    sleep(Duration::from_secs(10));
}
