#![allow(clippy::enum_variant_names)]

use super::reg::Regs;
use std::{fmt::Display, num::ParseIntError};

use crate::{
    error::RError,
    isas::{instruction::Inst, isa::ISA},
};

type Imm = u32;
type Src = (u32, u32);
type Dst = u32;
type Fun3 = u32;
type Fun7 = u32;
type Opcode = u32;
type Csr = u32;



fn opcode(code: u32) -> u32 {
    code & 0x7f
}

fn src(code: u32) -> Src {
    ((code >> 15) & 0x1f, (code >> 20) & 0x1f)
}

fn dst(code: u32) -> Dst {
    (code >> 7) & 0x1f
}

fn fun3(code: u32) -> Fun3 {
    (code >> 12) & 0x7
}

fn fun7(code: u32) -> Fun7 {
    (code >> 25) & 0x7f
}

fn imm(code: u32) -> Imm {
    match opcode(code) {
        0b0110111 | 0b0010111 => code & 0xfffff000, // U-type
        0b1101111 => {
            // J-type
            let imm20 = (code >> 31) & 0x1;
            let imm10_1 = (code >> 21) & 0x3ff;
            let imm11 = (code >> 20) & 0x1;
            let imm19_12 = (code >> 12) & 0xff;
            let most = code & 0x80000000;
            (imm20 << 20)
                | (imm19_12 << 12)
                | (imm11 << 11)
                | (imm10_1 << 1)
                | ((most as i32) >> 11) as u32
        }
        0b1100111 | 0b0010011 | 0b0000011 => {
            // I-type
            let most = code & 0x80000000;
            // most bit arithmetically extended to 20 bits
            ((most as i32) >> 20) as u32 | (code >> 20) & 0xfff
        }
        0b1100011 => {
            // B-type
            let imm12 = (code >> 31) & 0x1;
            let imm10_5 = (code >> 25) & 0x3f;
            let imm4_1 = (code >> 8) & 0xf;
            let imm11 = (code >> 7) & 0x1;
            let most = code & 0x80000000;
            // most bit arithmetically extended to 20 bits
            ((most as i32) >> 20) as u32
                | (imm12 << 12)
                | (imm11 << 11)
                | (imm10_5 << 5)
                | (imm4_1 << 1)
        }
        0b0100011 => {
            // S-type
            let imm11_5 = (code >> 25) & 0x7f;
            let imm4_0 = (code >> 7) & 0x1f;
            let most = code & 0x80000000;
            (imm11_5 << 5) | imm4_0 | ((most as i32) >> 20) as u32
        }
        _ => panic!("Invalid opcode {:x}", opcode(code)),
    }
}

fn csr(code: u32) -> Csr {
    (code >> 20) & 0xfff
}

