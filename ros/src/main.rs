#![no_std]
#![no_main]

use crate::task::TaskManager;
use loader::load_file;
use ram::cte::{Context, Event};
use ram::{tm::halt, *};
use task::{Task, TM};
extern crate alloc;

mod allocator;
mod filesystem;
mod loader;
mod syscall;
mod task;

#[no_mangle]
pub fn on_interrupt(event: Event, context: &mut Context) {
    match event {
        Event::Yield => {
            context.mepc += 4;
        }
        Event::Error => {
            println!("error");
        }
        Event::Syscall => {
            syscall::do_syscall(context);
        }
        _ => {
            println!("Unknown Event");
            halt(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    cte::init(on_interrupt);
    let fs = filesystem::FileSystem::new();
    let entry = load_file(&fs.files[0], 0);
    unsafe {
        TM = Some(TaskManager::new());
        TM.as_mut().unwrap().add(Task::new("shell", entry));
        TM.as_mut().unwrap().run();
    }
    // should not reach here
    halt(1);
}
