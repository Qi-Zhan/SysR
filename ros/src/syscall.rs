use ram::{cte::Context, print, println};
use rconfig::{syscall::*, std_io::*};

pub fn do_syscall(context: &Context) {
    match context.regs[SYSCALL_REG_NUM] {
        SYSCALL_EXIT => {
            println!("SYSCALL_EXIT");
            println!("exit code: {}", context.regs[10]);
            loop {}
        }
        SYSCALL_WRITE => {
            println!("SYSCALL_WRITE");
            let fd = context.regs[10];
            let buf = context.regs[11] as *const u8;
            let len = context.regs[12];
            let mut p = buf;
            unsafe {
                for _ in 0..len {
                    match fd {
                        STDOUT | STDERR => print!("{}", *p as char),
                        _ => todo!("only support stdout, which is fd=1, but got fd={}", fd)
                    }
                    p = p.offset(1);
                }
            }
            context.regs[10] = len;
        }
        _ => {
            println!("unknown syscall");
            loop {}
        }
    }
}
