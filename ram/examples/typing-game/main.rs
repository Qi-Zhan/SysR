#![no_std]
#![no_main]

mod font;


use ram::klib::get_time;
use ram::tm::halt;
use ram::{io::*, println, print};


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
    ch: char,
    x: usize,
    y: usize,
    v: usize,
    t: usize,
}

struct Game {
    /// The characters to be displayed
    texture:    [[u32; CHAR_WIDTH * CHAR_HEIGHT]; 26],
    /// The characters to be displayed
    blank:      [u32; CHAR_WIDTH * CHAR_HEIGHT],  
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
        }
    }
}

fn update_screen(pixels: &[u32]) {
    Vga::write(pixels);
}

fn video_init(pixels: &[u32]) {
    let screen_w = WIDTH;
    let screen_h = HEIGHT;

}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut pixels = [0xff_u32; WIDTH * HEIGHT];
    video_init(&mut pixels);
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
            update_screen(&pixels);
            step += 1;
            println!("step: {}", step);
            last = now;
        }
    };
}