#![no_std]
#![no_main]

use am4r::*;
use am4r::klib::puts;

// cargo build --target riscv32i-unknown-none-elf 
// cargo build --target riscv32i-unknown-none-elf --release

mod syscall;
mod filesystem;

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    loop {
        puts("Hello, world!\n");
        tm::halt(0);
    }
}
