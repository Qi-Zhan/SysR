// a free-standing shell, which is the first user app

#![no_main]
#![no_std]

use rapps::*;
use rconfig::std_io::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = "Hello, world!\n";
    let success = write(STDOUT, hello.as_ptr(), hello.len());
    println!("write {} bytes", success);
    exit(0);
}
