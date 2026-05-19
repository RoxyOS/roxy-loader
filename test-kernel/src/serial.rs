use core::fmt::{self, Arguments, Write};

use spin::{Lazy, Mutex};
use uart_16550::{Config, Uart16550, Uart16550Tty, backend::PioBackend};

const SERIAL_PORT: u16 = 0x3f8;

static SERIAL: Lazy<Mutex<Uart16550Tty<PioBackend>>> = Lazy::new(|| unsafe {
    Uart16550Tty::new_port(SERIAL_PORT, Config::default())
        .expect("serial port should be valid")
        .into()
});

pub fn serial_print(args: Arguments) {
    let _ = SERIAL.lock().write_fmt(args);
}
