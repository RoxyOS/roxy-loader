#[macro_export]
macro_rules! run_command {
    ($cmd:literal) => {
        let sh = $crate::private::xshell::Shell::new()?;
        $crate::private::xshell::cmd!(sh, $cmd).run()?;
    };
}
