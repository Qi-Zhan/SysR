use lazy_static::lazy_static;
use std::{collections::HashMap, ops::{Index, IndexMut}};

use crate::isas::reg::RegisterModel;

const REG_NUM: usize = 32;
const PRIVILEGE_REG_NUM: usize = 0x1000;

lazy_static! {

    static ref INDEX2NAME: [&'static str; REG_NUM] = [
        "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
        "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11", "t3", "t4",
        "t5", "t6",
    ];

    static ref NAME2INDEX: HashMap<&'static str, u32> = {
        let mut m = HashMap::new();
        for i in 0..REG_NUM {
            m.insert(INDEX2NAME[i], i as u32);
        }
        m         
    };

    static ref INDEX2CSR: HashMap<u16, &'static str> = {
        let csr_list = vec![
            (0x001, "fflags"), (0x002, "frm"), (0x003, "fcsr"), 
            (0x100, "sstatus"), (0x104, "sie"), (0x105, "stvec"),
            (0x106, "scounteren"), (0x140, "sscratch"), (0x141, "sepc"), 
            (0x142, "scause"), (0x143, "stval"), (0x144, "sip"), 
            (0x180, "satp"), 
            (0xC00, "cyclel"), (0xC01, "time"), (0xC02, "instret"), 
            // TODO: Add hpmcounter* 
            (0xC80, "cycleh"), (0xC81, "timeh"), (0xC82, "instreth"),
            // TODO: Add hpmcounter*h
            (0x300, "mstatus"), (0x301, "misa"), (0x302, "medeleg"), 
            (0x303, "mideleg"), (0x304, "mie"), (0x305, "mtvec"), 
            (0x306, "mcounteren"), (0x310, "mscratch"), (0x340, "mscratch"), 
            (0x341, "mepc"), (0x342, "mcause"), (0x343, "mtval"),
            (0x344, "mip"), (0x34A, "mtinst"), 
            (0x7A0, "mcycle"), (0x7A1, "minstret"), (0xB00, "mcycleh"), (0xB01, "minstreth"),
            (0x300, "mstatus")
            ];
        let mut m = HashMap::new();
        csr_list.iter().for_each(|(index, name)| {
            m.insert(*index, *name);
        });
        m
    };

    static ref CSR2INDEX: HashMap<&'static str, u16> = {
        let mut m = HashMap::new();
        INDEX2CSR.iter().for_each(|(index, name)| {
            m.insert(*name, *index);
        });
        m
    };



}

/// RISC-V Register Model
#[derive(Debug, Clone)]
pub struct Regs {
    regs: [u32; REG_NUM],

    pc: u32,

    csr: [u32; 0x1000],
}

impl Default for Regs {
    fn default() -> Self {
        Self::new()
    }
}

impl Regs {
    pub fn new() -> Self {
        Regs {
            regs: [0; REG_NUM],
            pc: 0,
            csr: [0; PRIVILEGE_REG_NUM],
        }
    }

    pub(super) fn index_to_name(index: u32) -> String {
        if index >= REG_NUM as u32 {
            panic!("Invalid register index: {}", index);
        }
        INDEX2NAME[index as usize].to_string()
    }

}

impl Index<u32> for Regs {
    type Output = u32;

    fn index(&self, index: u32) -> &Self::Output {
        if index >= REG_NUM as u32 {
            panic!("Invalid register index: {}", index);
        }
        if index == 0 {
            return &0;
        }
        &self.regs[index as usize]
    }
}

impl IndexMut<u32> for Regs {
    fn index_mut(&mut self, index: u32) -> &mut Self::Output {
        if index >= REG_NUM as u32 {
            panic!("Invalid register index: {}", index);
        }
        &mut self.regs[index as usize]
    }
}

impl RegisterModel for Regs {

    // fn read_register(&self, index: u32) -> Option<u32> {
    //     if index > REG_NUM as u32 {
    //         return None;
    //     }
    //     if index == 0 {
    //         return Some(0);
    //     }
    //     Some(self.regs[index as usize])
    // }

    fn name_to_index(&self, name: &str) -> Option<u32> {
        if let Some(index) = name.strip_prefix('x') {
            let index = index.parse::<u32>().ok()?;
            if index < REG_NUM as u32 {
                return Some(index);
            }
            None // Invalid register index
        } else {
            NAME2INDEX.get(name).copied()
        }
    }

    fn read_register_by_name(&self, name: &str) -> Option<u32> {
        let index = self.name_to_index(name);
        match index {
            Some(index) => Some(self[index]),
            None => match name {
                "pc" => Some(self.pc),
                _ => {
                    match CSR2INDEX.get(name) {
                        Some(index) => self.read_register_previlege((*index).into()),
                        None => None,
                    }
                }
            },
        }
    }

    fn read_register_previlege(&self, index: u32) -> Option<u32> {
        if index >= PRIVILEGE_REG_NUM as u32 {
            return None;
        }
        Some(self.csr[index as usize])
    }

    fn write_register_previlege(&mut self, index: u32, value: u32) {
        if index >= PRIVILEGE_REG_NUM as u32 {
            return;
        }
        self.csr[index as usize] = value;
    }

    // fn write_register(&mut self, index: u32, value: u32) {
    //     if index <= 31 {
    //         self.regs[index as usize] = value;
    //     }
    // }

    fn write_register_by_name(&mut self, name: &str, value: u32) {
        let index = self.name_to_index(name);
        match index {
            Some(index) => self[index] = value,
            None => match name {
                "pc" => self.pc = value,
                _ => {
                    if let Some(index) = CSR2INDEX.get(name) { 
                        self.write_register_previlege((*index).into(), value) 
                    }
                }
            },
        }
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (String, u32)>> {
        let mut registers = Vec::new();
        for i in 0..32 {
            registers.push((Regs::index_to_name(i), self.regs[i as usize]));
        }
        registers.push(("pc".to_string(), self.pc));
        Box::new(registers.into_iter())
    }
}
