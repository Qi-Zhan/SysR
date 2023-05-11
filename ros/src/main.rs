#![no_std]
#![no_main]

use ram::*;
use ram::cte::{Event, Context};
use ram::klib::puts;

// cargo build --target riscv32i-unknown-none-elf 
// cargo build --target riscv32i-unknown-none-elf --release

mod syscall;
mod filesystem;

pub fn on_interrupt(event: Event, context: &Context) -> Context {
    match event {
        Event::Yield => {
            puts("yield\n");
            context.clone()
        },
        Event::Error => {
            puts("error\n");
            context.clone()
        },
        Event::Syscall => {
            puts("syscall\n");
            // syscall::do_syscall(context)
        },
        _ => {
            puts("unknown\n");
            context.clone()
        },
    }
    puts("on_interrupt\n");
}

#[no_mangle]
pub extern "C" fn _start(_argc: isize, _argv: *const *const u8) -> ! {
    cte::init(&on_interrupt);

    loop {
        cte::yield_();
        puts("Hello, world!\n");
        tm::halt(0);
    }
}
