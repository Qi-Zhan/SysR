use ram::{cte::Context, print, println};
use rconfig::{syscall::*, std_io::*};



pub fn do_syscall(context: &mut Context) {
    match context.regs[SYSCALL_REG_NUM as usize] {
        SYSCALL_EXIT => {
            println!("SYSCALL_EXIT");
            println!("exit code: {}", context.regs[SYSCALL_REG_RET as usize]);
            loop {}
        }
        SYSCALL_WRITE => {
            let fd = context.regs[SYSCALL_REG_ARG0 as usize];
            let buf = context.regs[SYSCALL_REG_ARG1 as usize] as *const u8;
            let len = context.regs[SYSCALL_REG_ARG2 as usize];
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
            context.regs[SYSCALL_REG_RET as usize] = len;
        }
        _ => {
            println!("unknown syscall");
            loop {}
        }
    }
}
