use super::exe;
use super::elformat::{EI_MAGO, EI_MAG1, EI_MAG2, EI_MAG3};
/// implementation of ELF file format, from https://en.wikipedia.org/wiki/Executable_and_Linkable_Format

macro_rules! show_header {
    ($name:ident) => {
        impl $name {
            pub fn show_header(&self) {
                println!("ELF Header: ");
                print!("  Magic:   ");
                for i in 0..16 {
                    print!("{:02x} ", self.header.ident[i]);
                }
                println!();
                println!("  Class:                             {}", EiClass::from_u8(self.header.ident[4]).unwrap());
                println!("  Data:                              {}", EiData::from_u8(self.header.ident[5]).unwrap());
                println!("  Version:                           {}", EiVersion::from_u8(self.header.ident[6]).unwrap());
                println!("  OS/ABI:                            {}", EiOsAbi::from_u8(self.header.ident[7]).unwrap());
                println!("  ABI Version:                       {}", self.header.ident[8]);
                println!("  Type:                              {}", EType::from_u16(self.header.e_type).unwrap());
                println!("  Machine:                           {}", EMachine::from_u16(self.header.machine).unwrap());
                println!("  Version:                           {}", self.header.version);
                println!("  Entry point address:               0x{:08x}", self.header.entry);
                println!("  Start of program headers:          {} (bytes into file)", self.header.phoff);
                println!("  Start of section headers:          {} (bytes into file)", self.header.shoff);
                println!("  Flags:                             0x{:08x}", self.header.flags);
                println!("  Size of this header:               {} (bytes)", self.header.ehsize);
                println!("  Size of program headers:           {} (bytes)", self.header.phentsize);
                println!("  Number of program headers:         {}", self.header.phnum);
                println!("  Size of section headers:           {} (bytes)", self.header.shentsize);
                println!("  Number of section headers:         {}", self.header.shnum);
                println!("  Section header string table index: {}", self.header.shstrndx);            
            }
        }
    }
}

macro_rules! show_program_headers {
    ($name: ident) => {
        impl $name {
            pub fn show_program_headers(&self) {
                println!("There are {} program headers, starting at offset {}:", self.header.phnum, self.header.phoff);
                println!("Program Headers:");
                println!("  Type           Offset   VirtAddr   PhysAddr   FileSiz MemSiz  Flg Align");
                for header in self.program_headers.iter() {
                    let p_type_string = format!("{:?}", PType::from_u32(header.p_type).unwrap());
                    print!("  {}", p_type_string);
                    if p_type_string.len() < 14 {
                        print!(" {}", &" ".repeat(14 - p_type_string.len()));
                    }
                    println!("0x{:06x} 0x{:08x} 0x{:08x} 0x{:05x} 0x{:05x} {:>20} 0x{:06x}",
                        header.offset,
                        header.vaddr,
                        header.paddr,
                        header.filesz,
                        header.memsz,
                        PFlags::from_u32(header.flags).unwrap(),
                        header.align);
                }
            }
        }
    };
}

macro_rules! show_section_headers {
    ($name: ident) => {
        impl $name {

            fn sh_name(&self, index: usize) -> String {
                let mut name = String::new();
                let mut i = self.section_headers[index].name as usize;
                let section_name_start = self.section_headers[self.header.shstrndx as usize].offset as usize;
                while self.bytes[section_name_start + i] != 0 {
                    name.push(self.bytes[section_name_start + i] as char);
                    i += 1;
                }
                name
            }

            pub fn show_section_headers(&self) {
                println!("There are {} section headers, starting at offset 0x{:x}:", self.header.shnum, self.header.shoff);
                println!("Section Headers:");
                println!("  [Nr] Name              Type            Addr     Off    Size   ES Flg Lk Inf Al");
                for i in 0..self.header.shnum {
                    println!("  [{:2}] {:<16} {:<16} {:08x} {:06x} {:06x} {:02x} {:3} {:2} {:3} {:2}",
                        i,
                        self.sh_name(i as usize),
                        SType::from_u32(self.section_headers[i as usize].sh_type).unwrap(),
                        self.section_headers[i as usize].addr,
                        self.section_headers[i as usize].offset,
                        self.section_headers[i as usize].size,
                        self.section_headers[i as usize].entsize,
                        SFlags::from_u32(self.section_headers[i as usize].flags.into()).unwrap(),
                        self.section_headers[i as usize].link,
                        self.section_headers[i as usize].info,
                        self.section_headers[i as usize].addralign);
                }
            }
        }
    };
}

