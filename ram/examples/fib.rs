#![no_std]
#![no_main]

use ram::klib::get_time;
use ram::tm::halt;
use ram::*;

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
    let result = fib(30);
    let new = get_time();
    println!("fib(30) = {} in {} ms", result, new - last);
    halt(0);
    loop {}
}
