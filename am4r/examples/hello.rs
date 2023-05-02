#![no_std]
#![no_main]

use am4r::klib::{puts, get_time};
use am4r::*;

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    let mut last = get_time(); 
    puts("Hello, world!\n");
    let mut i = 0;
    loop {
        let new = get_time();
        if let Some(event) = io::KeyBoard::read() {
            println!("key event {:?}", event)
        }
        if (new - last) > 1000 {
            println!("count {}", i);
            i += 1;
            last = new;
        } 
    }
}