macro_rules! show_symbol_table {
    ($name: ident) => {

        impl $name {

            fn symbol_name(&self, entry:&Symbol) -> String {
                let mut name = String::new(); 
                let mut i = entry.name as usize; 
                if i == 0 {
                    return name;
                }
                // let shndx = entry.shndx as usize; MAYBE A BUG FOR STRTBINDEX
                let string_table_start = self.section_headers[self.strtbindex[1]].offset as usize;
                while self.bytes[string_table_start + i] != 0 {
                    name.push(self.bytes[string_table_start + i] as char);
                    i += 1;
                }
                name
            }

            pub fn find_symbol(&self, name: &str) -> Option<u64> {
                for symbol in self.symbols.iter() {
                    if self.symbol_name(symbol) == name {
                        return Some(symbol.value as u64)
                    }
                }
                None
            }

            pub fn show_symbol_table(&self) {
                println!("Symbol table '.symtab' contains {} entries:", self.symbols.len());
                println!("   Num:    Value          Size Type    Bind   Vis      Ndx Name");
                let symbols = &self.symbols;
                for i in 0..symbols.len() {
                    let symbol = &symbols[i];
                    let st_type = STType::from_u8(symbol.info & 0xf).unwrap();
                    let st_bind = STBind::from_u8((symbol.info >> 4) & 0xf).unwrap();
                    let st_vis = STVis::from_u8((symbol.other >> 2) & 0x3).unwrap();
                    let st_shndx = symbol.shndx;
                    let (st_shndx_string, name) = if st_shndx == 0 {
                        ("UND".to_string(), "".to_string())
                    } else if st_shndx == 0xfff1 {
                        ("ABS".to_string(), "".to_string())
                    } else if st_shndx == 0xfff2 {
                        ("COM".to_string(), "".to_string())
                    } else {
                        (format!("{:3}", st_shndx), self.symbol_name(symbol))
                    };
                    println!("  {:6}: {:016x} {:6} {:<7} {:<7} {:<7} {:<4} {}",
                        i,
                        symbol.value,
                        symbol.size,
                        st_type,
                        st_bind,
                        st_vis,
                        st_shndx_string,
                        name);
                }
            }
        }
    };
}

macro_rules! elf_parse {
    ($name: ident) => {
            fn parse(input: &[u8]) -> Result<Self, RError> {
                if input.len() < 64 {
                    return Err(RError::Other("Input is too short".to_string()));
                }
                let header = ELFHeader::linearparse(&input); 
                let mut program_headers = Vec::new();
                let mut section_headers = Vec::new();
                let mut strtbindex = Vec::new();
                if header.phoff != 0 {
                    let mut index = header.phoff as usize;
                    for _ in 0..header.phnum {
                        if index + header.phentsize as usize > input.len() {
                            return Err(RError::Other("Program header is out of range".to_string()));
                        }
                        let phdr = ProgramHeader::linearparse(&input[index.. (index + header.phentsize as usize)]);
                        program_headers.push(phdr);
                        index += header.phentsize as usize;
                    }
                }
                if header.shoff != 0 {
                    let mut index = header.shoff as usize;
                    for _ in 0..header.shnum {
                        if index + header.shentsize as usize > input.len() {
                            return Err(RError::Other("Section header is out of range".to_string()));
                        }
                        let shdr = SectionHeader::linearparse(&input[index..index+header.shentsize as usize]);
                        if shdr.sh_type == SType::Strtab as u32 {
                            strtbindex.push(section_headers.len() - 1);
                        }
                        section_headers.push(shdr);
                        index += header.shentsize as usize;
                    }
                }
                let mut symbols = Vec::new();
                for i in 0..header.shnum as usize {
                    let section = &section_headers[i];
                    let entsize = section.entsize as usize;
                    if section_headers[i].sh_type == SType::Symtab as u32 {
                        let mut index = section.offset as usize;
                        for _ in 0..(section.size / section.entsize) {
                            if index + entsize > input.len() {
                                return Err(RError::Other("Symbol is out of range".to_string()));
                            }
                            let symbol = Symbol::linearparse(&input[index..index + entsize]);
                            symbols.push(symbol);
                            index += entsize
                        }
                    }
                }
                let bytes = input.to_vec();
                Ok($name {
                    header,
                    program_headers,
                    section_headers,
                    strtbindex,
                    symbols,
                    bytes,
                })
            }
    };
}

