//! # RApps
//! provide syscall and stdio for user program
//! 

#![no_std]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
use core::{arch::asm, fmt::Write};
use rconfig::{std_io::*, syscall::*};

/// use macro to handle syscall so it can be used in arbitrary number of arguments
#[macro_export]
macro_rules! syscall {
    ($num: expr) => {
        {
            let ret: u32 = 0;
            #[cfg(target_arch="riscv32")]
            unsafe {
                asm!("ecall", in("a7") $num, out("a0") ret);
            }
            ret
        }
    };
    ($num: expr, $arg1: expr) => {
        {
            let mut ret: u32 = 0;
            #[cfg(target_arch="riscv32")]
            unsafe {
                asm!(
                    "mv a7, {x0}",
                    "mv a0, {x1}",
                    "ecall",
                    "mv {x2}, a0",
                    x0 = in(reg) $num,
                    x1 = in(reg) $arg1,
                    x2 = out(reg) ret,
                );
            }
            ret
        }
    };
    ($num: expr, $arg1: expr, $arg2: expr) => {
        {
            let mut ret: u32 = 0;
            #[cfg(target_arch="riscv32")]
            unsafe {
                asm!(
                    "mv a7, {x0}",
                    "mv a0, {x1}",
                    "mv a1, {x2}",
                    "ecall",
                    "mv {x3}, a0",
                    x0 = in(reg) $num,
                    x1 = in(reg) $arg1,
                    x2 = in(reg) $arg2,
                    x3 = out(reg) ret,
                );
            }
            ret
        }
    };
    ($num: expr, $arg1: expr, $arg2: expr, $arg3: expr) => {
        {
            let mut ret: u32 = 0;
            #[cfg(target_arch="riscv32")]
            unsafe {
                asm!(
                    "mv a7, {x0}",
                    "mv a0, {x1}",
                    "mv a1, {x2}",
                    "mv a2, {x3}",
                    "ecall",
                    "mv {x4}, a0",
                    x0 = in(reg) $num,
                    x1 = in(reg) $arg1,
                    x2 = in(reg) $arg2,
                    x3 = in(reg) $arg3,
                    x4 = out(reg) ret,
                );
            }
            ret
        }
    };
    ($num: expr, $arg1: expr, $arg2: expr, $arg3: expr, $arg4: expr) => {
        {
            let ret: u32 = 0;
            #[cfg(target_arch="riscv32")]
            asm!(
                "mv a7, {x0}",
                "mv a0, {x1}",
                "mv a1, {x2}",
                "mv a2, {x3}",
                "mv a3, {x4}",
                "ecall",
                "mv {x5}, a0",
                x0 = in(reg) $num,
                x1 = in(reg) $arg1,
                x2 = in(reg) $arg2,
                x3 = in(reg) $arg3,
                x4 = in(reg) $arg4,
                x5 = out(reg) ret,
            );
            ret
        }
    }
}

/* syscall wrappers */
pub fn write(fd: u32, buf: *const u8, len: usize) -> u32 {
    syscall!(SYSCALL_WRITE, fd, buf as u32, len as u32)
}

pub fn read(fd: u32, buf: *mut u8, len: usize) -> u32 {
    syscall!(SYSCALL_READ, fd, buf as u32, len as u32)
}

pub fn exit(code: u32) -> ! {
    syscall!(SYSCALL_EXIT, code);
    panic!("should not reach here, syscall exit failed");
}

/* stdlib */
pub fn puts(s: &str) {
    write(STDOUT, s.as_ptr(), s.len());
}


/* dummy stdout */
pub struct DummyStdout;

impl Write for DummyStdout {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        puts(s);
        Ok(())
    }
}
// // use write to implement print and println
#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! print {
    // implement the macro ourselves
    ($($arg:tt)*) => {
        match format_args!($($arg)*) {
            args => {
                use core::fmt::Write;
                let mut sp = $crate::DummyStdout;
                write!(sp, "{}", args).unwrap();
            }
        }
    };
}

#[panic_handler]
#[cfg(not(test))]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // use crate::{println, print};
    // print!("\x1b[1;31m");
    // println!("{}", _info);

    loop {}
}
