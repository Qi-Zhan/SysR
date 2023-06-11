#![no_main]
#![no_std]

extern crate alloc;
use rapps::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let id = getpid();
    for i in (0..10).rev() {
        println!("[user] my pid is {}, i = {}", id, i);
        yield_();
    }
    exit(0);
}