mod elf32 {
    use linearparse_derive::LinearParse;
    use crate::util::LinearParse;
    use super::super::exe::Exe;
    use super::super::elformat::*;
    use crate::isas::isa::ISA;
    use crate::error::RError;

    #[derive(Debug)]
    pub struct ELF32 {
        pub header: ELFHeader,
        pub program_headers: Vec<ProgramHeader>,
        pub section_headers: Vec<SectionHeader>,
        pub strtbindex: Vec<usize>,
        pub symbols: Vec<Symbol>,
        pub bytes: Vec<u8>,
    }

    #[derive(Debug, LinearParse)]
    pub struct ELFHeader {
        pub ident:      [u8; 16],   // Magic number and other info
        pub e_type:     u16,        // Object file type
        pub machine:    u16,        // Architecture
        pub version:    u32,        // Object file version
        pub entry:      Elf32Addr,  // Entry point virtual address
        pub phoff:      Elf32Off,   // Program header table file offset
        pub shoff:      Elf32Off,   // Section header table file offset
        pub flags:      u32,        // Processor-specific flags
        pub ehsize:     u16,        // ELF header size in bytes
        pub phentsize:  u16,        // Program header table entry size
        pub phnum:      u16,        // Program header table entry count
        pub shentsize:  u16,        // Section header table entry size
        pub shnum:      u16,        // Section header table entry count
        pub shstrndx:   u16,        // Section header string table index
    }

    #[derive(Debug, LinearParse)]
    pub struct ProgramHeader {
        pub p_type:     u32,        // Segment type
        pub offset:     Elf32Off,   // Segment file offset
        pub vaddr:      Elf32Addr,  // Segment virtual address
        pub paddr:      Elf32Addr,  // Segment physical address
        pub filesz:     u32,        // Segment size in file
        pub memsz:      u32,        // Segment size in memory
        pub flags:      u32,        // Segment flags
        pub align:      u32,        // Segment alignment
    }

    #[derive(Debug, LinearParse)]
    pub struct SectionHeader {
        pub name:        Elf32Word,  // Section name (string tbl index)
        pub sh_type:        Elf32Word,  // Section type
        pub flags:       Elf32Word,  // Section flags
        pub addr:        Elf32Addr,  // Section virtual addr at execution
        pub offset:      Elf32Off,   // Section file offset
        pub size:        Elf32Word,  // Section size in bytes
        pub link:        Elf32Word,  // Link to another section
        pub info:        Elf32Word,  // Additional section information
        pub addralign:   Elf32Word,  // Section alignment
        pub entsize:     Elf32Word,  // Entry size if section holds table
    }

    #[derive(Debug, LinearParse)]
    pub struct Symbol {
        pub name:        Elf32Word,  // Symbol name (string tbl index)
        pub value:       Elf32Addr,  // Symbol value
        pub size:        Elf32Word,  // Symbol size
        pub info:        u8,         // Symbol type and binding
        pub other:       u8,         // Symbol visibility
        pub shndx:       Elf32Half,  // Section index
    }
        
    show_header!(ELF32);
    show_program_headers!(ELF32);
    show_section_headers!(ELF32);
    show_symbol_table!(ELF32);

