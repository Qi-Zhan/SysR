use crate::info;
use crate::ioe::{serial::SerialPort, timer::Timer, IO};
use crate::isas::MemoryModel;
use crate::{add_device, settings::*};

pub struct Mem {
    mem: Vec<u8>,
    pub devices: Vec<Box<dyn IO>>,
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

impl Mem {
    pub fn new() -> Self {
        let mem = vec![0; 0x100000000];
        let mut devices: Vec<Box<dyn IO>> = Vec::new();
        // register devices
        add_device!(ENABLE_SERIAL, SerialPort, devices);
        add_device!(ENABLE_TIMER, Timer, devices);
        // only enable vga and keyboard when sdl feature is enabled
        #[cfg(feature = "sdl")]
        {
            use crate::ioe;
            add_device!(ENABLE_KBD, ioe::keyboard::Keyboard, devices);
            add_device!(ENABLE_VGA, ioe::vga::Screen, devices);
        }
        Mem { mem, devices }
    }

    pub fn update_devices(&mut self) {
        for device in self.devices.iter_mut() {
            device.update();
        }
    }
}

impl MemoryModel for Mem {
    fn load_mem(&mut self, index: u32, bytes: u8) -> Option<u32> {
        for device in self.devices.iter_mut() {
            if device.match_(index as u64) {
                return device.read(index as u64);
            }
        }
        let mut value: u32 = 0;
        for i in 0..bytes as usize {
            // little endian
            value += (self.mem[index as usize + i] as u32) << (i * 8);
        }
        Some(value)
    }

    fn store_mem(&mut self, index: u32, bytes: u8, value: u32) {
        for device in self.devices.iter_mut() {
            if device.match_(index as u64) {
                device.write(index as u64, value as u64);
                return;
            }
        }
        for i in 0..bytes as usize {
            self.mem[index as usize + i] = (value >> (i * 8)) as u8;
        }
    }
}
