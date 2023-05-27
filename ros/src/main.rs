#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec;

use ram::cte::{Context, Event};
use ram::klib::puts;
use ram::tm::halt;
use ram::*;
use rconfig::layout::USER_APP_BASE;

mod allocater;
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

// shell, cat, ls, etc.

const APPS: [&str; 4] = ["shell", "cat", "ls", "echo"];

macro_rules! copy_app {
    ($app: literal, $base: expr) => {
        let app = include_bytes!(concat!(
            "../../target/riscv32i-unknown-none-elf/release/",
            $app
        ));
        let app_len = app.len();
        let mut p = $base as *mut u8;
        for i in 0..app_len {
            *p = app[i];
            p = p.offset(1);
        }
    };
}

unsafe fn load_apps() {
    // TDOO: set the function and logic to build.rs
    copy_app!("shell", USER_APP_BASE);
    // copy_app!("cat", USER_APP_BASE + USER_APP_SIZE);
    // copy_app!("ls", USER_APP_BASE + USER_APP_SIZE * 2);
    // copy_app!("echo", USER_APP_BASE + USER_APP_SIZE * 3);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        load_apps();
    }

    cte::init(on_interrupt);
    let mut vec = vec![1, 2, 3];
    for i in vec.iter() {
        println!("{}", i);
    }
    for i in 0..10000 {
        vec.push(i);
    }
    halt(0);
    let mut count = 0;
    loop {
        cte::yield_();
        count += 1;
        println!("count {count}");
    }
}
