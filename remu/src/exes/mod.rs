pub mod elf;
mod elformat;
pub mod simplexe;

use crate::error::RError;
use crate::isas::ISA;
use std::fs::File;
use std::io::Read;

pub trait Exe: Sized {
    fn parse(input: &[u8]) -> Result<Self, RError>;
    fn parse_path(path: &str) -> Result<Self, RError> {
        let mut file = File::open(path).map_err(|e| RError::IOError(e.to_string()))?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)
            .map_err(|e| RError::IOError(e.to_string()))?;
        Self::parse(&bytes)
    }

    fn load_binary(&mut self, cpu: &mut impl ISA) -> Result<(), RError>;
}
