//! AM Library functions

use crate::{io::IO, print};
use core::fmt::Write;

pub fn rand() -> u32 {
    static mut SEED: u32 = 0xdeadbeef;
    unsafe {
        SEED = SEED.wrapping_mul(0xdefaced).wrapping_add(0x1234567);
        SEED
    }
}

/// return [min, max)
pub fn rand_range(min: u32, max: u32) -> u32 {
    rand() % (max - min) + min
}

/// actually do not need
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

pub fn getc() -> char {
    crate::io::SerialPort::read()
}

pub fn getline(buf: &mut [char]) -> usize {
    let mut i = 0;
    while i < buf.len() {
        let c = getc();
        if c == '\n' {
            break;
        }
        buf[i] = c;
        i += 1;
    }
    i
}

pub fn print_chars(buf: &[char]) {
    for c in buf {
        if c == &'\0' {
            break;
        }
        print!("{}", c);
    }
}

pub fn puts(s: &str) {
    let mut serial = crate::io::SerialPort;
    serial.write_str(s).unwrap();
}

/// # Safety
///
/// `dest` and `src` must be valid pointers.
pub unsafe fn memcpy(dest: *mut u8, src: *const u8, n: usize) {
    let mut p = dest;
    for i in 0..n {
        *p = *src.add(i);
        p = p.offset(1);
    }
}

/// # Safety
///
/// `dest` must be a valid pointer.
pub unsafe fn memset(dest: *mut u8, c: u8, n: usize) {
    let mut p = dest;
    for _ in 0..n {
        *p = c;
        p = p.offset(1);
    }
}
