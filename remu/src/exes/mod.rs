pub mod simplexe;
pub mod elf;
mod elformat;

use crate::error::RError;
use crate::isas::ISA;
use std::{fs::File, io::Read};

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
