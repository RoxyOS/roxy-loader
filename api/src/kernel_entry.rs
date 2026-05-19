/// Defines the entry point used by kernels started with `roxy-loader`.
///
/// Use this macro in the crate that contains your kernel entry function.
/// It connects your function to the startup convention expected by
/// `roxy-loader`.
///
/// # Examples
///
/// ```ignore
/// #![no_std]
/// #![no_main]
///
/// use roxy_loader_api::{bootinfo::BootInfo, kernel_entry};
///
/// kernel_entry!(kernel_main);
///
/// fn kernel_main(bootinfo: &BootInfo) -> ! {
///     let _framebuffer = bootinfo.framebuffer;
///
///     loop {}
/// }
/// ```
#[macro_export]
macro_rules! kernel_entry {
    ($kernel_main:ident) => {
        #[unsafe(no_mangle)]
        extern "sysv64" fn _start(bootinfo: *const $crate::bootinfo::BootInfo) -> ! {
            unsafe {
                let bootinfo = &*bootinfo;
                $kernel_main(bootinfo);
            }
        }
    };
}
