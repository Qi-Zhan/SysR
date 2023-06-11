#![no_main]
#![no_std]

extern crate alloc;
use rapps::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let id = getpid();
    for i in 1..10 {
        println!("my pid is {}, i = {}", id, i);
    }
    exit(0);
}
