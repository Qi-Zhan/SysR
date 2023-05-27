#![no_std]
/* This file show the all SETTING that all project used, it includes:
 * Interface of REMU and ROS, such as IO port
 * Interface of ROS and RAPPS, such as system call number
 * Layout of OS kernel, such as heap, start address of kernel
*/

/* Interface of REMU and ROS */
pub mod ios {
    pub const DEVICE_BASE: u64 = 0xa0000000;
    pub const MMIO_BASE: u64 = 0xa0000000;
    pub const SERIAL_PORT: u64 = DEVICE_BASE + 0x00003f8;
    pub const KBD_ADDR: u64 = DEVICE_BASE + 0x0000060;
    pub const VGACTL_ADDR: u64 = DEVICE_BASE + 0x0000100;
    pub const AUDIO_ADDR: u64 = DEVICE_BASE + 0x0000200;
    pub const DISK_ADDR: u64 = DEVICE_BASE + 0x2000000;

    pub const VGA_ADDR: u64 = MMIO_BASE + 0x1000000;
    pub const AUDIO_SBUF_ADDR: u64 = MMIO_BASE + 0x1200000;
    pub const TIMER_ADDR: u64 = MMIO_BASE + 0x48;
}

/* Interface of ROS and RAPPS */
pub mod syscall {
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
}
/* Layout of OS kernel */
// TODO: a graph
pub mod layout {
    pub const KERNEL_START: usize = 0x80000000;
    pub const KERNEL_HEAP_START: usize = 0x84000000;
    pub const KERNEL_HEAP_END: usize = 0xa0000000;
}
