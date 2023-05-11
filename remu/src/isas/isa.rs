use super::mem::MemoryModel;
use super::reg::RegisterModel;
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
