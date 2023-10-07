use remu::exes::{Exe, ELF};
use remu::isas::{ISA, RV32CPU};
use remu::rdb::Debugger;
use remu::{info, warn};
use std::process::exit;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut debugger = Debugger::new();
    let mut cpu = RV32CPU::default();
    let mut exe = {
        if args.len() > 1 {
            ELF::parse_path(&args[1]).unwrap()
        } else {
            println!("Usage: {} <elf>", args[0]);
            std::process::exit(1);
        }
    };
    exe.load_binary(&mut cpu).unwrap();
    if args.len() == 2 {
        if let Err(e) = cpu.run() {
            match e {
                remu::error::RError::Ebreak(exitcode) => {
                    info!("Program exited with code {}", exitcode);
                    exit(0);
                }
                _ => {
                    warn!("Program exited with error: {:?}", e);
                    exit(1);
                }
            }
        }
    } else {
        debugger.debug(&mut cpu);
    }
}
