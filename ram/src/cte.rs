#![allow(unreachable_code)]
//! ---------- CTE: Interrupt Handling and Context Switching ----------
//! - init register exception handler
//! - yield to kernel
//! - ienable and iset to turn on interrupt and set interrupt mode
//! - Context: register context
//! - Event: interrupt event

#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use core::arch::{asm, global_asm};

global_asm!(include_str!("trap.S"));

static mut IRQ: Option<fn(Event, &mut Context)> = None;

/// init exception entry, register exception handler
#[no_mangle]
pub fn init(irq: fn(Event, &mut Context)) {
    unsafe {
        IRQ = Some(irq);
    }
    #[cfg(target_arch = "riscv32")]
    unsafe {
        asm!(
            "la t0, am_asm_trap",
            "csrw mtvec, t0",
            // "csrw mstatus, {x1}",
            // x1 = in(reg) 0x1800,
        );
    }
    iset(true); // turn on interrupt
}

#[no_mangle]
pub fn _am_handler(context: &mut Context) {
    let event = match context.mcause {
        0xb => match context.regs[17] {
            0xffffffff => Event::Yield,
            _ => Event::Syscall,
        },
        _ => Event::Error,
    };
    unsafe {
        IRQ.unwrap()(event, context);
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub regs: [u32; 32],
    pub mcause: u32,
    pub mstatus: u32,
    pub mepc: u32,
}

impl Context {
    pub fn new(entry: u32, id: usize) -> Self {
        Context {
            regs: [0; 32],
            mcause: 0,
            mstatus: id as u32,
            mepc: entry,
        }
    }

    pub fn assign_to(&self, context: &mut Context) {
        context.regs = self.regs;
        context.mcause = self.mcause;
        context.mstatus = self.mstatus;
        context.mepc = self.mepc;
    }
}

#[no_mangle]
pub fn yield_() {
    #[cfg(target_arch = "riscv32")]
    unsafe {
        asm!("li a7, -1", "ecall");
    }
}

pub fn ienable() -> bool {
    #[cfg(target_arch = "riscv32")]
    unsafe {
        let mut old: u32 = 0;
        asm!(
            "csrr {x0}, mie",
            "csrrw {x0}, mie, {x1}",
            "andi {x0}, {x0}, 0x80",
            "slli {x0}, {x0}, 0x1f",
            "srli {x0}, {x0}, 0x1f",
            x0 = out(reg) old,
            x1 = in(reg) 0x80,
        );
        return old != 0;
    }
    false
}

pub fn iset(_enable: bool) {
    #[cfg(target_arch = "riscv32")]
    unsafe {
        asm!(
            "csrrw x0, mie, {x1}",
            x1 = in(reg) if _enable { 0x80 } else { 0 },
        );
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum Event {
    Yield,
    Syscall,
    Pagefault,
    Error,
    Timer,
    Iodev,
}

// Context *kcontext    (Area kstack, void (*entry)(void *), void *arg);
