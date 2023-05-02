#![allow(dead_code)]
#![allow(clippy::upper_case_acronyms)]
#![allow(non_camel_case_types)]
#![allow(clippy::enum_clike_unportable_variant)]

use std::fmt::{Display, Formatter, Result};

pub type Elf32Addr = u32;
pub type Elf32Off = u32;
pub type Elf32Section = u16;
pub type Elf32Versym = u16;
pub type EflByte = u8;
pub type Elf32Half = u16;
pub type Elf32Word = u32;
pub type Elf32Sword = i32;
pub type Elf32Xword = u64;
pub type Elf32Sxword = i64;

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Section = u16;
pub type Elf64Versym = u16;
pub type Elf64Half = u16;
pub type Elf64Word = u32;
pub type Elf64Sword = i32;
pub type Elf64Xword = u64;
pub type Elf64Sxword = i64;

pub const EI_MAGO: u8 = 0x7f; // magic number
pub const EI_MAG1: u8 = 0x45; // 'E'
pub const EI_MAG2: u8 = 0x4c; // 'L'
pub const EI_MAG3: u8 = 0x46; // 'F'

macro_rules! define_enum {
    ($name:ident, $type:ty, $($variant:ident = $value:expr),*) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $name {
            $($variant = $value),*
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                match self {
                    $($name::$variant => write!(f, "{}", stringify!($variant))),*
                }
            }
        }

        impl $name {
            pub fn from_u8(value: $type) -> Option<Self> {
                match value {
                    $($value => Some(Self::$variant)),*,
                    _ => None
                }
            }
        }

        impl $name {
            pub fn from_u16(value: $type) -> Option<Self> {
                match value {
                    $($value => Some(Self::$variant)),*,
                    _ => None
                }
            }
        }

        impl $name {
            pub fn from_u32(value: $type) -> Option<Self> {
                match value {
                    $($value => Some(Self::$variant)),*,
                    _ => None
                }
            }
        }

        impl From<$name> for $type {
            fn from(value: $name) -> Self {
                value as $type
            }
        }
    }
}

define_enum!(EiData, u8, None = 0, Data2LSB = 1, Data2MSB = 2);

define_enum!(EiVersion, u8, None = 0, Current = 1);

define_enum!(
    EiOsAbi,
    u8,
    SystemV = 0,
    Hpux = 1,
    NetBSD = 2,
    Linux = 3,
    GNUHurd = 4,
    Solaris = 6,
    Aix = 7,
    Irix = 8,
    FreeBSD = 9,
    Tru64 = 10,
    NovellModesto = 11,
    OpenBSD = 12,
    OpenVMS = 13,
    NonStopKernel = 14,
    Aros = 15,
    FenixOS = 16,
    CloudABI = 17,
    StratusTechnologiesOpenVOS = 18
);


define_enum!(
    EType,
    u16,
    None = 0,
    Relocatable = 1,
    Executable = 2,
    Shared = 3,
    Core = 4,
    LoProc = 0xff00,
    HiProc = 0xffff
);

define_enum!(
    EMachine,
    u16,
    None = 0,
    M32 = 1,
    Sparc = 2,
    Intel386 = 3,
    Motorola68000 = 4,
    Motorola88000 = 5,
    Reserverd = 6,
    Intel860 = 7,
    Mips = 8,
    S370 = 9,
    MipsRS3000LE = 10,
    Reserved11 = 11,
    Reserved12 = 12,
    RiscV = 243
);

define_enum! {
    EiClass,
    u8,
    None = 0,
    Elf32 = 1,
    Elf64 = 2
}

define_enum!(
    PType,
    u32,
    Null = 0,
    Load = 1,
    Dynamic = 2,
    Interp = 3,
    Note = 4,
    Shlib = 5,
    Phdr = 6,
    Tls = 7,
    Attributes = 0x70000003,
    LoOS = 0x60000000,
    HiOS = 0x6fffffff,
    LoProc = 0x70000000,
    HiProc = 0x7fffffff
);

define_enum!(
    PFlags,
    u32,
    None = 0,
    __X = 0x1,
    _W_ = 0x2,
    _WX = 0x3,
    R__ = 0x4,
    R_X = 0x5,
    RW_ = 0x6,
    RWX = 0x7
);

define_enum!(
    SType,
    u32,
    Null = 0,
    Progbits = 1,
    Symtab = 2,
    Strtab = 3,
    Rela = 4,
    Hash = 5,
    Dynamic = 6,
    Note = 7,
    Nobits = 8,
    Rel = 9,
    Shlib = 10,
    Dynsym = 11,
    Loproc = 0x70000000,
    Hiproc = 0x7fffffff,
    Louser = 0x80000000,
    Hiuser = 0xffffffff,
    InitArray = 14,
    FiniArray = 15,
    Attributes = 0x70000003
);

// TODO: it is incorrect since it is not a bit mask
define_enum!(
    SFlags,
    u64,
    Null = 0,
    Write = 0x1,
    Alloc = 0x2,
    ExecInstr = 0x4,
    WA = 0x3,
    WAX = 0x7,
    WAEX = 0xf,
    AE = 0x6,
    Merge = 0x10,
    Strings = 0x20,
    MS = 0x30,
    InfoLink = 0x40,
    LinkOrder = 0x80,
    OsNonconforming = 0x100,
    Group = 0x200,
    Tls = 0x400,
    Compressed = 0x800,
    MaskOS = 0x0ff00000,
    MaskProc = 0xf0000000,
    Ordered = 0x4000000,
    Exclude = 0x8000000
);

define_enum!(
    STType,
    u8,
    None = 0,
    Object = 1,
    Func = 2,
    Section = 3,
    File = 4,
    Common = 5,
    Tls = 6,
    LoOS = 10,
    HiOS = 12,
    LoProc = 13,
    HiProc = 15
);

define_enum!(
    STBind,
    u8,
    Local = 0,
    Global = 1,
    Weak = 2,
    LoOS = 10,
    HiOS = 12,
    LoProc = 13,
    HiProc = 15
);

define_enum!(
    STVis,
    u8,
    Default = 0,
    Internal = 1,
    Hidden = 2,
    Protected = 3
);
