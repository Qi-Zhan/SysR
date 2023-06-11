//! OS Syscall implementation
//! - exit not return
//! - write
//! - read
//! - open  not implemented

use alloc::boxed::Box;
use ram::{cte::Context, io::IO, print, println};
use rconfig::{std_io::*, syscall::*, layout::USER_APP_SIZE};

use crate::task::{TaskState, TM};

pub fn do_syscall(context: &mut Context) {
    match context.regs[SYSCALL_REG_NUM as usize] {
        SYSCALL_EXIT => unsafe {
            let id = TM.as_ref().unwrap().current;
            TM.as_mut().unwrap().tasks[id].state = TaskState::Exit;
            TM.as_mut().unwrap().schedule(context);
            // println!("new context 0x{:x} {}", context.mepc, context.mstatus);
        },
        SYSCALL_WRITE => {
            let fd = context.regs[SYSCALL_REG_ARG0 as usize];
            let buf = context.regs[SYSCALL_REG_ARG1 as usize] as *const u8;
            let len = context.regs[SYSCALL_REG_ARG2 as usize];
            let id = unsafe { TM.as_ref().unwrap().current };
            let mut p = unsafe { buf.offset(id as isize * USER_APP_SIZE as isize) };
            unsafe {
                match fd {
                    STDOUT | STDERR => {
                        for _ in 0..len {
                            print!("{}", *p as char);
                            p = p.offset(1);
                        }
                    }
                    _ => todo!(
                        "only support stdout/stderr, which is fd=1/2, but got fd={}",
                        fd
                    ),
                }
            }
            context.regs[SYSCALL_REG_RET as usize] = len;
            context.mepc += 4;
        }
        SYSCALL_READ => {
            let fd = context.regs[SYSCALL_REG_ARG0 as usize];
            let buf = context.regs[SYSCALL_REG_ARG1 as usize] as *mut u8;
            let len = context.regs[SYSCALL_REG_ARG2 as usize];
            // let mut p = buf;
            let mut p = unsafe { buf.offset(TM.as_ref().unwrap().current as isize * USER_APP_SIZE as isize) };
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
            context.mepc += 4;
        }
        SYSCALL_SBARK => {
            let _size = context.regs[SYSCALL_REG_ARG0 as usize];
            let addr = Box::into_raw(Box::new([0u8; 4096])) as u32;
            context.regs[SYSCALL_REG_RET as usize] = addr;
            context.mepc += 4;
        }
        SYSCALL_GETPID => {
            unsafe {
                context.regs[SYSCALL_REG_RET as usize] = TM.as_ref().unwrap().current as u32;
            }
            context.mepc += 4;
        }
        _ => {
            println!("unknown syscall");
            loop {}
        }
    }
}
