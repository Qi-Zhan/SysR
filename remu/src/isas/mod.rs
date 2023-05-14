pub mod riscv;

use crate::error::RError;

pub trait ISA: MemoryModel + RegisterModel + Sized {
    fn name(&self) -> String;

    /// decide whether 32bit or 64bit
    fn xlen(&self) -> u32;

    #[inline]
    fn run(&mut self) -> Result<(), RError> {
        loop {
            self.step()?;
        }
    }

    fn load_from_assembly(strs: Vec<&str>) -> Result<Self, RError>;

    fn step(&mut self) -> Result<(), RError> {
        let pc = self.pc();
        let inst_code = self.fetch_inst(pc)?;

        match self.execute(inst_code) {
            Ok(next_pc) => {
                self.update_pc(next_pc);
                self.device_update()
            }
            Err(e) => Err(e),
        }
    }

    fn device_update(&mut self) -> Result<(), RError>;

    #[inline]
    fn fetch_inst(&mut self, pc: u32) -> Result<u32, RError> {
        Ok(self.load_mem(pc, 4).unwrap())
    }

    fn disassemble(&mut self, addr: u32) -> Result<String, RError>;

    fn execute(&mut self, inst_code: u32) -> Result<u32, RError>;

    fn execute_assem(&mut self, assembly: &str) -> Result<u32, RError>;
}

pub trait MemoryModel {
    fn load_mem(&mut self, index: u32, bytes: u8) -> Option<u32>;
    fn store_mem(&mut self, index: u32, bytes: u8, value: u32);
    fn store_mems(&mut self, index: u32, value: &[u32]) {
        for (i, item) in value.iter().enumerate() {
            self.store_mem(index + i as u32, 1, *item);
        }
    }
}

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub trait RegisterModel: Index<u32, Output = u32> + IndexMut<u32> {
    fn read_register_by_name(&self, name: &str) -> Option<u32>;

    fn write_register_by_name(&mut self, name: &str, value: u32);
    
    fn name_to_index(&self, name: &str) -> Option<u32>;

    fn iter(&self) -> Box<dyn Iterator<Item = (String, u32)>>;

    fn read_register_previlege(&self, index: u32) -> Option<u32>;

    fn write_register_previlege(&mut self, index: u32, value: u32);

    fn pc(&self) -> u32;

    fn update_pc(&mut self, pc: u32);
}

pub trait Inst: Display + Clone + Copy + PartialEq + Eq + Sized {
    /// Assemble the instruction into a 32-bit machine code.
    fn assemble(&self) -> u32;
    /// Disassemble the instruction from Inst.
    fn disassemble(&self) -> String {
        self.to_string()
    }
    /// Parse the assembly into an instruction.
    fn parse_assembly(assembly: &str, cpu: &impl ISA) -> Result<Self, RError>
    where
        Self: Sized;
    /// Execute the instruction.
    fn execute(&self, cpu: &mut impl ISA) -> Result<u32, RError>;
    /// decode the 32-bit machine code into an instruction.
    fn decode(machine_code: u32) -> Result<Self, RError>
    where
        Self: Sized;
}
