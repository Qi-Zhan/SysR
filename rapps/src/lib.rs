use std::arch::asm;

pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_READ: usize = 63;
pub const SYSCALL_OPEN: usize = 56;
pub const SYSCALL_CLOSE: usize = 57;
pub const SYSCALL_FORK: usize = 220;
pub const SYSCALL_EXEC: usize = 221;
pub const SYSCALL_WAITPID: usize = 260;
pub const SYSCALL_GETPID: usize = 172;
pub const SYSCALL_SLEEP: usize = 101;

pub fn syscall(_cause: u32, arg1: Option<u32>, arg2: Option<u32>) -> u32 {
    #[cfg(target_arch="riscv32")]
    unsafe {
        let mut ret: u32 = 0;
        let a7: u32 = cause;
        asm!(
            "mv a7, {x0}",
            "ecall",
            "mv {x0}, a0",
            x0 = out(reg) ret,
            x1 = in(reg) a7,
        );
        return ret;
    }
    0
}
