use crate::filesystem::{Finfo, FileSystem};

#[repr(C)]
#[derive(Debug)]
struct ElfHeader {
    magic:      [u8; 4],
    elf:        [u8; 12],
    elf_type:   [u8; 2],
    arch:       [u8; 2],
    version:    [u8; 4],
    entry:      [u8; 4],
    phoff:      [u8; 4],
    shoff:      [u8; 4],
    flags:      [u8; 4],
    ehsize:     [u8; 2],
    phentsize:  [u8; 2],
    phnum:      [u8; 2],
    shentsize:  [u8; 2],
    shnum:      [u8; 2],
    shstrndx:   [u8; 2],
}

#[repr(C)]
#[derive(Debug)]
struct ProgramHeader {
    p_type:     [u8; 4],
    offset:     [u8; 4],
    vaddr:      [u8; 4],
    paddr:      [u8; 4],
    filesz:     [u8; 4],
    memsz:      [u8; 4],
    flags:      [u8; 4],
    align:      [u8; 4],
}

pub fn load_fs(fs: &FileSystem) {
    for i in 0..fs.files.len() {
        let info = &fs.files[i];
        if info.name.len() == 0 {
            break;
        }
        todo!("load app to memory")
    }
}

/// Analyze elf file and load it into memory
pub fn load_file(info: &Finfo) -> u32 {
    let start = info.offset;
    let elf = unsafe { &*(start as *const u8 as *const ElfHeader) };
    assert_eq!(elf.magic, [0x7f, 0x45, 0x4c, 0x46]); // elf magic
    assert_eq!(elf.arch, [243, 0x00]); // riscv32

    // Load program into memory
    let mut p = start + u32::from_le_bytes(elf.phoff) as usize;
    for _ in 0..u16::from_le_bytes(elf.phnum) {
        let ph = unsafe { &*(p as *const u8 as *const ProgramHeader) };
        if u32::from_le_bytes(ph.p_type) == 1 { // loadable
            let offset= u32::from_le_bytes(ph.offset) as usize;
            let vaddr = u32::from_le_bytes(ph.vaddr) as usize;
            let filesz = u32::from_le_bytes(ph.filesz) as usize;
            let memsz = u32::from_le_bytes(ph.memsz) as usize;
            let mut p = start + offset;
            for index in 0..filesz {
                let byte = unsafe { *(p as *const u8) };
                unsafe { *((index + vaddr)as *mut u8) = byte };
                p += 1;
            }
            for index in filesz..memsz {
                unsafe { *((index + vaddr)as *mut u8) = 0 };
            }
        }
        p += u16::from_le_bytes(elf.phentsize) as usize;
    }

    let entry = u32::from_le_bytes(elf.entry);
    entry
}
