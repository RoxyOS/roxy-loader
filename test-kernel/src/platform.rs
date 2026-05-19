use core::fmt;

use os_test_framework::{ExitState, Platform};
use qemu_exit::{QEMUExit, X86};

use crate::serial::serial_print;

const QEMU_EXIT_SUCCESS_STATUS: u32 = 33;
const QEMU_EXIT_FAILURE_CODE: u32 = 17;
const QEMU_EXIT_PORT: u16 = 0xf4;

pub struct TestPlatform;

impl Platform for TestPlatform {
    fn print(&mut self, args: fmt::Arguments) {
        serial_print(args);
    }

    fn exit(&self, state: ExitState) -> ! {
        match state {
            ExitState::Success => qemu_exit_handle().exit_success(),
            ExitState::Failed => qemu_exit_handle().exit(QEMU_EXIT_FAILURE_CODE),
        }
    }
}

fn qemu_exit_handle() -> qemu_exit::X86 {
    unsafe { X86::new(QEMU_EXIT_PORT, QEMU_EXIT_SUCCESS_STATUS) }
}