    impl Exe for ELF32 {
        fn load_binary(&mut self, cpu:& mut impl ISA) -> Result<(), RError> {
            for i in 0..self.header.phnum as usize {
                if self.program_headers[i].p_type == PType::Load as u32{
                    let mut index = self.program_headers[i].offset as usize;
                    let mut vaddr = self.program_headers[i].vaddr as usize;
                    for _ in 0..self.program_headers[i].filesz {
                        // if self.program_headers[i].filesz == 0x0217c {
                        //     // print 4 bytes
                        //     if index % 4 == 0 {
                        //         let value = (self.bytes[index] as u32) | ((self.bytes[index+1] as u32) << 8) | ((self.bytes[index+2] as u32) << 16) | ((self.bytes[index+3] as u32) << 24);
                        //         println!("vaddr: 0x{:x}, value: 0x{:08x}", vaddr, value);
                        //     }
                        // }
                        cpu.store_mem(vaddr as u32, 1, self.bytes[index] as u32);
                        index += 1;
                        vaddr += 1;
                    }
                }
            }
            cpu.update_pc(self.header.entry);
            Ok(())
        }

        elf_parse!(ELF32);
    }
}

mod elf64 {

    use linearparse_derive::LinearParse;
    use crate::util::LinearParse;
    use super::super::exe::Exe;
    use super::super::elformat::*;
    use crate::isas::isa::ISA;
    use crate::error::RError;

    #[derive(Debug)]
    pub struct ELF64 {
        pub header: ELFHeader,
        pub program_headers: Vec<ProgramHeader>,
        pub section_headers: Vec<SectionHeader>,
        pub strtbindex: Vec<usize>,
        pub symbols: Vec<Symbol>,
        pub bytes: Vec<u8>,
    }

    #[derive(Debug, LinearParse)]
    pub struct ELFHeader {
        pub ident: [u8; 16],
        pub e_type: u16,
        pub machine: u16,
        pub version: u32,
        pub entry: u64,
        pub phoff: u64,
        pub shoff: u64,
        pub flags: u32,
        pub ehsize: u16,
        pub phentsize: u16,
        pub phnum: u16,
        pub shentsize: u16,
        pub shnum: u16,
        pub shstrndx: u16,
    }

    #[derive(Debug, LinearParse)]
    pub struct ProgramHeader {
        pub p_type:     u32,        // Segment type
        pub flags:      u32,        // Segment flags
        pub offset:     Elf64Off,   // Segment file offset
        pub vaddr:      Elf64Addr,  // Segment virtual address
        pub paddr:      Elf64Addr,  // Segment physical address
        pub filesz:     u64,        // Segment size in file
        pub memsz:      u64,        // Segment size in memory
        pub align:      u64,        // Segment alignment
    }
    
    #[derive(Debug, LinearParse)]
    pub struct SectionHeader {
        pub name: u32,
        pub sh_type: u32,
        pub flags: u64,
        pub addr: u64,
        pub offset: u64,
        pub size: u64,
        pub link: u32,
        pub info: u32,
        pub addralign: u64,
        pub entsize: u64,
    }

    #[derive(Debug, LinearParse)]
    pub struct Symbol {
        pub name: Elf64Word,
        pub info: u8,
        pub other: u8,
        pub shndx: Elf64Half,
        pub value: Elf64Addr,
        pub size: Elf64Xword,
    }

    show_header!(ELF64);
    show_program_headers!(ELF64);
    show_section_headers!(ELF64);
    show_symbol_table!(ELF64);


    impl Exe for ELF64 {
        fn load_binary(&mut self, cpu: &mut impl ISA) -> Result<(), RError> {
            for i in 0..self.header.phnum as usize {
                if self.program_headers[i].p_type == PType::Load as u32{
                    let mut index = self.program_headers[i].offset as usize;
                    let mut vaddr = self.program_headers[i].vaddr as usize;
                    for _ in 0..self.program_headers[i].filesz {
                        cpu.store_mem(vaddr as u32, 1, self.bytes[index] as u32);
                        index += 1;
                        vaddr += 1;
                    }
                }
            }
            cpu.update_pc(self.header.entry as u32);
            Ok(())
        }

