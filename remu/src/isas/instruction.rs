use std::fmt::Display;
use crate::error::RError;
use super::ISA;

pub trait Inst: Display + Clone + Copy + PartialEq + Eq + Sized {
    /// Assemble the instruction into a 32-bit machine code.
    fn assemble(&self) -> u32;
    /// Disassemble the instruction from Inst.
    fn disassemble(&self) -> String {
        self.to_string()
    }
    /// Parse the assembly into an instruction.
    fn parse_assembly(assembly: &str, cpu: &impl ISA) -> Result<Self, RError> where Self: Sized;
    /// Execute the instruction.
    fn execute(&self, cpu: &mut impl ISA) -> Result<u32, RError>;
    /// decode the 32-bit machine code into an instruction.
    fn decode(machine_code: u32) -> Result<Self, RError> where Self: Sized;
}