fn get(code: u32, high: u32, low: u32) -> u32 {
    (code >> low) & ((1 << (high - low + 1)) - 1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Instruction {
    /*
     * R-type instruction
     *
     * 31    25  24 20 19   15 14  12 11  7 6      0
     * ---------------------------------------------
     * | funct7 | rs2 | rs1 | funct3 | rd | opcode |
     * ---------------------------------------------
     */
    RType(Fun7, Src, Fun3, Dst, Opcode),
    /*
     * I-type instruction
     *
     * 31    25 24  19 15 14 12 11     7 6      0
     * ---------------------------------------------
     * | imm[11:0] | rs1 | funct3 | rd |    opcode |
     * ---------------------------------------------
     */
    IType(Imm, Src, Fun3, Dst, Opcode),
    CSRType(Csr, Src, Fun3, Dst, Opcode),
    /*
     * S-type instruction
     * 31    25 24  19 15 14 12 11     7 6      0
     * ---------------------------------------------
     * | imm[11:5] | rs2 | rs1 | funct3 | imm[4:0] | opcode |
     * ---------------------------------------------
     */
    SType(Imm, Src, Fun3, Opcode),
    /*
     * B-type instruction
     * 31    25 24  19 15 14 12 11     7 6      0
     * ---------------------------------------------
     * | imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode |
     * ---------------------------------------------
     */
    BType(Imm, Src, Fun3, Opcode),
    /*
     * U-type instruction
     * 31    25 24  19 15 14 12 11     7 6      0
     * ---------------------------------------------
     * | imm[31:12]              | rd | opcode |
     * ---------------------------------------------
     */
    UType(Imm, Dst, Opcode),
    /*
     * J-type instruction
     * 31    25 24  19 15 14 12 11     7 6      0
     * ---------------------------------------------
     * | imm[20|10:1|11|19:12] | rd | opcode |
     * ---------------------------------------------
     */
    JType(Imm, Dst, Opcode),
    Nop,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::RType(funct7, (rs1, rs2), funct3, rd, _) => {
                match funct3 {
                    0b000 => match funct7 {
                        0b0000000 => {
                            // if add and rd == 0, it's nop
                            if *rd == 0 {
                                return write!(f, "nop");
                            } else {
                                write!(f, "add")
                            }
                        }
                        0b0100000 => write!(f, "sub"),
                        _ => panic!("Invalid funct7 {:x}", funct7),
                    },
                    0b001 => write!(f, "sll"),
                    0b010 => write!(f, "slt"),
                    0b011 => write!(f, "sltu"),
                    0b100 => write!(f, "xor"),
                    0b101 => match funct7 {
                        0b0000000 => write!(f, "srl"),
                        0b0100000 => write!(f, "sra"),
                        _ => panic!("Invalid funct7 {:x}", funct7),
                    },
                    0b110 => write!(f, "or"),
                    0b111 => write!(f, "and"),
                    _ => panic!("Invalid funct3 {:x}", funct3),
                }?;
                write!(
                    f,
                    " {}, {}, {}",
                    Regs::index_to_name(*rd),
                    Regs::index_to_name(*rs1),
                    Regs::index_to_name(*rs2)
                )
            }
            Instruction::IType(imm, (rs1, _), funct3, rd, opcode) => match opcode {
                0b1100111 => {
                    write!(f, "jalr x{}, x{}, {}", rd, rs1, imm)
                }
                0b0000011 => {
                    match funct3 {
                        0b000 => write!(f, "lb"),
                        0b001 => write!(f, "lh"),
                        0b010 => write!(f, "lw"),
                        0b100 => write!(f, "lbu"),
                        0b101 => write!(f, "lhu"),
                        _ => panic!("Invalid funct3 {:x}", funct3),
                    }?;
                    write!(f, " x{}, {}(x{})", rd, imm, rs1)
                }
                0b0010011 => {
                    match funct3 {
                        0b000 => {
                            if *rs1 == 0 {
                                return write!(f, "li {} {}", Regs::index_to_name(*rd), imm);
                            } else if *imm == 0 {
                                return write!(
                                    f,
                                    "mv {} {}",
                                    Regs::index_to_name(*rd),
                                    Regs::index_to_name(*rs1)
                                );
                            } else {
                                write!(f, "addi")
                            }
                        }
                        0b001 => write!(f, "slli"),
                        0b010 => write!(f, "slti"),
                        0b011 => write!(f, "sltiu"),
                        0b100 => write!(f, "xori"),
                        0b101 => {
                            match get(*imm, 11, 10) {
                                0b00 => write!(f, "srli")?,
                                0b01 => write!(f, "srai")?,
                                _ => panic!("Invalid funct7 {:x}", get(*imm, 11, 10)),
                            };
                            return write!(
                                f,
                                " {}, {}, {}",
                                Regs::index_to_name(*rd),
                                Regs::index_to_name(*rs1),
                                get(*imm, 5, 0)
                            );
                        }
                        0b110 => write!(f, "ori"),
                        0b111 => write!(f, "andi"),
                        _ => panic!("Invalid funct3 {:x}", funct3),
                    }?;
                    write!(
                        f,
                        " {}, {}, {:#x}",
                        Regs::index_to_name(*rd),
                        Regs::index_to_name(*rs1),
                        imm
                    )
                }
                _ => panic!("Invalid opcode {:x}", opcode),
            },
            Instruction::CSRType(csr, (rs1, _), funct3, rd, _) => {
                match *funct3 {
                    0b001 => write!(f, "csrrw"),
                    0b010 => write!(f, "csrrs"),
                    0b011 => write!(f, "csrrc"),
                    0b101 => write!(f, "csrrwi"),
                    0b110 => write!(f, "csrrsi"),
                    0b111 => write!(f, "csrrci"),
                    0b000 => {
                        return match csr {
                            0b000000000000 => write!(f, "ecall"),
                            0b000000000001 => write!(f, "ebreak"),
                            0b000100000010 => write!(f, "sret"),
                            0b001100000010 => write!(f, "mret"),
                            0b000100000101 => write!(f, "wfi"),
                            _ => panic!("Invalid csr 0x{:x}", self.assemble()),
                        }
                    }
                    _ => panic!("Invalid funct3 in csr 0x{:x}", funct3),
                }?;
                write!(
                    f,
                    " {}, {}, 0x{:x}",
                    Regs::index_to_name(*rd),
                    Regs::index_to_name(*rs1),
                    csr
                )
            }
            Instruction::SType(imm, (rs1, rs2), funct3, _) => {
                match funct3 {
                    0b000 => write!(f, "sb"),
                    0b001 => write!(f, "sh"),
                    0b010 => write!(f, "sw"),
                    0b100 => write!(f, "sd"),
                    _ => panic!("Invalid funct3 {:x}", funct3),
                }?;
                write!(
                    f,
                    " {} , {}({})",
                    Regs::index_to_name(*rs2),
                    imm,
                    Regs::index_to_name(*rs1)
                )
            }
            Instruction::BType(imm, (rs1, rs2), funct3, _) => {
                match funct3 {
                    0b000 => write!(f, "beq"),
                    0b001 => write!(f, "bne"),
                    0b100 => write!(f, "blt"),
                    0b101 => write!(f, "bge"),
                    0b110 => write!(f, "bltu"),
                    0b111 => write!(f, "bgeu"),
                    _ => panic!("Invalid funct3 {:x}", funct3),
                }?;
                write!(
                    f,
                    " {}, {}, 0x{:x}",
                    Regs::index_to_name(*rs1),
                    Regs::index_to_name(*rs2),
                    imm
                )
            }
            Instruction::UType(imm, rd, _) => {
                write!(f, "lui {}, 0x{:x}", Regs::index_to_name(*rd), imm >> 12)
            }
            Instruction::JType(imm, rd, _) => {
                write!(f, "jal {}, 0x{:x}", Regs::index_to_name(*rd), imm)
            }
            Instruction::Nop => write!(f, "nop"),
        }
    }
}

