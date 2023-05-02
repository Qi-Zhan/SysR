#![no_std]
#![no_main]

mod font;


use am4r::klib::get_time;
use am4r::{io::*, println, print};


use font::FONT;

const FPS: u64 = 30;
const CPS: usize = 5;
const CHAR_WIDTH: usize = 8;
const CHAR_HEIGHT: usize = 16;
const NCHARS: usize = 128;
const WIDTH : usize = 400;
const HEIGHT: usize = 300;

enum Color {
    White   = 0xeeeeee,
    Reg     = 0xff0033,
    Green   = 0x00cc33,
    Purple  = 0x2a0a29,
}

#[derive(Clone, Copy)]
struct Character {
    ch: char,
    x: usize,
    y: usize,
    v: usize,
    t: usize,
}

fn update_screen(pixels: &[u32]) {
    Vga::write(pixels);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut last = get_time();
    let mut step = 0;
    // let mut chars: [Character; NCHARS] = [Character {ch: 'a', x: 0, y: 0, v: 0, t: 0}; NCHARS];
    let mut pixels = [0xff_u32; WIDTH * HEIGHT];
    loop {
        println!("Hello, world!");

        // let now = get_time();
        // if (now - last) > (1000 / FPS) {
            update_screen(&pixels);
            step += 1;
            println!("step: {}", step);
            // last = now;
        // }
    };
}