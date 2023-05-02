/* Lib Function  */

use core::fmt::Write;



pub fn rand() -> u32 {
    static mut SEED: u32 = 0xdeadbeef;
    unsafe {
        SEED = SEED.wrapping_mul(0xdefaced).wrapping_add(0x1234567);
        SEED
    }
}

pub fn rand_range(min: u32, max: u32) -> u32 {
    rand() % (max - min) + min
}

/// actually do not need, just implement it in future string
pub fn atoi(s: &str) -> i32 {
    let mut res = 0;
    for c in s.chars() {
        res = res * 10 + (c as u8 - b'0') as i32;
    }
    res
}

/// get time ms
pub fn get_time() -> u64 {
    crate::io::Timer::read()
}

// pub fn getc() -> char {
//     let mut buf = [0u8; 1];
//     ioe_read(Device::SerialPort, &mut buf);
//     buf[0] as char
// }

pub fn puts(s: &str) {
    let mut serial = crate::io::SerialPort;
    serial.write_str(s).unwrap();
}