fn parse_str(s: &str) -> Result<u32, ParseIntError> {
    if let Some(new) = s.strip_prefix("0x") {
        u32::from_str_radix(new, 16)
    } else {
        s.parse::<u32>()
    }
}

impl Inst for Instruction {
    fn assemble(&self) -> u32 {
        match self {
            Instruction::RType(funct7, (rs1, rs2), funct3, rd, opcode) => {
                (funct7 << 25) | (rs2 << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
            }
            Instruction::IType(imm, (rs1, _), funct3, rd, opcode) => {
                (imm << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
            }
            Instruction::SType(imm, (rs1, rs2), funct3, opcode) => {
                ((get(*imm, 11, 5)) << 25)
                    | (rs2 << 20)
                    | (rs1 << 15)
                    | (funct3 << 12)
                    | (get(*imm, 4, 0) << 7)
                    | opcode
            }
            Instruction::BType(imm, (rs1, rs2), funct3, opcode) => {
                ((get(*imm, 12, 12)) << 31)
                    | ((get(*imm, 10, 5)) << 25)
                    | (rs2 << 20)
                    | (rs1 << 15)
                    | (funct3 << 12)
                    | ((get(*imm, 4, 1) << 8) | (get(*imm, 11, 11) << 7))
                    | opcode
            }
            Instruction::UType(imm, rd, opcode) => (imm << 12) | (rd << 7) | opcode,
            Instruction::JType(imm, rd, opcode) => {
                ((get(*imm, 20, 20)) << 31)
                    | ((get(*imm, 10, 1)) << 21)
                    | ((get(*imm, 11, 11)) << 20)
                    | ((get(*imm, 19, 12)) << 12)
                    | (rd << 7)
                    | opcode
            }
            Instruction::CSRType(csr, (rs1, _), funct3, rd, opcode) => {
                (csr << 20) | (rs1 << 15) | (funct3 << 12) | (rd << 7) | opcode
            }
            Instruction::Nop => 0b0001111,
        }
    }

    /// TODO: a lot
    fn parse_assembly(assembly: &str, cpu: &impl ISA) -> Result<Self, RError> {
        let assembly = assembly.replace([',', '(', ')'], " ");

        let tokens = assembly.split_whitespace().collect::<Vec<_>>();
        return match tokens.len() {
            1 => {
                if tokens[0].to_lowercase() == "ecall" {
                    return Ok(Instruction::CSRType(0, (0, 0), 0, 0, 0b1110011));
                }
                if tokens[0].to_lowercase() == "ebreak" {
                    return Ok(Instruction::CSRType(0, (0, 0), 0, 0, 0b1110011));
                }
                Err(RError::InvalidAssembly(assembly))
            }
            3 => {
                let rd = cpu
                    .name_to_index(tokens[1])
                    .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                match parse_str(tokens[2]) {
                    Ok(imm) => {
                        if tokens[0].to_lowercase().starts_with("lui") {
                            Ok(Instruction::UType(imm << 12, rd, 0b0110111))
                        } else if tokens[0].to_lowercase().starts_with("auipc") {
                            Ok(Instruction::UType(imm, rd, 0b0010111))
                        } else if tokens[0].to_lowercase().starts_with("jal") {
                            Ok(Instruction::JType(imm, rd, 0b1101111))
                        } else {
                            Err(RError::InvalidAssembly(assembly))
                        }
                    }
                    _ => Err(RError::InvalidAssembly(assembly.clone())),
                }
            }
            4 => {
                return match tokens[0].to_lowercase().as_str() {
                    "beq" | "bne" | "blt" | "bge" | "bltu" | "bgeu" => {
                        let rs1 = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let rs2 = cpu
                            .name_to_index(tokens[2])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let imm = parse_str(tokens[3])
                            .map_err(|_| RError::InvalidAssembly(assembly.clone()))?;
                        let funct3 = match tokens[0].to_lowercase().as_str() {
                            "beq" => 0b000,
                            "bne" => 0b001,
                            "blt" => 0b100,
                            "bge" => 0b101,
                            "bltu" => 0b110,
                            "bgeu" => 0b111,
                            _ => Err(RError::InvalidAssembly(assembly.clone()))?,
                        };
                        Ok(Instruction::BType(imm, (rs1, rs2), funct3, 0b1100011))
                    }
                    "lb" | "lh" | "lw" | "lbu" | "lhu" => {
                        let rd = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let imm = parse_str(tokens[2])
                            .map_err(|_| RError::InvalidAssembly(assembly.clone()))?;
                        let rs1 = cpu
                            .name_to_index(tokens[3])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let funct3 = match tokens[0].to_lowercase().as_str() {
                            "lb" => 0b000,
                            "lh" => 0b001,
                            "lw" => 0b010,
                            "lbu" => 0b100,
                            "lhu" => 0b101,
                            _ => Err(RError::InvalidAssembly(assembly.clone()))?,
                        };
                        Ok(Instruction::IType(imm, (rs1, 0), funct3, rd, 0b0000011))
                    }
                    "sb" | "sh" | "sw" => {
                        let imm = parse_str(tokens[2])
                            .map_err(|_| RError::InvalidAssembly(assembly.clone()))?;
                        let rs1 = cpu
                            .name_to_index(tokens[3])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let rs2 = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let funct3 = match tokens[0].to_lowercase().as_str() {
                            "sb" => 0b000,
                            "sh" => 0b001,
                            "sw" => 0b010,
                            _ => Err(RError::InvalidAssembly(assembly.clone()))?,
                        };
                        Ok(Instruction::SType(imm, (rs1, rs2), funct3, 0b0100011))
                    }
                    "addi" | "slti" | "sltiu" | "xori" | "ori" | "andi" | "slli" | "srli"
                    | "srai" => {
                        let rd = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let rs1 = cpu
                            .name_to_index(tokens[2])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let imm = parse_str(tokens[3])
                            .map_err(|_| RError::InvalidAssembly(assembly.clone()))?;
                        let funct3 = match tokens[0].to_lowercase().as_str() {
                            "addi" => 0b000,
                            "slti" => 0b010,
                            "sltiu" => 0b011,
                            "xori" => 0b100,
                            "ori" => 0b110,
                            "andi" => 0b111,
                            "slli" => 0b001,
                            "srli" => 0b101,
                            "srai" => 0b101,
                            _ => Err(RError::InvalidAssembly(assembly.clone()))?,
                        };
                        Ok(Instruction::IType(imm, (rs1, 0), funct3, rd, 0b0010011))
                    }
                    "add" | "sub" | "sll" | "slt" | "sltu" | "xor" | "srl" | "sra" | "or"
                    | "and" => {
                        let rd = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let rs1 = cpu
                            .name_to_index(tokens[2])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let rs2 = cpu
                            .name_to_index(tokens[3])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let funct3 = match tokens[0].to_lowercase().as_str() {
                            "add" => 0b000,
                            "sll" => 0b001,
                            "slt" => 0b010,
                            "sltu" => 0b011,
                            "xor" => 0b100,
                            "srl" => 0b101,
                            "or" => 0b110,
                            "and" => 0b111,
                            _ => Err(RError::InvalidAssembly(assembly.clone()))?,
                        };
                        let funct7 = match tokens[0].to_lowercase().as_str() {
                            "add" => 0b0000000,
                            "sub" => 0b0100000,
                            "sll" => 0b0000000,
                            "srl" => 0b0000000,
                            "sra" => 0b0100000,
                            _ => 0b0000000,
                        };
                        Ok(Instruction::RType(
                            funct7,
                            (rs1, rs2),
                            funct3,
                            rd,
                            0b0110011,
                        ))
                    }
                    "csrrw" | "csrrs" | "csrrc" | "csrrwi" | "csrrsi" | "csrrci" => {
                        let rd = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let csr = cpu
                            .name_to_index(tokens[2])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let imm = parse_str(tokens[3])
                            .map_err(|_| RError::InvalidAssembly(assembly.clone()))?;
                        let funct3 = match tokens[0].to_lowercase().as_str() {
                            "csrrw" => 0b001,
                            "csrrs" => 0b010,
                            "csrrc" => 0b011,
                            "csrrwi" => 0b101,
                            "csrrsi" => 0b110,
                            "csrrci" => 0b111,
                            _ => Err(RError::InvalidAssembly(assembly.clone()))?,
                        };
                        Ok(Instruction::IType(imm, (csr, 0), funct3, rd, 0b1110011))
                    }
                    "jalr" => {
                        let rd = cpu
                            .name_to_index(tokens[1])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let rs1 = cpu
                            .name_to_index(tokens[3])
                            .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                        let imm = parse_str(tokens[2])
                            .map_err(|_| RError::InvalidAssembly(assembly.clone()))?;
                        Ok(Instruction::IType(imm, (rs1, 0), 0b000, rd, 0b1100111))
                    }
                    _ => Err(RError::InvalidAssembly(assembly.clone())),
                }
            }
            _ => Err(RError::InvalidAssembly(assembly)),
        };
    }

    fn execute(&self, cpu: &mut impl ISA) -> Result<u32, RError> {
        match self {
            Instruction::RType(funct7, (rs1, rs2), funct3, rd, _) => {
                let rs1 = cpu[*rs1];
                let rs2 = cpu[*rs2];
                let result = match funct3 {
                    0b000 => {
                        match funct7 {
                            0b0000000 => rs1.wrapping_add(rs2), // add
                            0b0100000 => rs1.wrapping_sub(rs2), // sub
                            _ => return Err(RError::InvalidCode(self.assemble())),
                        }
                    }
                    0b001 => rs1.wrapping_shl(rs2), // sll
                    0b101 => {
                        match funct7 {
                            0b0000000 => rs1.wrapping_shr(rs2),                 // srl
                            0b0100000 => (rs1 as i32).wrapping_shr(rs2) as u32, // sra
                            _ => return Err(RError::InvalidCode(self.assemble())),
                        }
                    }
                    0b110 => rs1 | rs2,                             // or
                    0b111 => rs1 & rs2,                             // and
                    0b100 => rs1 ^ rs2,                             // xor
                    0b010 => (rs1 as i32).lt(&(rs2 as i32)) as u32, // slt
                    0b011 => rs1.lt(&rs2) as u32,                   // sltu
                    _ => return Err(RError::InvalidCode(self.assemble())),
                };
                cpu[*rd] = result;
                Ok(cpu.pc() + 4)
            }
            Instruction::IType(imm, (rs1, _), funct3, rd, opcode) => {
                match opcode {
                    0b1100111 => {
                        // jalr
                        let rs1 = cpu[*rs1];
                        let result = rs1.wrapping_add(*imm);
                        cpu[*rd] = cpu.pc() + 4;
                        Ok(result)
                    }
                    0b0010011 => {
                        let rs1 = cpu[*rs1];
                        let result = match funct3 {
                            0b000 => rs1.wrapping_add(*imm), // addi
                            0b001 => rs1.wrapping_shl(*imm), // slli
                            0b101 => {
                                match get(*imm, 10, 10) {
                                    0 => rs1.wrapping_shr(get(*imm, 5, 0)), // srli
                                    1 => {
                                        let imm = get(*imm, 5, 0);
                                        (rs1 as i32).wrapping_shr(imm) as u32 // srai
                                    }
                                    _ => panic!("Invalid funct7"),
                                }
                            }
                            0b100 => rs1 ^ *imm, // xori
                            0b111 => rs1 & *imm, // andi
                            0b110 => rs1 | *imm, // ori
                            0b010 => (rs1 as i32).lt(&(*imm as i32)) as u32, // slti
                            0b011 => rs1.lt(imm) as u32, // sltiu
                            _ => panic!("Invalid funct3"),
                        };
                        cpu[*rd] = result;
                        Ok(cpu.pc() + 4)
                    }
                    0b0000011 => {
                        let rs1 = cpu[*rs1];
                        let addr = rs1.wrapping_add(*imm);
                        let mut result = match funct3 {
                            0b000 => cpu.load_mem(addr, 1), // lb
                            0b001 => cpu.load_mem(addr, 2), // lh
                            0b010 => cpu.load_mem(addr, 4), // lw
                            0b100 => cpu.load_mem(addr, 1), // lbu
                            0b101 => cpu.load_mem(addr, 2), // lhu
                            _ => panic!("Invalid funct3"),
                        }
                        .ok_or(RError::InvalidMem(addr))?;
                        match funct3 {
                            0b100 | 0b101 | 0b010 => (),
                            0b000 => result = result as i8 as u32,
                            0b001 => result = result as i16 as u32,
                            _ => unreachable!(),
                        }
                        cpu[*rd] = result;
                        Ok(cpu.pc() + 4)
                    }
                    _ => panic!("Invalid opcode in IType {}", self),
                }
            }
            Instruction::SType(imm, (rs1, rs2), funct3, _) => {
                let rs1 = cpu[*rs1];
                let rs2 = cpu[*rs2];
                let addr = rs1.wrapping_add(*imm);
                match funct3 {
                    0b000 => cpu.store_mem(addr, 1, rs2), // sb
                    0b001 => cpu.store_mem(addr, 2, rs2), // sh
                    0b010 => cpu.store_mem(addr, 4, rs2), // sw
                    _ => panic!("Invalid funct3"),
                }
                Ok(cpu.pc() + 4)
            }
            Instruction::BType(imm, (rs1, rs2), funct3, _) => {
                let rs1 = cpu[*rs1];
                let rs2 = cpu[*rs2];
                let result = match funct3 {
                    0b000 => rs1.eq(&rs2),                   // beq
                    0b001 => rs1.ne(&rs2),                   // bne
                    0b100 => (rs1 as i32).lt(&(rs2 as i32)), // blt
                    0b101 => (rs1 as i32).ge(&(rs2 as i32)), // bge
                    0b110 => rs1.lt(&rs2),                   // bltu
                    0b111 => rs1.ge(&rs2),                   // bgeu
                    _ => panic!("Invalid funct3"),
                };
                if result {
                    Ok(cpu.pc().wrapping_add(*imm))
                } else {
                    Ok(cpu.pc() + 4)
                }
            }
            Instruction::UType(imm, rd, opcode) => {
                match opcode {
                    0b0110111 => cpu[*rd] = *imm, // lui
                    0b0010111 => cpu[*rd] = imm.wrapping_add(cpu.pc()), // auipc
                    _ => panic!("Invalid opcode"),
                }
                Ok(cpu.pc() + 4)
            }
            Instruction::JType(imm, rd, _) => {
                // jal
                cpu[*rd] = cpu.pc() + 4;
                Ok(cpu.pc().wrapping_add(*imm))
            }
            Instruction::Nop => Ok(cpu.pc() + 4),
            Instruction::CSRType(csr, (rs1, _), funct3, rd, _) => {
                let assemble = self.assemble();
                if assemble == 0b0000_0000_0000_0000_0000_0000_0111_0011 { // ecall
                    cpu.write_register_by_name("mepc", cpu.pc());
                    cpu.write_register_by_name("mcause", 0);
                    return Ok(cpu.read_register_by_name("mtvec").unwrap())
                } else if assemble == 0b0000_0000_0001_0000_0000_0000_0111_0011 { // ebreak
                    return Err(RError::Ebreak(
                        cpu.read_register_by_name("a0").unwrap() as i8
                    ));
                } else if assemble == 0b0001_0000_0010_0000_0000_0000_0111_0011 {
                    // sret
                    let return_value = cpu.read_register_by_name("sepc").unwrap();
                    return Ok(return_value);
                } else if assemble == 0b0011_0000_0010_0000_0000_0000_0111_0011 {
                    // mret
                    let return_value = cpu.read_register_by_name("mepc").unwrap();
                    return Ok(return_value);
                }
                let _rs1 = cpu[*rs1];
                match *funct3 {
                    0b001 => {
                        // csrrw R[rd]=CSR; CSR=R[rs1]
                        let csr_value = cpu.read_register_previlege(*csr).unwrap();
                        let src = cpu[*rs1];
                        cpu[*rd] = csr_value;
                        cpu.write_register_previlege(*csr, src);
                    }
                    0b010 => {
                        // csrrs R[rd]=CSR; CSR=CSR|R[rs1]
                        let csr_value = cpu.read_register_previlege(*csr).unwrap();
                        let src = cpu[*rs1];
                        cpu[*rd] = csr_value; 
                        cpu.write_register_previlege(*csr, csr_value | src);
                    }
                    0b011 => {
                        // csrrc R[rd]=CSR; CSR=CSR&~R[rs1]
                        let csr_value = cpu.read_register_previlege(*csr).unwrap();
                        let src = cpu[*rs1];
                        cpu[*rd] = csr_value;
                        cpu.write_register_previlege(*csr, csr_value & !src);
                    }
                    0b101 => {
                        // csrrwi R[rd]=CSR; CSR= uimm
                        let csr_value = cpu.read_register_previlege(*csr).unwrap();
                        cpu[*rd] = csr_value;
                        cpu.write_register_previlege(*csr, *rs1);
                    }
                    0b110 => {
                        // csrrsi R[rd]=CSR; CSR=CSR | uimm
                        let csr_value = cpu.read_register_previlege(*csr).unwrap();
                        cpu[*rd] = csr_value;
                        cpu.write_register_previlege(*csr, csr_value | *rs1);
                    }
                    0b111 => {
                        // csrrci R[rd]=CSR; CSR=CSR& ~uimm
                        let csr_value = cpu.read_register_previlege(*csr).unwrap();
                        cpu[*rd] = csr_value;
                        cpu.write_register_previlege(*csr, csr_value & !*rs1);
                    }
                    _ => panic!("Invalid funct3 in CSRType"),
                }
                Ok(cpu.pc() + 4)
            }
        }
    }

    fn decode(machine_code: u32) -> Result<Self, RError>
    where
        Self: Sized,
    {
        let opcode = opcode(machine_code);
        match opcode {
            0b011_0011 => {
                // R-Type
                let funct7 = fun7(machine_code);
                let src = src(machine_code);
                let funct3 = fun3(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::RType(funct7, src, funct3, dst, opcode))
            }
            0b001_0011 | 0b000_0011 | 0b1100111 => {
                // I-Type
                let imm = imm(machine_code);
                let src = src(machine_code);
                let funct3 = fun3(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::IType(imm, src, funct3, dst, opcode))
            }
            0b111_0011 => {
                // Csr-Type, ecall, ebreak
                let csr = csr(machine_code);
                let src = src(machine_code);
                let funct3 = fun3(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::CSRType(csr, src, funct3, dst, opcode))
            }
            0b010_0011 => {
                // S-Type
                let imm = imm(machine_code);
                let src = src(machine_code);
                let funct3 = fun3(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::SType(imm, src, funct3, dst))
            }
            0b110_0011 => {
                // B-Type
                let imm = imm(machine_code);
                let src = src(machine_code);
                let funct3 = fun3(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::BType(imm, src, funct3, dst))
            }
            0b110_1111 => {
                // J-Type
                let imm = imm(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::JType(imm, dst, opcode))
            }
            0b011_0111 | 0b001_0111 => {
                // U-Type
                let imm = imm(machine_code);
                let dst = dst(machine_code);
                Ok(Instruction::UType(imm, dst, opcode))
            }
            0b000_1111 => {
                // FENCE, PAUSE considered as NOP
                Ok(Instruction::Nop)
            }
            _ => Err(RError::InvalidCode(machine_code)),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::isas::{reg::RegisterModel, riscv::cpu};

    #[test]
    fn test_add_sub() {
        let mut cpu = cpu::RiscvCPU::default();
        cpu[1] = 2;
        cpu[2] = 1;
        let add = Instruction::RType(0, (1, 2), 0, 3, 0b0110011); // add x3, x1, x2
        assert_eq!(add.to_string(), "add gp, ra, sp");
        add.execute(&mut cpu).unwrap();
        assert_eq!(cpu[3], 3);

        cpu[1] = 0xffffffff;
        cpu[2] = 0x2;
        add.execute(&mut cpu).unwrap();
        assert_eq!(cpu[3], 1);

        cpu[1] = 0xffffffff;
        cpu[2] = 0xffffffff;
        add.execute(&mut cpu).unwrap();
        assert_eq!(cpu[3], 0xfffffffe);

        let code = 0b01000001111100011000001110110011;
        let rtype = Instruction::decode(code).unwrap();
        assert_eq!(code, rtype.assemble());
    }

    #[test]
    fn test_and_or() {
        let mut cpu = cpu::RiscvCPU::default();
        // x17 = 0x55551111 and x18 = 0xff00ff00 then the instruction and
        // will set x12 to the value 0x55001100.
        cpu[17] = 0x55551111;
        cpu[18] = 0xff00ff00;
        let and = Instruction::RType(0, (17, 18), 0b111, 12, 0b0110011); // and x12, x17, x18
        assert_eq!(and.to_string(), "and a2, a7, s2");
        and.execute(&mut cpu).unwrap();
        cpu[12] = 0x55551111 & 0xff00ff00;

        // x17 = 0x55551111 and x18 = 0xff00ff00 then the instruction or
        // will set x12 to the value 0xff55ff11.
        cpu[17] = 0x55551111;
        cpu[18] = 0xff00ff00;
        let or = Instruction::RType(0, (17, 18), 0b110, 12, 0b0110011); // or x12, x17, x18
        or.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0xff55ff11);
    }

    #[test]
    fn test_sll() {
        let mut cpu = cpu::RiscvCPU::default();

        // x17 = 12345678 and x18 = 0x08 sll
        // set x12 0x34567800
        cpu[17] = 0x12345678;
        cpu[18] = 0x08;
        let sll = Instruction::RType(0, (17, 18), 0b001, 12, 0b0110011); // sll x12, x17, x18
        sll.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0x34567800);
    }

    #[test]
    fn test_xori() {
        let mut cpu = cpu::RiscvCPU::default();

        // x17 = 0x55551111 ， then xori x12,x17,0x800 will set x12 0xaaaae911.
        cpu[17] = 0x55551111;
        let xori = Instruction::IType(0xfffff800, (17, 0), 0b100, 12, 0b0010011); // xori x12,x17,0x800
        assert_eq!(
            cpu[17] ^ 0xfffff800_u32,
            0xaaaae911_u32
        );
        xori.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0xaaaae911);

        let code = 0x00ff10b7; //  lui     ra,0xff1
        cpu.execute(code).unwrap();
        let code = 0xf0008093; //  add     ra,ra,-256
        cpu.execute(code).unwrap();
        let code = 0xf0f0c713; // xori     a4,ra,-241
        cpu.execute(code).unwrap();
        assert_eq!(cpu.read_register_by_name("a4").unwrap(), 0xff00f00f);
    }

    #[test]
    fn test_sltu_slt() {
        let mut cpu = cpu::RiscvCPU::default();

        // if x17 = 0x12345678 and x18 = 0x0000ffff then the instruction sltu x12,x17,x18 will set x12 to the value 0x00000000.
        //If x17 = 0x12345678 and x18 = 0x8000ffff then the instruction sltu x12,x17,x18 will set x12 to the value 0x00000001.
        cpu[17] = 0x12345678;
        cpu[18] = 0x0000ffff;
        let sltu = Instruction::RType(0, (17, 18), 0b011, 12, 0b0110011); // sltu x12, x17, x18
        assert_eq!(sltu.to_string(), "sltu a2, a7, s2");
        sltu.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0x00000000);

        cpu[17] = 0x12345678;
        cpu[18] = 0x8000ffff;
        sltu.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0x00000001);


