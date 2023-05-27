#![no_std]
use core::arch::asm;

pub fn syscall(cause: u32, arg1: Option<u32>, arg2: Option<u32>) -> u32 {
    todo!("implement user syscall");
    #[cfg(target_arch = "riscv32")]
    unsafe {
        let mut ret: u32 = 0;
        let a7: u32 = cause;
        asm!(
            "mv a7, {x0}",
            "ecall",
            "mv {x0}, a0",
            x0 = out(reg) ret,
            // x1 = in(reg) a7,
        );
        return ret;
    }
    //  if malloc or free, use global allocator, global allocator use syscall
    0
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // use crate::{println, print};
    // print!("\x1b[1;31m");
    // println!("{}", _info);

    loop {}
}
