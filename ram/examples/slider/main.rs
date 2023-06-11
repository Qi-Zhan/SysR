#![no_std]
#![no_main]

use ram::{io::*, println, tm::halt};

const WIDTH : usize = 400;
const HEIGHT: usize = 300;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let p1:&[u8; WIDTH*HEIGHT*3] = include_bytes!("p1.bin");
    let p2:&[u8; WIDTH*HEIGHT*3] = include_bytes!("p2.bin");
    
    let mut picture1 = [0_u32; WIDTH * HEIGHT];
    let mut picture2 = [0_u32; WIDTH * HEIGHT];
    println!("init p1");
    for i in 0..WIDTH * HEIGHT {
        let r = p1[i * 3] as u32;
        let g = p1[i * 3 + 1] as u32;
        let b = p1[i * 3 + 2] as u32;
        picture1[i] = (r << 16) | (g << 8) | b;
    }
    println!("init p2");
    for i in 0..WIDTH * HEIGHT {
        let r = p2[i * 3] as u32;
        let g = p2[i * 3 + 1] as u32;
        let b = p2[i * 3 + 2] as u32;
        picture2[i] = (r << 16) | (g << 8) | b;
    }
    println!("init done");
    println!("Type '<-' to show p1, '->' to show p2, 'Esc' to exit");
    loop {
        if let Some(c) = KeyBoard::read() {
            match c {
                KBEvent::Press(Key::Esc) => halt(0),
                KBEvent::Press(Key::Left) => Vga::write_all(&picture1),
                KBEvent::Press(Key::Right) => Vga::write_all(&picture2),
                _ => {}
            }
        }
    };
}
