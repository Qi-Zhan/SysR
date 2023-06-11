pub mod instruction;
pub mod mem;
pub mod reg;

use std::ops::Index;
use std::ops::IndexMut;

use crate::error::RError;
use crate::isas::{Inst, MemoryModel, RegisterModel, ISA};
use crate::warn;
use instruction::Instruction;

pub struct RV32CPU {
    regs: reg::Regs,
    pub mems: mem::Mem,
    mode: PrivilegeMode,
}

#[derive(Debug, Clone, Copy)]
pub enum PrivilegeMode {
    User = 0,
    Supervisor = 1,
    // Reserved = 2,
    Machine = 3,
}

impl Default for RV32CPU {
    fn default() -> Self {
        RV32CPU {
            regs: reg::Regs::new(),
            mems: mem::Mem::new(),
            mode: PrivilegeMode::Supervisor,
        }
    }
}

impl RV32CPU {
    pub fn new(regs: reg::Regs, mems: mem::Mem) -> Self {
        RV32CPU {
            regs,
            mems,
            mode: PrivilegeMode::Supervisor,
        }
    }
}

impl Index<u32> for RV32CPU {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        &self.regs[index]
    }
}

impl IndexMut<u32> for RV32CPU {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        &mut self.regs[index]
    }
}

impl MemoryModel for RV32CPU {
    fn load_mem(&mut self, index: u32, bytes: u8) -> Option<u32> {
        // virtual address for user mode
        let size = rconfig::layout::USER_APP_SIZE as u32;
        let index = match self.mode {
            PrivilegeMode::User => {
                let id = self.read_register_by_name("mstatus").unwrap();
                index + size * id
            }
            _ => index,
        };
        self.mems.load_mem(index, bytes)
    }

    fn store_mem(&mut self, index: u32, bytes: u8, value: u32) {
        // virtual address for user mode
        let size = rconfig::layout::USER_APP_SIZE as u32;
        let index = match self.mode {
            PrivilegeMode::User => {
                let id = self.read_register_by_name("mstatus").unwrap();
                index + size * id
            }
            _ => index,
        };
        self.mems.store_mem(index, bytes, value);
    }
}

impl RegisterModel for RV32CPU {
    #[inline]
    fn read_register_by_name(&self, name: &str) -> Option<u32> {
        self.regs.read_register_by_name(name)
    }

    #[inline]
    fn write_register_by_name(&mut self, name: &str, value: u32) {
        self.regs.write_register_by_name(name, value);
    }

    #[inline]
    fn name_to_index(&self, name: &str) -> Option<u32> {
        self.regs.name_to_index(name)
    }

    #[inline]
    fn iter(&self) -> Box<dyn Iterator<Item = (String, u32)>> {
        self.regs.iter()
    }

    #[inline]
    fn read_register_previlege(&self, index: u32) -> Option<u32> {
        self.regs.read_register_previlege(index)
    }

    #[inline]
    fn write_register_previlege(&mut self, index: u32, value: u32) {
        self.regs.write_register_previlege(index, value);
    }

    #[inline]
    fn pc(&self) -> u32 {
        self.regs.pc()
    }

    #[inline]
    fn update_pc(&mut self, pc: u32) {
        self.regs.update_pc(pc);
    }
}

impl ISA for RV32CPU {
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

    fn execute(&mut self, inst_code: u32) -> Result<u32, RError> {
        match Instruction::decode(inst_code) {
            Ok(inst) => inst.execute(self),
            Err(err) => {
                warn!("invalid code at {:x}", self.regs.pc());
                Err(err)
            }
        }
    }

    fn device_update(&mut self) -> Result<(), RError> {
        static mut START_TIME: u32 = 1;
        unsafe {
            if START_TIME % 10000 == 0 {
                self.mems.update_devices();
            }
            START_TIME += 1;
        }
        Ok(())
    }

    fn priviledge_level_down(&mut self) {
        self.mode = match self.mode {
            PrivilegeMode::User => PrivilegeMode::User,
            PrivilegeMode::Supervisor => PrivilegeMode::User,
            PrivilegeMode::Machine => PrivilegeMode::Supervisor,
        }
    }

    fn priviledge_level_up(&mut self) {
        self.mode = match self.mode {
            PrivilegeMode::User => PrivilegeMode::Supervisor,
            PrivilegeMode::Supervisor => PrivilegeMode::Machine,
            PrivilegeMode::Machine => PrivilegeMode::Machine,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::RV32CPU;

    #[test]
    fn test_basic() {
        let mut riscvisa = RV32CPU::default();
        riscvisa[4] = 100;
        assert_eq!(riscvisa[4], 100);
    }
}
