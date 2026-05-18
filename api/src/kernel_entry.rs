#[macro_export]
macro_rules! kernel_entry {
    ($kernel_main:ident) => {
        #[unsafe(no_mangle)]
        extern "sysv64" fn _start(bootinfo: *const $crate::BootInfo) -> ! {
            unsafe {
                let bootinfo = &*bootinfo;
                $kernel_main(bootinfo);
            }
        }
    };
}