        // If x17 = 0x82345678 and x18 = 0x0000ffff then the instruction slt x12,x17,x18 will set
        // x12 to the value 0x00000001.
        cpu[17] = 0x82345678;
        cpu[18] = 0x0000ffff;
        let slt = Instruction::RType(0, (17, 18), 0b010, 12, 0b0110011); // slt x12, x17, x18
        assert_eq!(slt.to_string(), "slt a2, a7, s2");
        slt.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0x00000001);

        // if x17 = 0x12345678 and x18 = 0x0000ffff then the instruction slt x12,x17,x18
        // will set x12 to the value 0x00000000.
        cpu[17] = 0x12345678;
        cpu[18] = 0x0000ffff;
        slt.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0x00000000);
    }

    #[test]
    fn test_srli_srai() {
        let mut cpu = cpu::RiscvCPU::default();

        // if x17 = 0x87654321 then the instruction srli x12,x17,4 will set x12 to the value 0x08765432.
        cpu[17] = 0x87654321;
        let srli = Instruction::IType(4, (17, 0), 0b101, 12, 0b0010011); // srli x12, x17, 4
        assert_eq!(srli.to_string(), "srli a2, a7, 4");
        srli.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0x08765432);

        // if x17 = 0x87654321 then the instruction srai x12,x17,4 will set x12 to the value 0xf8765432.
        cpu[17] = 0x87654321;
        let srai = Instruction::IType((1 << 10) + 4, (17, 0), 0b101, 12, 0b0010011); // srai x12, x17, 4
        assert_eq!(srai.to_string(), "srai a2, a7, 4");
        srai.execute(&mut cpu).unwrap();
        assert_eq!(cpu[12], 0xf8765432);
    }

    #[test]
    fn test_parse_assembly() {
        let cpu = cpu::RiscvCPU::default();
        let add = Instruction::RType(0, (1, 2), 0b000, 3, 0b0110011);
        assert_eq!(
            Instruction::parse_assembly("add x3, x1, x2", &cpu).unwrap(),
            add
        );
        let addi = Instruction::IType(10, (1, 0), 0b000, 2, 0b0010011);
        assert_eq!(
            Instruction::parse_assembly("addi x2, x1, 10", &cpu).unwrap(),
            addi
        );
        assert_eq!(addi.to_string(), "addi sp, ra, 0xa");
        let lui = Instruction::UType(0x12345000, 1, 0b0110111);
        assert_eq!(
            Instruction::parse_assembly("lui x1, 0x12345", &cpu).unwrap(),
            lui
        );
        assert_eq!(lui.to_string(), "lui ra, 0x12345");
        let auipc = Instruction::UType(0x12345, 1, 0b0010111);
        assert_eq!(
            Instruction::parse_assembly("auipc ra, 0x12345", &cpu).unwrap(),
            auipc
        );
        let jal = Instruction::JType(0x12345, 1, 0b1101111);
        assert_eq!(
            Instruction::parse_assembly("jal ra, 0x12345", &cpu).unwrap(),
            jal
        );
        let jalr = Instruction::IType(0x12345, (1, 0), 0b000, 2, 0b1100111);
        assert_eq!(
            Instruction::parse_assembly("jalr sp, 0x12345(x1)", &cpu).unwrap(),
            jalr
        );
        let addi = Instruction::IType(0x123, (1, 0), 0b000, 2, 0b0010011);
        assert_eq!(
            Instruction::parse_assembly("addi sp, ra, 0x123", &cpu).unwrap(),
            addi
        );
        let beq = Instruction::BType(0x12345, (1, 2), 0b000, 0b1100011);
        assert_eq!(
            Instruction::parse_assembly("beq ra, sp, 0x12345", &cpu).unwrap(),
            beq
        );
    }

    #[test]
    fn test_addi() {
        let code: u32 = 0x30352073; // csrs mideleg, a0
                                    // print bit vector
                                    // 0011_0000_0011_01010_010_00000_1110011
        let c = Instruction::decode(code).unwrap();
        assert_eq!(c.assemble(), code);
        let code = 0xfff08093; // csrrs a0, mstatus, a0
        let mut cpu = cpu::RiscvCPU::default();
        cpu.write_register_by_name("ra", 0x80000000);
        cpu.execute(code).unwrap();
        assert_eq!(cpu.read_register_by_name("ra").unwrap(), 0x7fffffff);
    }

    #[test]
    fn test_bne() {
        let code: u32 = 0xfe5214e3; // bne tp,t0,80000370
        let _c = Instruction::decode(code).unwrap();
        let mut cpu = cpu::RiscvCPU::default();
        cpu.update_pc(0x80000388);
        let new_pc = cpu.execute(code).unwrap();
        assert_eq!(new_pc, 0x8000038c);

        cpu.write_register_by_name("t0", 1);
        let new_pc = cpu.execute(code).unwrap();
        assert_eq!(new_pc, 0x80000370);
    }

    #[test]
    fn test_csr() {
        let code: u32 = 0x30352073; // csrs mideleg, a0
                                    // print bit vector
                                    // 0011_0000_0011_01010_010_00000_1110011
        let c = Instruction::decode(code).unwrap();
        assert_eq!(c.assemble(), code);
        let mut cpu = cpu::RiscvCPU::default();
        cpu.write_register_by_name("ra", 0x80000000);
        cpu.execute(code).unwrap();
        assert_eq!(cpu.read_register_by_name("ra").unwrap(), 0x80000000);
    }
}
