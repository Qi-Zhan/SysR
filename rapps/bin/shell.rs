// a free-standing shell
#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    puts("Hello, world!\n");
    loop {}
}
