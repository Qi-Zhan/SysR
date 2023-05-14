use super::IO;
use sdl2::event::Event;

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

pub(crate) struct Keyboard {
    base: u64,
    events: Vec<KBEvent>,
}

impl Default for Keyboard {
    fn default() -> Self {
        Self::new(super::KBD_ADDR)
    }
}

impl Keyboard {
    fn new(base: u64) -> Self {
        Self {
            base,
            events: Vec::new(),
        }
    }
}

impl IO for Keyboard {
    fn match_(&self, addr: u64) -> bool {
        addr >= self.base && addr < self.base + 8
    }

    fn name(&self) -> &str {
        "keyboard"
    }

    fn read(&mut self, addr: u64) -> Option<u32> {
        if addr == self.base {
            if let Some(event) = self.events.pop() {
                return Some(event.into());
            } else {
                return Some(0);
            }
        }
        None
    }

    fn write(&mut self, addr: u64, value: u64) {
        if addr == self.base || addr == 0 {
            let event: KBEvent = (value as u32).into();
            self.events.push(event);
        }
    }

    fn update(&mut self) {}
}

fn sdlcode2u32(sdlcode: sdl2::keyboard::Keycode) -> Key {
    use sdl2::keyboard::Keycode::*;
    match sdlcode {
        Escape => Key::Esc,
        F1 => Key::F(1),
        F2 => Key::F(2),
        F3 => Key::F(3),
        F4 => Key::F(4),
        Up => Key::Up,
        Down => Key::Down,
        Left => Key::Left,
        Right => Key::Right,
        A => Key::Char('a'),
        B => Key::Char('b'),
        C => Key::Char('c'),
        D => Key::Char('d'),
        E => Key::Char('e'),
        F => Key::Char('f'),
        G => Key::Char('g'),
        H => Key::Char('h'),
        I => Key::Char('i'),
        J => Key::Char('j'),
        K => Key::Char('k'),
        L => Key::Char('l'),
        M => Key::Char('m'),
        N => Key::Char('n'),
        O => Key::Char('o'),
        P => Key::Char('p'),
        Q => Key::Char('q'),
        R => Key::Char('r'),
        S => Key::Char('s'),
        T => Key::Char('t'),
        U => Key::Char('u'),
        V => Key::Char('v'),
        W => Key::Char('w'),
        X => Key::Char('x'),
        Y => Key::Char('y'),
        Z => Key::Char('z'),
        Num1 => Key::Char('1'),
        Num2 => Key::Char('2'),
        Num3 => Key::Char('3'),
        Num4 => Key::Char('4'),
        Num5 => Key::Char('5'),
        Num6 => Key::Char('6'),
        Num7 => Key::Char('7'),
        Num8 => Key::Char('8'),
        Num9 => Key::Char('9'),
        Num0 => Key::Char('0'),
        Return => Key::Enter,
        Backspace => Key::Backspace,
        Tab => Key::Tab,
        Space => Key::Space,
        LCtrl => Key::Ctrl('l'),
        // LShift => Key::Shift('l'),
        _ => todo!("sdlcode2u32: {:?}", sdlcode),
    }
}

impl From<Event> for KBEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::KeyDown { keycode, .. } => {
                if let Some(keycode) = keycode {
                    let scancode = sdlcode2u32(keycode);
                    KBEvent::Press(scancode)
                } else {
                    unreachable!()
                }
            }
            Event::KeyUp { keycode, .. } => {
                if let Some(keycode) = keycode {
                    let scancode = sdlcode2u32(keycode);
                    KBEvent::Release(scancode)
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use sdl2::event::Event;

    use crate::ioe::IO;
    use crate::ioe::keyboard::{KBEvent, Key};

    use super::super::KBD_ADDR;
    use super::Keyboard;
    #[test]
    fn test_kbd() {
        let mut kbd = Keyboard::default();
        kbd.write(KBD_ADDR, 0x1C);
        assert_eq!(kbd.read(KBD_ADDR), Some(0x1C));
        assert_eq!(kbd.read(KBD_ADDR), Some(0));

        let event: Event = Event::KeyDown {
            keycode: Some(sdl2::keyboard::Keycode::A),
            timestamp: 0,
            window_id: 0,
            scancode: None,
            keymod: sdl2::keyboard::Mod::empty(),
            repeat: false,
        };
        let kbevent: KBEvent = event.into();
        assert_eq!(kbevent, KBEvent::Press(Key::Char('a')));
        let code: u32 = kbevent.into();
        assert_eq!(code, 'a' as u32);
        let kbevent: KBEvent = code.into();
        assert_eq!(kbevent, KBEvent::Press(Key::Char('a')));
    }
}
