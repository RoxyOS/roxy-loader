use roxy_loader_api::bootinfo::BootInfo;
use spin::Once;

static BOOTINFO: Once<BootInfo> = Once::new();

pub fn init(bootinfo: &BootInfo) {
    BOOTINFO.call_once(|| BootInfo {
        framebuffer: bootinfo.framebuffer,
    });
}

pub fn bootinfo() -> &'static BootInfo {
    BOOTINFO.get().expect("bootinfo not initialized")
}
