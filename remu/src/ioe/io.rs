/// maybe use async to implement IO
pub trait IO  {
    fn match_(&self, addr: u64) -> bool;
    fn read(&mut self, addr: u64) -> Option<u32>;
    fn write(&mut self, addr: u64, value: u64);
    fn update(&mut self,);
    fn name(&self) -> &str;
}
    
pub const DEVICE_BASE: u64 = 0xa0000000;
pub const MMIO_BASE: u64 = 0xa0000000;
pub const SERIAL_PORT: u64 = DEVICE_BASE + 0x00003f8;
pub const KBD_ADDR: u64 = DEVICE_BASE + 0x0000060;
pub const VGACTL_ADDR: u64 = DEVICE_BASE + 0x0000100;
pub const AUDIO_ADDR: u64 = DEVICE_BASE + 0x0000200;
pub const DISK_ADDR: u64 = DEVICE_BASE + 0x0000300;

pub const VGA_ADDR: u64 = MMIO_BASE + 0x1000000;
pub const AUDIO_SBUF_ADDR: u64 = MMIO_BASE + 0x1200000;
pub const TIMER_ADDR: u64 = MMIO_BASE + 0x48;
