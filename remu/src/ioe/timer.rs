use super::IO;

#[derive(Debug)]
pub(crate) struct Timer {
    base: u64,
}

impl Timer {
    pub(crate) fn new(base: u64) -> Self {
        Self { base }
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new(super::TIMER_ADDR)
    }
}

impl IO for Timer {
    fn match_(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.base + 8
    }

    fn name(&self) -> &str {
        "timer"
    }

    fn read(&mut self, addr: u64) -> Option<u32> {
        // return current time
        let time = std::time::UNIX_EPOCH.elapsed().unwrap().as_millis() as u64;
        if addr == self.base {
            // low 32 bits
            Some(time as u32)
        } else {
            // high 32 bits
            Some((time >> 32) as u32)
        }
    }

    fn write(&mut self, _addr: u64, _value: u64) {}

    fn update(&mut self) {}
}
