#![no_std]
#![no_main]

mod font;

use ram::klib::get_time;
use ram::klib::rand_range;
use ram::tm::halt;
use ram::{io::*, println};

use font::FONT;

const FPS: u64 = 30;
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;
const NCHARS: usize = 6;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

const COL_WHITE: u32 = 0xeeeeee;
const COL_PURPLE: u32 = 0x2a0a29;

#[derive(Clone, Copy, Debug)]
struct Character {
    pub ch: u32,
    pub x: usize,
    pub y: usize,
    pub v: usize,
}

impl Character {
    fn new() -> Self {
        Character {
            ch: rand_range(1, 27),
            x: rand_range(0, (WIDTH - CHAR_WIDTH) as u32) as usize,
            y: 0,
            v: 1,
        }
    }

    fn draw(&self, pixels: &mut [u32]) {
        let base = (self.ch - 1) as usize * CHAR_HEIGHT;
        for i in 0..CHAR_HEIGHT {
            for j in 0..CHAR_WIDTH {
                if FONT[base + i] & (1 << (CHAR_WIDTH - 1 - j)) != 0 {
                    pixels[(self.y + i) * WIDTH + self.x + j] = COL_WHITE;
                }
            }
        }
    }
}

struct Game {
    /// display buffer
    pub pixels: [u32; WIDTH * HEIGHT],
    // chars in screen
    pub chars: [Character; NCHARS],
}

impl Game {
    fn new() -> Self {
        let mut chars = [Character::new(); NCHARS];
        for i in 0..NCHARS {
            chars[i] = Character::new();
        }
        Game {
            pixels: [COL_PURPLE; WIDTH * HEIGHT],
            chars,
        }
    }

    fn char_to_u32(c: char) -> u32 {
        if c >= 'a' && c <= 'z' {
            return (c as u32) - ('a' as u32) + 1;
        }
        if c >= 'A' && c <= 'Z' {
            return (c as u32) - ('A' as u32) + 1;
        }
        0
    }

    /// user type a char
    fn hit(&mut self, c: char) {
        for i in 0..NCHARS {
            if self.chars[i].ch == Game::char_to_u32(c) {
                println!("hit: {}", c);
                self.chars[i].ch = 0;
                return;
            }
        }
    }

    /// update pixels by chars
    fn update(&mut self) {
        for i in 0..NCHARS {
            if self.chars[i].ch != 0 {
                self.chars[i].y += self.chars[i].v;
            }
        }
    }

    fn render(&mut self) {
        // clean
        for i in 0..WIDTH * HEIGHT {
            self.pixels[i] = COL_PURPLE;
        }
        for i in 0..NCHARS {
            if self.chars[i].ch != 0 && self.chars[i].y < HEIGHT {
                self.chars[i].draw(&mut self.pixels);
            }
        }
        Vga::write_all(&self.pixels);
    }

    fn over(&self) -> bool {
        for i in 0..NCHARS {
            if self.chars[i].y >= HEIGHT - CHAR_HEIGHT {
                return true;
            }
        }
        false
    }

    fn win(&self) -> bool {
        for i in 0..NCHARS {
            if self.chars[i].ch != 0 {
                return false;
            }
        }
        true
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello, this a simple typing game!");
    let mut game = Game::new();
    Vga::write_all(&game.pixels);
    let mut last = get_time();
    loop {
        if game.win() {
            println!("\x1b[1;32mYou Win!\x1b[0m");
            halt(0);
        }
        if game.over() {
            println!("\x1b[1;31mGame Over!\x1b[0m");
            halt(0);
        }

        if let Some(event) = KeyBoard::read() {
            match event {
                KBEvent::Press(Key::Char(c)) => {
                    game.hit(c);
                }
                _ => {}
            }
        }

        let now = get_time();
        if (now - last) > (1000 / FPS) {
            game.update();
            game.render();
            last = now;
        }
    }
}
