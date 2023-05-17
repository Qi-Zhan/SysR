#![allow(unused_imports)]
use core::{arch::asm, fmt::Write};
// ----------------------- TM: Turing Machine -----------------------
pub fn putch(c: char) {
    let mut serial = crate::io::SerialPort;
    serial.write_char(c).unwrap();
}

pub fn halt(code: i8) {
    #[cfg(target_arch="riscv32")]
    unsafe {
        asm!(
            "mv a0, {x0}",
            "ebreak",
            x0 = in(reg) code, 
        );
    }
    // TODO: implement for other architectures
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
