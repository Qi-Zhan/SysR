use super::IO;

// 400x300x32, every pixel is 4 bytes by red, green, blue, alpha
pub struct Screen {
    address: u64,
    width: usize,
    height: usize,
    buffer: Vec<u32>,
    // canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Default for Screen {
    fn default() -> Self {
        Self::new(super::VGA_ADDR, 400, 300)
    }
}

impl Screen {
    fn new(address: u64, width: u32, height: u32) -> Self {
        Screen {
            address,
            width: width as usize,
            height: height as usize,
            buffer: vec![0; width as usize * height as usize],
            // canvas,
        }
    }

    fn size(&self) -> u64 {
        self.width as u64 * self.height as u64
    }
}

impl IO for Screen {
    fn match_(&self, addr: u64) -> bool {
        addr >= self.address && addr < self.address + self.size() * 4
    }

    fn name(&self) -> &'static str {
        "vga"
    }

    fn read(&mut self, addr: u64) -> Option<u32> {
        let offset = if addr < self.address {
            addr 
        } else {
            addr - self.address - self.size() 
        };
        let offset = offset as usize;
        Some(self.buffer[offset])
    }

    fn write(&mut self, addr: u64, value: u64) {
        let offset = (addr - self.address) / 4;
        let offset = offset as usize;
        let value = value as u32;
        self.buffer[offset] = value;
    }

}
