/// The test is based on the RISC-V Foundation's RISC-V Compliance Test Suite.
/// https://github.com/riscv-software-src/riscv-tests

// rv32ui	RV32 user-level,        integer only
// rv32si	RV32 supervisor-level,  integer only
// rv64ui	RV64 user-level,        integer only
// rv64uf	RV64 user-level,        integer and floating-point
// rv64uv	RV64 user-level,        integer, floating-point, and vector
// rv64si	RV64 supervisor-level,  integer only
// rv64sv   RV64 supervisor-level,  integer and vector


use remu::{isas::{riscv::cpu::RiscvCPU, ISA, RegisterModel}, exes::{elf::ELF, Exe}, error::RError};

const RISCV_TEST_DIR: &str = "/Users/zhanqi/project/riscv-tests/isa";


#[test]
fn riscv_test() {

    if !std::path::Path::new(RISCV_TEST_DIR).exists() {
        eprintln!("RISCV_TEST_DIR does not exist, Please set the RISCV_TEST_DIR environment in riscv-test.rs");
        return;
    }
    // list all executable file in RISCV_TEST_DIR
    let mut test_files = vec![];
    for entry in std::fs::read_dir(RISCV_TEST_DIR).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if !file_name.ends_with(".dump") {
                test_files.push(file_name.to_string());
            }
        }
    }    

    let mut rv32ui = vec![];
    let mut rv32si = vec![];
    let mut rv64ui = vec![];
    let mut rv64uf = vec![];
    let mut rv64uv = vec![];
    let mut rv64si = vec![];
    let mut rv64sv = vec![];

    for file in test_files {
        if file.starts_with("rv32ui") {
            rv32ui.push(file);
        } else if file.starts_with("rv32si") {
            rv32si.push(file);
        } else if file.starts_with("rv64ui") {
            rv64ui.push(file);
        } else if file.starts_with("rv64uf") {
            rv64uf.push(file);
        } else if file.starts_with("rv64uv") {
            rv64uv.push(file);
        } else if file.starts_with("rv64si") {
            rv64si.push(file);
        } else if file.starts_with("rv64sv") {
            rv64sv.push(file);
        }
    }

    for file in rv32ui {
        if file.contains("-v") {
            continue;
        }
        test_one(file.as_str());
    }

}

fn test_one(file: &str) {
    println!("test: {}", file);
    return;
    
    let path = format!("{}/{}", RISCV_TEST_DIR, file);
    let mut cpu = RiscvCPU::default();
    let mut exe = ELF::parse_path(path.as_str()).unwrap();
    let pass = exe.find_symbol("pass");
    let fail = exe.find_symbol("fail");
    exe.load_binary(&mut cpu).unwrap();
    loop {
        let pc = cpu.pc();
        if let Some(pass) = pass {
            if pc == pass as u32 {
                println!("Passed");
                return;
            }
        }
        if let Some(fail) = fail {
            if pc == fail as u32 {
                panic!("Failed")
            }
        }
        print!("pc: {:x} ", pc);
        let assembly = cpu.disassemble(pc).unwrap();
        println!("{}", assembly);
        match cpu.step() {
            Ok(_) => (),
            Err(RError::Ecall) => {
                if cpu.read_register_by_name("a7").unwrap() == 93 {
                    let gp = cpu.read_register_by_name("gp").unwrap();
                    assert_eq!(gp, 1);
                    println!("Passed");
                    break;
                }
            }
            Err(e) => {
                panic!("{:?}", e);
            }
        }
    }
}