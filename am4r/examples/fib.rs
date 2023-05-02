#![no_std]
#![no_main]

use am4r::klib::get_time;
use am4r::*;
use am4r::tm::halt;

fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    let last = get_time(); 
    let result = fib(20);
    let new = get_time();
    println!("fib(20) = {} in {} ms", result, new - last);
    halt(0);
    loop {}
}
