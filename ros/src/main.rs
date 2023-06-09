#![no_std]
#![no_main]

use crate::task::TaskManager;
use loader::load_file;
use ram::cte::{Context, Event};
use ram::{tm::halt, *};
use task::{Task, TaskState, TM};
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
            unsafe {
                let cur = TM.as_mut().unwrap().current;
                // save current context
                context.assign_to(&mut TM.as_mut().unwrap().tasks[cur].context);
                TM.as_mut().unwrap().tasks[cur].state = TaskState::Ready;
                TM.as_mut().unwrap().schedule(context)
            };
        }
        Event::Error => {
            println!("error");
        }
        Event::Syscall => {
            syscall::do_syscall(context);
        }
        _ => {
            println!("Unknown Event {:?}", event);
            halt(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    cte::init(on_interrupt);
    let fs = filesystem::FileSystem::new();
    unsafe {
        TM = Some(TaskManager::new());
        for i in 0..16 {
            if fs.files[i].name == "" {
                break;
            }
            let entry = load_file(&fs.files[i], i * 0x500000);
            let name = fs.files[i].name;
            TM.as_mut().unwrap().add(Task::new(name, entry, i));
        }
        TM.as_mut().unwrap().run();
    }
    panic!("should not reach here")
}
