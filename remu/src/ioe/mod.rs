#[cfg(feature="sdl")]
pub mod keyboard;
pub mod serial;
pub mod timer;
pub mod vga;

pub trait IO {
    fn match_(&self, addr: u64) -> bool;
    fn read(&mut self, addr: u64) -> Option<u32>;
    fn write(&mut self, addr: u64, value: u64);
    fn update(&mut self);
    fn name(&self) -> &str;
}

use rconfig::ios;

pub const DEVICE_BASE: u64 = ios::DEVICE_BASE;
pub const MMIO_BASE: u64 = ios::MMIO_BASE;
pub const SERIAL_PORT: u64 = ios::SERIAL_PORT;
pub const KBD_ADDR: u64 = ios::KBD_ADDR;
pub const VGACTL_ADDR: u64 = ios::VGACTL_ADDR;
pub const AUDIO_ADDR: u64 = ios::AUDIO_ADDR;
pub const DISK_ADDR: u64 = ios::DISK_ADDR;

pub const VGA_ADDR: u64 = ios::VGA_ADDR;
pub const AUDIO_SBUF_ADDR: u64 = ios::AUDIO_SBUF_ADDR;
pub const TIMER_ADDR: u64 = ios::TIMER_ADDR;
