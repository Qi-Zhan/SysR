//! a very simple shell, which is the first user app

#![no_main]
#![no_std]

extern crate alloc;
use rapps::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let id = getpid();
    println!("Hello, I'm a simple shell, my pid is {}", id);
    let mut buffer: [u8; 100] = [0; 100];
    loop {
        print!("> ");
        getline(&mut buffer);
        let buffer = unsafe { core::str::from_utf8_unchecked(&buffer) };
        if buffer.starts_with("echo") {
            println!("{}", &buffer[5..]);
        } else if buffer.starts_with("exit") {
            println!("Bye!");
            break;
        } else {
            println!("unknown command");
            println!("try: echo, exit");
        }
    }
    exit(0);
}
