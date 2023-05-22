#![no_std]
#![no_main]

use ram::*;
use ram::cte::{Event, Context};
use ram::klib::puts;
use ram::tm::halt;

mod syscall;
mod filesystem;

#[no_mangle]
pub fn _on_interrupt(event: Event, context: &mut Context)  {
    match event {
        Event::Yield => {
            context.mepc += 4;
            puts("yield\n");
        },
        Event::Error => {
            puts("error\n");
        },
        Event::Syscall => {
            puts("syscall\n");
            syscall::do_syscall(&context);
            context.mepc += 4;
        },
        _ => {
            puts("unknown\n");
            halt(11);
        },
    }
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {

    cte::init(_on_interrupt);
    let mut count = 0;
    puts("Hello, world!\n");
    loop {
        cte::yield_();
        count += 1;
        println!("count {count}");
    }
}
