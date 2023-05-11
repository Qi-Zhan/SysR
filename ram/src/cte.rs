// ---------- CTE: Interrupt Handling and Context Switching ----------

use core::arch::{asm, global_asm};

// global_asm!(include_str!("trap.S"));

/// init exception entry, 
pub fn init(handler: fn(Event, &Context) -> Context) {
    // #[cfg(target_arch="riscv32")]
    // println!("{:p}", moo as *const ());
    unsafe {
        asm!(
            "la t0, am_asm_trap",
            "csrw mtvec, t0",
            // mstatus初始化为0x1800.
            "csrw mstatus, {x1}",
            // x0 = in(reg) am_handler as *const () as u32,
            // x0 = in(reg) am_asm_trap as u32,
            x1 = in(reg) 0x1800,
        );
    }
    todo!("init");
}

pub fn am_handler(context: &Context) -> Context {
    let ev = match context.mcause {
        0xffffffff  =>  Event::Yield,
        _           =>  Event::Error,
        
    };

    todo!("am_handler");
}

#[repr(C)]
pub struct Context {
    pub regs:       [u32; 32],
    pub mcause:     u32,
    pub mstatus:    u32,
    pub mepc:       u32,
}

pub fn yield_() {
    #[cfg(target_arch="riscv32")]
    unsafe {
        asm!(
            "li a7, -1",
            "ecall",
        );
    }
}

pub fn ienable() -> bool {
    let mut old: u32 = 0;
    #[cfg(target_arch="riscv32")]
    unsafe {
        asm!(
            "csrr {x0}, mie",
            "csrrw {x0}, mie, {x1}",
            "andi {x0}, {x0}, 0x80",
            "slli {x0}, {x0}, 0x1f",
            "srli {x0}, {x0}, 0x1f",
            x0 = out(reg) old,
            x1 = in(reg) 0x80,
        );
    }
    old != 0
}

pub fn iset(enable: bool) {
    #[cfg(target_arch="riscv32")]
    unsafe {
        asm!(
            "csrrw x0, mie, {x1}",
            x1 = in(reg) if enable { 0x80 } else { 0 },
        );
    }
}

pub enum Event {
    Null,
    Yield,
    Syscall,
    Pagefault,
    Error,
    Timer,
    Iodev,
}

// Context *kcontext    (Area kstack, void (*entry)(void *), void *arg);

