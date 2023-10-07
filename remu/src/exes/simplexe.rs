use std::collections::HashMap;

use crate::{
    error::RError,
    isas::{riscv::instruction::Instruction, Inst, ISA},
    util::parse_str,
};

use super::Exe;

/// Simple Exe is basicly executable file consists of assembly code
/// which is suitable for run code compiled from our own compiler
/// and we can ignore some official rule to simpilify
#[derive(Debug)]
pub struct SimpleExe {
    /// entry address
    entry: u32,
    /// instructions
    pub asm: Vec<String>,
    labels: HashMap<String, u32>,
    /// holes to be filled,
    pub holes: Vec<(u32, String)>,
}

impl Default for SimpleExe {
    fn default() -> Self {
        SimpleExe {
            entry: 0x80000000,
            asm: Vec::new(),
            labels: HashMap::new(),
            holes: Vec::new(),
        }
    }
}

impl Exe for SimpleExe {
    /// currently only support riscv32
    fn parse(input: &[u8]) -> Result<Self, RError> {
        let input = std::str::from_utf8(input)
            .map_err(|_| RError::IOError("input is not utf8".to_string()))?;
        let lines = input
            .split('\n')
            .map(|s| {
                if let Some(pos) = s.find('#') {
                    s[..pos].to_string()
                } else {
                    s.to_string()
                }
            })
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        Ok(SimpleExe {
            entry: 0x80000000,
            asm: lines,
            labels: HashMap::new(),
            holes: Vec::new(),
        })
    }

    fn load_binary(&mut self, cpu: &mut impl ISA) -> Result<(), RError> {
        assert_eq!(cpu.name(), "RISC-V 32");
        let mut current = self.entry;
        for line in self.asm.iter() {
            // parse label
            if line.ends_with(':') {
                self.labels
                    .insert(line.trim_end_matches(':').to_string(), current);
            }
            // parse directive
            else if line.starts_with('.') {
                self.parse_direction(line, cpu)?;
            }
            // parse instruction
            else {
                let inst = self.parse_assembly(line, cpu)?;
                cpu.store_mems(current, &inst);
                current += 4 * inst.len() as u32;
            }
        }
        // choose entry address
        if let Some(entry) = self.labels.get("_start") {
            self.entry = *entry;
        } else if let Some(entry) = self.labels.get("main") {
            self.entry = *entry;
        } else {
            return Err(RError::Other("no entry point".to_string()));
        }
        Ok(())
    }
}

