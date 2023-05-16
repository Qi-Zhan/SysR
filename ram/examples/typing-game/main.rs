#![no_std]
#![no_main]

mod font;


use ram::klib::get_time;
use ram::tm::halt;
use ram::{io::*, println, print};
use ram::klib::rand_range;

use font::FONT;

const FPS:          u64 = 30;
const CPS:          usize = 5;
const CHAR_WIDTH:   usize = 8;
const CHAR_HEIGHT:  usize = 16;
const NCHARS:       usize = 128;

const WIDTH : usize = 400;
const HEIGHT: usize = 300;

const COL_WHITE:    u32 = 0xeeeeee;
const COL_RED:      u32 = 0xff0033;
const COL_GREEN:    u32 = 0x00cc33;
const COL_PURPLE:   u32 = 0x2a0a29;

const WHITE:    usize = 0;
const RED:      usize = 1;
const GREEN:    usize = 2;
const PURPLE:   usize = 3;

// enum Color {
//     White   = 0xeeeeee,
//     Reg     = 0xff0033,
//     Green   = 0x00cc33,
//     Purple  = 0x2a0a29,
// }

#[derive(Clone, Copy)]
struct Character {
    ch: u32,
    x: u32,
    y: usize,
    v: usize,
    t: usize,
}

impl Character {
    fn new() -> Self {
        Character {
            ch: rand_range(0, 26),
            x: rand_range(0, WIDTH - CHAR_WIDTH),
            y: 0,
            v: 1,
            t: 0,
        }
    }

    fn draw(&self, game: &Game, pixels: &mut [u32]) {
        let mut ch = self.ch;
        if self.t > 0 {
            ch = 26;
        }
        for j in 0..CHAR_HEIGHT {
            for k in 0..CHAR_WIDTH {
                let idx = self.y * WIDTH + self.x + j * WIDTH + k;
                if idx < WIDTH * HEIGHT {
                    pixels[idx] = game.texture[ch as usize][j * CHAR_WIDTH + k];
                }
            }
        }
    }
}

struct Game {
    /// The characters to be displayed
    texture:    [[u32; CHAR_WIDTH * CHAR_HEIGHT]; 26],
    /// The characters to be displayed
    blank:      [u32; CHAR_WIDTH * CHAR_HEIGHT],  
    /// display buffer
    pub pixels:     [u32; WIDTH * HEIGHT],
}

impl Game {
    fn new() -> Self {
        let mut texture = [[0_u32; CHAR_WIDTH * CHAR_HEIGHT]; 26];
        for c in 0..26 {
            for j in 0..CHAR_HEIGHT {
                for k in 0..CHAR_WIDTH {
                    texture[c][j * CHAR_WIDTH + k] = FONT[c*CHAR_HEIGHT*CHAR_WIDTH + j*CHAR_WIDTH + k] as u32;
                }
            }
        }
        let blank = [COL_PURPLE; CHAR_WIDTH * CHAR_HEIGHT];
        Game {
            texture,
            blank,
            pixels: [COL_PURPLE; WIDTH * HEIGHT],
        }
    }
}

fn update_screen(pixels: &[u32]) {
    Vga::write_all(pixels);
}

fn video_init(pixels: &[u32]) {
    let screen_w = WIDTH;
    let screen_h = HEIGHT;

}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut game = Game::new();
    video_init(&mut game.pixels);
    let mut last = get_time();
    let mut step = 0;
    // let mut chars: [Character; NCHARS] = [Character {ch: 'a', x: 0, y: 0, v: 0, t: 0}; NCHARS];
    println!("Type 'Esc' to exit");
    loop {
        if let Some(c) = KeyBoard::read() {
            match c {
                KBEvent::Press(Key::Esc) => halt(0),
                _ => {}
            }
        }
        let now = get_time();
        if (now - last) > (1000 / FPS) {
            // update_screen(&pixels);
            step += 1;
            println!("step: {}", step);
            last = now;
        }
    };
}