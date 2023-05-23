#![no_std]
#![no_main]

use ram::cte::{Context, Event};
use ram::klib::puts;
use ram::tm::halt;
use ram::*;

mod filesystem;
mod loader;
mod syscall;
mod utils;

#[no_mangle]
pub fn on_interrupt(event: Event, context: &mut Context) {
    match event {
        Event::Yield => {
            context.mepc += 4;
            puts("yield\n");
        }
        Event::Error => {
            puts("error\n");
        }
        Event::Syscall => {
            puts("syscall\n");
            syscall::do_syscall(&context);
            context.mepc += 4;
        }
        _ => {
            puts("unknown\n");
            halt(11);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    cte::init(on_interrupt);
    // puts(utils::LOGO);
    let mut count = 0;
    // TODO: kernel level shell
    loop {
        puts("ROS> ");
        cte::yield_();
        count += 1;
        println!("count {count}");
    }
}