impl SimpleExe {
    /// parse assembly code to binary codes
    fn parse_assembly(&self, assembly: &str, cpu: &impl ISA) -> Result<Vec<u32>, RError> {
        let assembly = assembly.replace([',', '(', ')'], " ");

        let tokens = assembly.split_whitespace().collect::<Vec<_>>();
        return match tokens.len() {
            1 => {
                if tokens[0].to_lowercase() == "ecall" {
                    return Ok(vec![
                        Instruction::CSRType(0, (0, 0), 0, 0, 0b1110011).assemble()
                    ]);
                }
                if tokens[0].to_lowercase() == "ebreak" {
                    return Ok(vec![
                        Instruction::CSRType(0, (0, 0), 0, 0, 0b1110011).assemble()
                    ]);
                }
                Err(RError::InvalidAssembly(assembly))
            }
            3 => {
                let rd = cpu
                    .name_to_index(tokens[1])
                    .ok_or(RError::InvalidAssembly(assembly.clone()))?;
                match parse_str(tokens[2]) {
                    Ok(imm) => match tokens[0] {
                        "lui" => Ok(vec![Instruction::UType(imm << 12, rd, 0b0110111).assemble()]),
                        "auipc" => Ok(vec![Instruction::UType(imm, rd, 0b0010111).assemble()]),
                        "jal" => Ok(vec![Instruction::JType(imm, rd, 0b1101111).assemble()]),
                        "li" => {
                            todo!("li")
                        }
                        _ => Err(RError::InvalidAssembly(assembly.clone())),
                    },
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
                        Ok(vec![
                            Instruction::BType(imm, (rs1, rs2), funct3, 0b1100011).assemble()
                        ])
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
                        Ok(vec![Instruction::IType(
                            imm,
                            (rs1, 0),
                            funct3,
                            rd,
                            0b0000011,
                        )
                        .assemble()])
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
                        Ok(vec![
                            Instruction::SType(imm, (rs1, rs2), funct3, 0b0100011).assemble()
                        ])
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
                        Ok(vec![Instruction::IType(
                            imm,
                            (rs1, 0),
                            funct3,
                            rd,
                            0b0010011,
                        )
                        .assemble()])
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
                        Ok(vec![Instruction::RType(
                            funct7,
                            (rs1, rs2),
                            funct3,
                            rd,
                            0b0110011,
                        )
                        .assemble()])
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
                        Ok(vec![Instruction::IType(
                            imm,
                            (csr, 0),
                            funct3,
                            rd,
                            0b1110011,
                        )
                        .assemble()])
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
                        Ok(vec![Instruction::IType(
                            imm,
                            (rs1, 0),
                            0b000,
                            rd,
                            0b1100111,
                        )
                        .assemble()])
                    }
                    _ => Err(RError::InvalidAssembly(assembly.clone())),
                }
            }
            _ => Err(RError::InvalidAssembly(assembly)),
        };
    }

    fn parse_direction(&self, direction: &str, _cpu: &impl ISA) -> Result<(), RError> {
        let mut tokens = direction.split_whitespace();
        let directive = tokens.next().unwrap();
        match directive {
            ".word" => {}
            ".byte" => {}
            ".half" => {}
            ".space" => {}
            ".ascii" => {}
            ".asciiz" => {}
            ".align" => {}
            ".data" => {}
            ".text" => {}
            _ => {
                return Err(RError::InvalidAssembly(direction.to_string()));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::isas::{riscv::RV32CPU, RegisterModel};

    use super::*;

    #[test]
    fn test_simple_exe() {
        let path = "tests/sample.S";
        let mut cpu = RV32CPU::default();
        let mut exe = SimpleExe::parse_path(path).unwrap();
        println!("{:#?}", exe.asm);
        exe.load_binary(&mut cpu).unwrap();
        assert_eq!(cpu.pc(), 0x80000000);
    }

    #[test]
    fn test_parse_assembly() {
        let cpu = RV32CPU::default();
        let add = Instruction::RType(0, (1, 2), 0b000, 3, 0b0110011);
        let exe = SimpleExe::default();
        assert_eq!(
            exe.parse_assembly("add x3, x1, x2", &cpu).unwrap()[0],
            add.assemble()
        );
        let addi = Instruction::IType(10, (1, 0), 0b000, 2, 0b0010011);
        assert_eq!(
            exe.parse_assembly("addi x2, x1, 10", &cpu).unwrap()[0],
            addi.assemble()
        );
        assert_eq!(addi.to_string(), "addi sp, ra, 0xa");
        let lui = Instruction::UType(0x12345000, 1, 0b0110111);
        assert_eq!(
            exe.parse_assembly("lui x1, 0x12345", &cpu).unwrap()[0],
            lui.assemble()
        );
        assert_eq!(lui.to_string(), "lui ra, 0x12345");
        let auipc = Instruction::UType(0x12345, 1, 0b0010111);
        assert_eq!(
            exe.parse_assembly("auipc ra, 0x12345", &cpu).unwrap()[0],
            auipc.assemble()
        );
        let jal = Instruction::JType(0x12345, 1, 0b1101111);
        assert_eq!(
            exe.parse_assembly("jal ra, 0x12345", &cpu).unwrap()[0],
            jal.assemble()
        );
        let jalr = Instruction::IType(0x12345, (1, 0), 0b000, 2, 0b1100111);
        assert_eq!(
            exe.parse_assembly("jalr sp, 0x12345(x1)", &cpu).unwrap()[0],
            jalr.assemble()
        );
        let addi = Instruction::IType(0x123, (1, 0), 0b000, 2, 0b0010011);
        assert_eq!(
            exe.parse_assembly("addi sp, ra, 0x123", &cpu).unwrap()[0],
            addi.assemble()
        );
        let beq = Instruction::BType(0x12345, (1, 2), 0b000, 0b1100011);
        assert_eq!(
            exe.parse_assembly("beq ra, sp, 0x12345", &cpu).unwrap()[0],
            beq.assemble()
        );
    }
}
