#![no_std]
#![no_main]

use ram::klib::{get_time, getline, print_chars, puts};
use ram::*;

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    let mut last = get_time();
    puts("What's your name?\n");
    let mut buffer: [char; 100] = ['\0'; 100];
    let len = getline(&mut buffer);
    puts("Hello, ");
    print_chars(&buffer[..len]);
    puts("!\n");
    let mut i = 0;
    loop {
        let new = get_time();
        if (new - last) > 1000 {
            println!("count {}", i);
            i += 1;
            last = new;
        }
    }
}
