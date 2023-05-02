use std::ops::Index;
use std::ops::IndexMut;

use super::instruction::Instruction;
use super::mem;
use super::reg;
use crate::error::RError;
use crate::isas::instruction::Inst;
use crate::isas::{isa::ISA, mem::MemoryModel, reg::RegisterModel};

pub struct RiscvCPU {
    regs: reg::Regs,
    pub mems: mem::Mem,
    pub mode: PrivilegeMode,
}

#[derive(Debug, Clone, Copy)]
pub enum PrivilegeMode {
    User = 0,
    Supervisor = 1,
    Reserved = 2,
    Machine = 3,
}

impl Default for RiscvCPU {
    fn default() -> Self {
        RiscvCPU {
            regs: reg::Regs::new(),
            mems: mem::Mem::new(),
            mode: PrivilegeMode::Machine,
        }
    }
}

impl RiscvCPU {
    pub fn new(regs: reg::Regs, mems: mem::Mem) -> Self {
        RiscvCPU {
            regs,
            mems,
            mode: PrivilegeMode::Machine,
        }
    }
}

impl Index<u32> for RiscvCPU {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.regs[index]
    }
}

impl IndexMut<u32> for RiscvCPU {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.regs[index]
    }
}

impl MemoryModel for RiscvCPU {
    fn load_mem(&mut self, index: u32, bytes: u8) -> Option<u32> {
        self.mems.load_mem(index, bytes)
    }

    fn store_mem(&mut self, index: u32, bytes: u8, value: u32) {
        self.mems.store_mem(index, bytes, value);
    }
}

impl RegisterModel for RiscvCPU {
    fn read_register_by_name(&self, name: &str) -> Option<u32> {
        self.regs.read_register_by_name(name)
    }

    fn write_register_by_name(&mut self, name: &str, value: u32) {
        self.regs.write_register_by_name(name, value);
    }
    fn name_to_index(&self, name: &str) -> Option<u32> {
        self.regs.name_to_index(name)
    }
    fn iter(&self) -> Box<dyn Iterator<Item = (String, u32)>> {
        self.regs.iter()
    }

    fn read_register_previlege(&self, index: u32) -> Option<u32> {
        self.regs.read_register_previlege(index)
    }

    fn write_register_previlege(&mut self, index: u32, value: u32) {
        self.regs.write_register_previlege(index, value);
    }
}

impl ISA for RiscvCPU {
    fn name(&self) -> String {
        "RISC-V 32".to_string()
    }

    fn xlen(&self) -> u32 {
        32
    }

    fn disassemble(&mut self, addr: u32) -> Result<String, RError> {
        let inst_code = self.fetch_inst(addr)?;
        let inst = Instruction::decode(inst_code)?;
        Ok(inst.to_string())
    }

    fn load_from_assembly(_strs: Vec<&str>) -> Result<Self, RError> {
        todo!()
    }

    fn execute(&mut self, inst_code: u32) -> Result<u32, RError> {
        // every time execute an instruction, mcycle and minstret should be increased
        self.write_register_by_name("mcycle", self.read_register_by_name("mcycle").unwrap() + 1);
        self.write_register_by_name(
            "minstret",
            self.read_register_by_name("minstret").unwrap() + 1,
        );
        let inst = Instruction::decode(inst_code)?;
        inst.execute(self)
    }

    fn device_update(&mut self) -> Result<(), RError> {
        static mut START_TIME: u32 = 1;
        unsafe {
            if START_TIME % 10000 == 0 {
                // debug!("update devices");
                self.mems.update_devices();
            }
            START_TIME += 1;
        }
        Ok(())
    }

    fn execute_assem(&mut self, assembly: &str) -> Result<u32, RError> {
        let inst = Instruction::parse_assembly(assembly, self)?;
        inst.execute(self)
    }
}

#[cfg(test)]
mod tests {

    use super::RiscvCPU;

    #[test]
    fn test_basic() {
        let mut riscvisa = RiscvCPU::default();
        riscvisa[4] = 100;
        assert_eq!(riscvisa[4], 100);
    }
}
