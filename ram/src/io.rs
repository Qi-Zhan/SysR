//! -------------------- IOE: Input/Output Devices --------------------
//! - SerialPort
//! - Timer
//! - KeyBoard
//! - Vga

use core::fmt::Write;
use rconfig::ios::*;

pub trait IO {
    type Input;
    type Output;
    fn read() -> Self::Input;
    // it seems that we do not need write in IO trait
}

pub struct SerialPort;
pub struct Timer;
pub struct KeyBoard;
pub struct Vga;

impl IO for Timer {
    type Input = u64;
    type Output = ();
    fn read() -> Self::Input {
        unsafe {
            let clock_low32 = (TIMER_ADDR as *mut u32).read_volatile();
            let clock_high32 = ((TIMER_ADDR + 4) as *mut u32).read_volatile();
            ((clock_high32 as u64) << 32) | (clock_low32 as u64)
        }
    }
}

impl IO for KeyBoard {
    type Input = Option<KBEvent>;
    type Output = ();
    fn read() -> Self::Input {
        unsafe {
            let code = (KBD_ADDR as *mut u32).read_volatile();
            match code {
                0 => None,
                _ => Some(KBEvent::from(code)),
            }
        }
    }
}

impl IO for SerialPort {
    type Input = u8;
    type Output = ();
    fn read() -> Self::Input {
        unsafe { (SERIAL_PORT as *mut u8).read_volatile() }
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

impl Vga {
    pub fn write_all(buffer: &[u32]) {
        unsafe {
            for (index, item) in buffer.iter().enumerate() {
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
