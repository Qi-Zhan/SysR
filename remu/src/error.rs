use std::fmt::Display;

use colored::Colorize;

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
            RError::CPUError(s) => write!(f, "{}: {}",("CPUError").bold().red() , s),
            RError::DebuggerError(s) => write!(f, "{}: {}", ("DebuggerError").bold().red(), s),
            RError::InvalidInstruction(s) => write!(f, "{}: {}", ("InvalidInstruction").bold().red(), s),
            RError::InvalidCode(code) => write!(f, "{}: {:b}", ("InvalidCode").bold().red(), code),
            RError::InvalidRegister(reg) => write!(f, "{}: {}", ("InvalidRegister").bold().red(), reg),
            RError::InvalidMem(s) => write!(f, "{}: {:#x}", ("InvalidMem").bold().red(), s),
            RError::AddressMisaligned(s) => write!(f, "{}: {:#x}", ("AddressMisaligned").bold().red(), s),
            RError::InvalidAssembly(s) => write!(f, "{}: {}", ("InvalidAssembly").bold().red(), s),
            RError::Ebreak(return_code) => {
                match *return_code {
                    0 => write!(f, "{}", ("program exited").bold().green()),
                    _ => write!(f, "{} with code {}", ("program exited").bold().red() , return_code),
                }
            }
            RError::Ecall => write!(f, "{}", ("Ecall").bold().red()),
            RError::IOError(s) => write!(f, "{}: {}", ("IOError").bold().red(), s),
            RError::Other(s) => write!(f, "{}: {}", ("Other").bold().red(), s),
        }
    }
}
