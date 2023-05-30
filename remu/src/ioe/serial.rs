use super::IO;
use console::Term;
use std::io::Write;

#[derive(Debug)]
pub(crate) struct SerialPort {
    base: u64,
    irq: u8,
    term: Term,
}

impl SerialPort {
    pub(crate) fn new(base: u64, irq: u8) -> Self {
        Self {
            base,
            irq,
            term: Term::stdout(),
        }
    }
}

impl Default for SerialPort {
    fn default() -> Self {
        Self::new(super::SERIAL_PORT, 4)
    }
}

impl IO for SerialPort {
    fn match_(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.base + self.irq as u64
    }

    fn name(&self) -> &str {
        "serial"
    }

    fn read(&mut self, _addr: u64) -> Option<u32> {
        self.term.read_char().ok().map(|c| c as u32)
    }

    fn write(&mut self, _addr: u64, value: u64) {
        print!("{}", value as u8 as char);
        // TODO: maybe influence performance
        std::io::stdout().flush().unwrap();
    }

    fn update(&mut self) {}
}
