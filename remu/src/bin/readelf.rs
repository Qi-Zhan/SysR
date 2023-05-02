use std::process::exit;

use remu::exes::{elf, exe::Exe};

/// my implementation of readelf to test the elf parser
/// it **only** supports the following options:
/// -h: print the header
/// -l : print the program header
/// -S : print the section header

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 3 {
        let help = "Usage: readelf <option> <file>
Options:
    -h: print the header
    -l : print the program header
    -S : print the section header
    -s : print the symbol table";
        eprintln!("{}", help);
        exit(1);
    }
    let option = args[1].as_str();
    let file = &args[2];
    match elf::ELF::parse_path(file) {
        Ok(elf) => {
            match option {
                "-h" => elf.show_header(),
                "-l" => elf.show_program_headers(),
                "-S" => elf.show_section_headers(),
                "-s" => elf.show_symbol_table(),
                _ => {
                    eprintln!("Invalid option");
                    exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }
}