        elf_parse!(ELF64);

    }

}

macro_rules! enum_add {
    ($name:ident) => {
        impl ELF {
            pub fn $name(&self) {
                match self {
                    ELF::ELF32(elf) => elf.$name(),
                    ELF::ELF64(elf) => elf.$name(),
                };
            }
        }
    };
}

enum_add!(show_header);
enum_add!(show_program_headers);
enum_add!(show_section_headers);
enum_add!(show_symbol_table);
pub enum ELF {
    ELF32(elf32::ELF32),
    ELF64(elf64::ELF64),
}

impl ELF {
    pub fn find_symbol(&self, name: &str) -> Option<u64> {
        match self {
            ELF::ELF32(elf) => elf.find_symbol(name),
            ELF::ELF64(elf) => elf.find_symbol(name),
        }
    }
}

impl exe::Exe for ELF {

    fn parse(input: &[u8]) -> Result<Self, crate::error::RError> {
        if input.len() < 4 {
            return Err(crate::error::RError::Other("input too short".to_string()));
        }
        let magics = &input[0..4];
        if magics != [EI_MAGO, EI_MAG1, EI_MAG2, EI_MAG3] {
            return Err(crate::error::RError::Other("not a elf file".to_string()));
        }
        let elf_class = input[4];
        match elf_class {
            1 => {
                let elf = elf32::ELF32::parse(input)?;
                Ok(ELF::ELF32(elf))
            }
            2 => {
                let elf = elf64::ELF64::parse(input)?;
                Ok(ELF::ELF64(elf))
            }
            _ => Err(crate::error::RError::Other("unknown elf class".to_string())),
        }
    }

