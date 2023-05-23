use remu::rdb::debugger::Debugger;
use remu::isas::riscv::RV32CPU;
use remu::exes::Exe;
use remu::exes::elf::ELF;


fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let mut debugger = Debugger::new();
    let mut cpu = RV32CPU::default();
    let mut exe = {
        if args.len() == 2 {
            ELF::parse_path(&args[1]).unwrap()
        } else {
            println!("Usage: {} <elf>", args[0]);
            std::process::exit(1);
        }
    };
    exe.load_binary(&mut cpu).unwrap();
    debugger.debug(&mut cpu);
}
