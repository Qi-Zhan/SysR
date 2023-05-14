use crate::isas::MemoryModel;
use crate::ioe::{keyboard::Keyboard, serial::SerialPort, timer::Timer, vga::Screen, IO};
use crate::settings::*;

// #[derive(Debug)]
pub struct Mem {
    // huge memory 
    mem: Vec<u8>,
    pub devices: Vec<Box<dyn IO>>,
}

impl Default for Mem {
    fn default() -> Self {
        Self::new()
    }
}

use crate::info;

impl Mem {

    pub fn new() -> Self {
        let mem = vec![0; 0x100000000];
        let mut devices: Vec<Box<dyn IO>> = Vec::new();
        // register devices
        if ENABLE_SERIAL {
            devices.push(Box::<SerialPort>::default());
            info!("serial port enabled")
        }
        if ENABLE_KBD {
            devices.push(Box::<Keyboard>::default());
            info!("keyboard enabled")
        }
        if ENABLE_VGA {
            devices.push(Box::<Screen>::default());
            info!("vga enabled")
        }
        if ENABLE_TIMER {
            devices.push(Box::<Timer>::default());
            info!("timer enabled")
        }

        Mem { 
            mem,
            devices,
        }
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
        for i in 0..bytes as usize { // little endian
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
