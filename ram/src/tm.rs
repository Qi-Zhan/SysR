//! ----------------------- TM: Turing Machine -----------------------
//! - halt(code: i8) -> !

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::empty_loop)]
use core::{arch::asm, fmt::Write};

pub fn halt(code: i8) -> ! {
    #[cfg(target_arch = "riscv32")]
    unsafe {
        asm!(
            "mv a0, {x0}",
            "ebreak",
            x0 = in(reg) code,
        );
    }
    loop {}
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    use crate::{print, println};
    print!("\x1b[1;31m");
    println!("{}", _info);
    loop {}
}
