
use ram::{cte::Context, println, print};

pub const SYSCALL_EXIT: u32 = 93;
pub const SYSCALL_WRITE: u32 = 64;
pub const SYSCALL_READ: u32 = 63;
pub const SYSCALL_OPEN: u32 = 56;
pub const SYSCALL_CLOSE: u32 = 57;
pub const SYSCALL_FORK: u32 = 220;
pub const SYSCALL_EXEC: u32 = 221;
pub const SYSCALL_WAITPID: u32 = 260;
pub const SYSCALL_GETPID: u32 = 172;
pub const SYSCALL_SLEEP: u32 = 101;

pub fn do_syscall(context: &Context) {
    match context.regs[17] {
        SYSCALL_EXIT => {
            println!("SYSCALL_EXIT");
            println!("exit code: {}", context.regs[10]);
            loop {}
        },
        _ => {
            println!("unknown syscall");
            loop {}
        }
    }
}
