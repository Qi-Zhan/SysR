//! OS Syscall implementation
//! - exit not return
//! - write
//! - read  not implemented
//! - open  not implemented

use ram::{cte::Context, print, println, io::IO};
use rconfig::{std_io::*, syscall::*};

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
                match fd {
                    STDOUT | STDERR => {
                        for _ in 0..len {
                            print!("{}", *p as char);
                            p = p.offset(1);
                        }
                    }
                    _ => todo!("only support stdout/stderr, which is fd=1/2, but got fd={}", fd),
                }
            }
            context.regs[SYSCALL_REG_RET as usize] = len;
        }
        SYSCALL_READ => {
            let fd = context.regs[SYSCALL_REG_ARG0 as usize];
            let buf = context.regs[SYSCALL_REG_ARG1 as usize] as *mut u8;
            let len = context.regs[SYSCALL_REG_ARG2 as usize];
            let mut p = buf;
            unsafe {
                match fd {
                    STDIN => {
                        // println!("buf = {:p}", buf);
                        for _ in 0..len {
                            *p = crate::io::SerialPort::read() as u8;
                            p = p.offset(1);
                        }
                    }
                    _ => todo!("only support stdin, which is fd=0, but got fd={}", fd),
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
