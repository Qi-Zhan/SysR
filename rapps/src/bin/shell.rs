// a free-standing shell, which is the first user app

#![no_main]
#![no_std]

use rapps::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = "Hello, world!\n";
    let mut buffer: [u8; 100] = [0; 100];
    loop {
        print!("> ");
        getline(&mut buffer);
        println!("You said: {}", core::str::from_utf8(&buffer).unwrap());
    }
    exit(0);
}
