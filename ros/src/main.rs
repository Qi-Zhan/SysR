#![no_std]
#![no_main]

extern crate alloc;
use core::arch::asm;
use loader::load_file;
use ram::cte::{Context, Event};
use ram::{klib::puts, tm::halt, *};

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
        }
        Event::Error => {
            puts("error\n");
        }
        Event::Syscall => {
            syscall::do_syscall(context);
            context.mepc += 4;
        }
        _ => {
            puts("unknown\n");
            halt(1);
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    cte::init(on_interrupt);
    let fs = filesystem::FileSystem::new();
    let entry = load_file(&fs.files[0]);
    // jump to entry
    unsafe {
        asm!(
            "jr {0}",
            in(reg) entry,
        )
    }
    halt(0);
    let mut count = 0;
    loop {
        cte::yield_();
        count += 1;
        println!("count {count}");
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    #[test]
    fn alloc_test() {
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.len(), 3);
        for i in 0..10000 {
            vec.push(i);
            assert_eq!(vec.len(), i + 4)
        }
    }
}