    fn load_binary(&mut self, cpu:& mut impl crate::isas::isa::ISA) -> Result<(), crate::error::RError> {
        match self {
            ELF::ELF32(elf) => elf.load_binary(cpu),
            ELF::ELF64(elf) => elf.load_binary(cpu),
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::exes::exe::Exe;

    #[test]
    fn test_parse_32_header() {
        use super::elf32::*;
        let elf = ELF32::parse_path("tests/a_32.out").unwrap();
        assert_eq!(elf.header.ident[0], 0x7f);
        assert_eq!(elf.header.ident[1], 0x45);
        assert_eq!(elf.header.ident[2], 0x4c);
        assert_eq!(elf.header.ident[3], 0x46);
        assert_eq!(elf.header.ident[4], 0x01);
        assert_eq!(elf.header.ident[5], 0x01);
        assert_eq!(elf.header.ident[6], 0x01);
        assert_eq!(elf.header.e_type, 0x02);
        assert_eq!(elf.header.machine, 0xf3);
        assert_eq!(elf.header.version, 0x01);
        assert_eq!(elf.header.entry, 0x100dc);
        assert_eq!(elf.header.phoff, 0x34);
        assert_eq!(elf.header.shoff, 23464);
        assert_eq!(elf.header.flags, 0x00);
        assert_eq!(elf.header.ehsize, 0x34);
        assert_eq!(elf.header.phentsize, 0x20);
        assert_eq!(elf.header.phnum, 0x03);
        assert_eq!(elf.header.shentsize, 0x28);
        assert_eq!(elf.header.shnum, 0x15);
        assert_eq!(elf.header.shstrndx, 0x14);
    }

    #[test]
    fn test_parse_64_header() {
        use super::elf64::*;
        let elf = ELF64::parse_path("tests/a_64.out").unwrap();
        assert_eq!(elf.header.ident[0], 0x7f);
        assert_eq!(elf.header.ident[1], 0x45);
        assert_eq!(elf.header.ident[2], 0x4c);
        assert_eq!(elf.header.ident[3], 0x46);
        assert_eq!(elf.header.ident[4], 0x02);
        assert_eq!(elf.header.ident[5], 0x01);
        assert_eq!(elf.header.ident[6], 0x01);
        assert_eq!(elf.header.e_type, 0x02);
        assert_eq!(elf.header.machine, 0xf3);
        assert_eq!(elf.header.version, 0x01);
        assert_eq!(elf.header.entry, 0x10118);
        assert_eq!(elf.header.phoff, 0x40);
        assert_eq!(elf.header.shoff, 21640);
        assert_eq!(elf.header.flags, 0x05);
        assert_eq!(elf.header.ehsize, 0x40);
        assert_eq!(elf.header.phentsize, 0x38);
        assert_eq!(elf.header.phnum, 0x03);
        assert_eq!(elf.header.shentsize, 0x40);
        assert_eq!(elf.header.shnum, 0x0f);
        assert_eq!(elf.header.shstrndx, 0x0e);

    }

    #[test]
    fn test_parse_32_program_headers() {
        use super::elf32::*;
        let elf = ELF32::parse_path("tests/a_32.out").unwrap();
        assert_eq!(elf.program_headers.len(), 3);
        let ph = &elf.program_headers[0];
        assert_eq!(ph.p_type, 0x70000003);
        assert_eq!(ph.offset, 0x003ebf);
        assert_eq!(ph.vaddr, 0x00000000);
        assert_eq!(ph.paddr, 0x00000000);
        assert_eq!(ph.filesz, 0x0001c);
        assert_eq!(ph.memsz, 0x00000);
        assert_eq!(ph.flags, 0x4);
        assert_eq!(ph.align, 0x1);

        let ph = &elf.program_headers[1];
        assert_eq!(ph.p_type, 0x1);
        assert_eq!(ph.offset, 0x000000);
        assert_eq!(ph.vaddr, 0x00010000);
        assert_eq!(ph.paddr, 0x00010000);
        assert_eq!(ph.filesz, 0x03632);
        assert_eq!(ph.memsz, 0x03632);
        assert_eq!(ph.flags, 0x5);
        assert_eq!(ph.align, 0x1000);

        let ph = &elf.program_headers[2];
        assert_eq!(ph.p_type, 0x1);
        assert_eq!(ph.offset, 0x003634);
        assert_eq!(ph.vaddr, 0x00014634);
        assert_eq!(ph.paddr, 0x00014634);
        assert_eq!(ph.filesz, 0x00858);
        assert_eq!(ph.memsz, 0x008b0);
        assert_eq!(ph.flags, 0x6);
        assert_eq!(ph.align, 0x1000); 
    }

    #[test]
    fn test_parse_64_program_headers() {
        use super::elf64::*;
        let elf = ELF64::parse_path("tests/a_64.out").unwrap();
        assert_eq!(elf.program_headers.len(), 3);
        let ph = &elf.program_headers[0];
        assert_eq!(ph.p_type, 0x70000003);
        assert_eq!(ph.offset, 0x365b);
        assert_eq!(ph.vaddr, 0x00000000);
        assert_eq!(ph.paddr, 0x00000000);
        assert_eq!(ph.filesz, 0x003e);
        assert_eq!(ph.memsz, 0x0000);
        assert_eq!(ph.flags, 0x4);
        assert_eq!(ph.align, 0x1);
    
        let ph = &elf.program_headers[1];
        assert_eq!(ph.p_type, 0x1);
        assert_eq!(ph.offset, 0x000000);
        assert_eq!(ph.vaddr, 0x00010000);
        assert_eq!(ph.paddr, 0x00010000);
        assert_eq!(ph.filesz, 0x2672);
        assert_eq!(ph.memsz, 0x2672);
        assert_eq!(ph.flags, 0x5);
        assert_eq!(ph.align, 0x1000);
    
        let ph = &elf.program_headers[2];
        assert_eq!(ph.p_type, 0x1);
        assert_eq!(ph.offset, 0x2674);
        assert_eq!(ph.vaddr, 0x00013674);
        assert_eq!(ph.paddr, 0x00013674);
        assert_eq!(ph.filesz, 0x0fb4);
        assert_eq!(ph.memsz, 0x103c);
        assert_eq!(ph.flags, 0x6);
        assert_eq!(ph.align, 0x1000); 
    }

}
