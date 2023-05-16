use std::fmt::Display;
use crate::color::{bold_red, bold_green};

#[derive(Debug)]
pub enum RError {
    CPUError(String),
    DebuggerError(String),
    InvalidInstruction(String),
    InvalidCode(u32),
    InvalidRegister(u32),
    InvalidMem(u32),
    InvalidAssembly(String),
    AddressMisaligned(u32),
    IOError(String),
    Ebreak(i8),
    Ecall,
    Other(String),
}

impl Display for RError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RError::CPUError(s) => write!(f, "{}: {}",bold_red("CPUError") , s),
            RError::DebuggerError(s) => write!(f, "{}: {}", bold_red("DebuggerError"), s),
            RError::InvalidInstruction(s) => write!(f, "{}: {}", bold_red("InvalidInstruction"), s),
            RError::InvalidCode(code) => write!(f, "{}: {:b}", bold_red("InvalidCode"), code),
            RError::InvalidRegister(reg) => write!(f, "{}: {}", bold_red("InvalidRegister"), reg),
            RError::InvalidMem(s) => write!(f, "{}: {:#x}", bold_red("InvalidMem"), s),
            RError::AddressMisaligned(s) => write!(f, "{}: {:#x}", bold_red("AddressMisaligned"), s),
            RError::InvalidAssembly(s) => write!(f, "{}: {}", bold_red("InvalidAssembly"), s),
            RError::Ebreak(return_code) => {
                match *return_code {
                    0 => write!(f, "{}", bold_green("program exited normally")),
                    _ => write!(f, "{} with code {}", bold_red("program exited") , return_code),
                }
            }
            RError::Ecall => write!(f, "{}", bold_red("Ecall")),
            RError::IOError(s) => write!(f, "{}: {}", bold_red("IOError"), s),
            RError::Other(s) => write!(f, "{}: {}", bold_red("Error"), s),
        }
    }
}
