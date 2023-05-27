use ram::{cte::Context, print, println};
use rconfig::syscall::*;

pub fn do_syscall(context: &Context) {
    match context.regs[17] {
        SYSCALL_EXIT => {
            println!("SYSCALL_EXIT");
            println!("exit code: {}", context.regs[10]);
            loop {}
        }
        _ => {
            println!("unknown syscall");
            loop {}
        }
    }
}
