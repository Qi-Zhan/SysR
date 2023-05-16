// -------------------- IOE: Input/Output Devices --------------------

use core::{arch::asm, fmt::Write};

#[derive(Debug)]
pub enum Device {
    SerialPort,
    KeyBoard,
    Vga,
    Audio,
    Disk,
    Fb,
    AudioSbuf,
    Timer,
}

pub struct SerialPort;
pub struct Timer;
pub struct KeyBoard;
pub struct Vga;

impl Timer {
    pub fn read() -> u64 {
        let mut clock_low32: u32 = 0;
        unsafe {
            asm!(
                "sw t0, 0(sp)",
                "li t0, 0xA0000048",
                "lw {}, 0(t0)",
                "lw t0, 0(sp)",
                out(reg) clock_low32,
            );
        }
        let mut clock_high32: u32 = 0;
        unsafe {
            asm!(
                "sw t0, 0(sp)",
                "li t0, 0xA000004C",
                "lw {}, 0(t0)",
                "lw t0, 0(sp)",
                out(reg) clock_high32,
            );
        }
        ((clock_high32 as u64) << 32) | (clock_low32 as u64)
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let buf = s.as_bytes();
        for c in buf {
            unsafe {
                (SERIAL_PORT as *mut u8).write_volatile(*c);
            }
        }
        Ok(())
    }
}

impl KeyBoard {
    pub fn read() -> Option<KBEvent> {
        let mut code: u32 = 0;
        unsafe {
            asm!(
                "sw t0, 0(sp)",
                "li t0, 0xa0000060",
                "lw {}, 0(t0)",
                "lw t0, 0(sp)",
                out(reg) code,
            );
        }
        match code {
            0 => None,
            _ => Some(KBEvent::from(code)),
        }
    }
}

impl Vga {
    pub fn write_all(buffer: &[u32]) {
        unsafe {
            for (index, item) in buffer.iter().enumerate() {
                // ((VGA_ADDR + (index * 4) as u64) as *mut u32).write_volatile(*item);
                ((VGA_ADDR + (index * 4) as u64) as *mut u32).write(*item);
            }
        }
    }

    pub fn write(buffer: &[u32], start: usize) {
        unsafe {
            for (index, item) in buffer.iter().enumerate() {
                ((VGA_ADDR + ((index + start) * 4) as u64) as *mut u32).write(*item);
            }
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for u32 {
    fn from(color: Color) -> Self {
        let r = color.r as u32;
        let g = color.g as u32;
        let b = color.b as u32;
        r | (g << 8) | (b << 16)
    }
}

impl From<u32> for Color {
    fn from(color: u32) -> Self {
        let b = (color >> 16) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let r = (color & 0xff) as u8;
        Color { r, g, b }
    }
}

pub const DEVICE_BASE: u64 = 0xa0000000;
pub const MMIO_BASE: u64 = 0xa0000000;
pub const SERIAL_PORT: u64 = DEVICE_BASE + 0x00003f8;
pub const KBD_ADDR: u64 = DEVICE_BASE + 0x0000060;
pub const RTC_ADDR: u64 = DEVICE_BASE + 0x0000048;
pub const VGACTL_ADDR: u64 = DEVICE_BASE + 0x0000100;
pub const AUDIO_ADDR: u64 = DEVICE_BASE + 0x0000200;
pub const DISK_ADDR: u64 = DEVICE_BASE + 0x0000300;
pub const VGA_ADDR: u64 = MMIO_BASE + 0x1000000;
pub const AUDIO_SBUF_ADDR: u64 = MMIO_BASE + 0x1200000;
pub const TIMER_ADDR: u64 = MMIO_BASE + 0x48;

#[derive(Debug, PartialEq)]
pub enum Key {
    Esc,
    Backspace,
    Tab,
    Enter,
    Space,
    Char(char),
    F(u8),
    Ctrl(char),
    Alt(char),
    CapsLock,
    NumLock,
    ScrollLock,
    Left,
    Right,
    Up,
    Down,
    PageUp,
    PageDown,
    Home,
    End,
    Insert,
    Delete,
    Unknown,
}

// implement Key to u32
impl From<Key> for u32 {
    fn from(key: Key) -> Self {
        match key {
            Key::Esc => 0x01,
            Key::F(n) => (0x3B + n).into(),
            Key::Char(c) => c as u32,
            Key::Ctrl(c) => (c as u32) & 0x1F,
            Key::Alt(c) => (c as u32) | 0x80,
            Key::CapsLock => 0x3A,
            Key::NumLock => 0x45,
            Key::ScrollLock => 0x46,
            Key::Left => 0x4B,
            Key::Right => 0x4D,
            Key::Up => 0x48,
            Key::Down => 0x50,
            Key::PageUp => 0x49,
            Key::PageDown => 0x51,
            Key::Home => 0x47,
            Key::End => 0x4F,
            Key::Insert => 0x52,
            Key::Delete => 0x53,
            Key::Backspace => 0x0E,
            Key::Tab => 0x0F,
            Key::Enter => 0x1C,
            Key::Space => 0x39,
            Key::Unknown => 0,
        }
    }
}

// u32 to Key
impl From<u32> for Key {
    fn from(value: u32) -> Self {
        match value {
            0x01 => Key::Esc,
            0x3A => Key::CapsLock,
            0x45 => Key::NumLock,
            0x46 => Key::ScrollLock,
            0x4B => Key::Left,
            0x4D => Key::Right,
            0x48 => Key::Up,
            0x50 => Key::Down,
            0x49 => Key::PageUp,
            0x51 => Key::PageDown,
            0x47 => Key::Home,
            0x4F => Key::End,
            0x52 => Key::Insert,
            0x53 => Key::Delete,
            0x0E => Key::Backspace,
            0x0F => Key::Tab,
            0x1C => Key::Enter,
            0x39 => Key::Space,
            0x3B..=0x44 => Key::F((value - 0x3B) as u8),
            0x80..=0x9F => Key::Alt((value - 0x80) as u8 as char),
            0x00..=0x1F => Key::Ctrl((value + 0x40) as u8 as char),
            _ => Key::Char(value as u8 as char),
        }
    }
}

impl From<KBEvent> for u32 {
    fn from(event: KBEvent) -> Self {
        match event {
            KBEvent::Press(key) => key.into(),
            KBEvent::Release(key) => {
                let code: u32 = key.into();
                code | 0x8000_0000
            }
        }
    }
}

impl From<u32> for KBEvent {
    fn from(value: u32) -> Self {
        if value & 0x8000_0000 == 0 {
            KBEvent::Press(Key::from(value))
        } else {
            KBEvent::Release(Key::from(value & 0x7FFF_FFFF))
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KBEvent {
    Press(Key),
    Release(Key),
}
