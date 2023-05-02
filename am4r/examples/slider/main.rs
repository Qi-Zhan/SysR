#![no_std]
#![no_main]

use am4r::klib::get_time;
use am4r::{io::*, println, print};

const WIDTH : usize = 400;
const HEIGHT: usize = 300;
  

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let p1:&[u8; WIDTH*HEIGHT*3] = include_bytes!("p1.bin");
    let p2:&[u8; WIDTH*HEIGHT*3] = include_bytes!("p2.bin");
    
    let mut picture1 = [0_u32; WIDTH * HEIGHT];
    let mut picture2 = [0_u32; WIDTH * HEIGHT];
    // turn p1 into picture1
    println!("init p1");
    for i in 0..WIDTH * HEIGHT {
        let r = p1[i * 3] as u32;
        let g = p1[i * 3 + 1] as u32;
        let b = p1[i * 3 + 2] as u32;
        picture1[i] = (r << 16) | (g << 8) | b;
    }
    // turn p2 into picture2
    println!("init p2");
    // for i in 0..WIDTH * HEIGHT {
    //     let r = p2[i * 3] as u32;
    //     let g = p2[i * 3 + 1] as u32;
    //     let b = p2[i * 3 + 2] as u32;
    //     picture2[i] = (r << 16) | (g << 8) | b;
    // }
    let mut last = get_time();
    let mut i= 0;
    println!("init done");
    loop {
        let now = get_time();
        if (now - last) > 5000 {
            if i == 0 {
                Vga::write(&picture1);
            } else {
                // Vga::write(&picture2);
            }
            i = (i + 1) % 2;
            last = now;
        }
    };
}
