#![no_main]
#![no_std]

extern crate alloc;
use rapps::*;

#[no_mangle] pub extern "C" fn _start() -> ! {
    let mut vec = alloc::vec![1, 2, 3];
    for i in 0..100 {
        assert_eq!(vec.len(), i + 3);
        vec.push(i);
    }
    for i in 0..100 {
        println!("count {}", i)
    }
    exit(0);
}
